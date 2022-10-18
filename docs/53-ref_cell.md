# RefCell<T>和内部可变性

## 一、内部可变性

内部可变性是Rust的设计模式之一，它允许你在只持有不可变引用的前提下进行修改。数据结构中使用了unsafe代码绕过rust正常的可变性和借用规则。

## 二、RefCell<T>

### 2.1 Rc<T>和RefCell<T>

与`Rc<T>`不同，`RefCell<T>`类型代表了其持有数据的唯一所有权。

我们先回忆一下借用规则

- 在任何给定的时间里，你要么只能拥有一个可变引用，要么只能拥有任意数量的不可变引用
- 引用总是有效的

`RefCell<T>`与`Box<T>`的区别如下

|                   Box<T>                   |                      RefCell<T>                       |
| ------------------------------------------ | ----------------------------------------------------- |
| 编译阶段强制代码遵循借用规则，否则出现错误 | 只会在运行时检查借用规则，没有满足借用规则则触发panic |

### 2.2 借用规则在不同阶段进行检查比较

在编译阶段检查，尽早暴露问题，没有运行时开销，对大多数场景是最佳选择，也是rust的默认行为。在运行时阶段检查，问题暴露延后，甚至影响到生产环境，因借用计数器产生些许性能损失，但实现了某些特定的内存安全场景（不可变环境中修改自身数据）。

针对编译器无法理解的代码，rust直接拒绝编译通过，避免产生问题。但如果开发者能够保证借用规则能满足，那么这个时候`RefCell<T>`就有它的勇武之处了。

### 2.3 RefCell<T>

与`Rc<T>`相似，只能用于但线程场景。

选择`Box<T>`、`Rc<T>`、`RefCell<T>`的依据

- Box<T>: 同一个数据拥有一个所有者，可变、不可变借用在编译时检查
- Rc<T>: 同一个数据拥有多个所有者，不可变引用在编译时检查
- RefCell<T>: 同一数据拥有过一个所有者，可变、不可变借用在运行时检查

其中，即便`RefCell<T>`本身不可变，但仍能修改其中存储的值

### 2.4 内部可变性，可变的借用一个不可变的值

借用规则有一个这样的推论，我们无法借用一个不可变的值。我们先看一个示例

```rust
fn main() {
    let x = 5;
    let y = &mut x;
}
```

上面的代码中，`x`是不可变的，让`y`借用一个`x`的可变引用，这样是不运行的，编译会发生报错。然而在某些情况下，我们需要这样一个值，它对外部是不可变的，但它同时能够在方法内部修改自身的值。除了这个数据的本身方法，其余的代码都不能修改这个值。

使用`RefCell<T>`就是获得这种内部可变性的一个方法，不过`RefCell<T>`并没有完全完全绕开借用规则，我们虽然使用内部可变性通过了编译阶段的借用检查，但借用检查仅仅是延后到了运行阶段而已，如果违反了借用规则，那么程序在运行时会出现panic。

下面我们来看一个示例

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

我们要测试实现了`Messenger`这个trait的值和一个`max`值来创建`LimitTracker`的时候，传入不同的value就能够触发`Messenger`发送不同的消息。而我们使用"mock object"模拟对象来创建`LimitTracker`实例之后，我门便可以通过调用模拟对象的`set_value`方法来检查模拟对象中是否存储了我们希望看到的消息。按照这个思路，我门实现一下测试代码

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messenger: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messenger: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&mut self, message: &str) {
            self.sent_messenger.push(String::from(message))
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // 测试超过70%的情况
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messenger.len(), 1);
    }
}
```

不过，编译上面的代码的时候，会出现错误，错误在于`Messenger` trait的`send`方法生命的时候`self`参数是不可变，但实现的方法，是可变的，因为我们需要修改里面的值。

这是时候这就是内部可变性可以使用的场景，这时候使用`RefCell`，最终修改的测试代码如下

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messenger: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messenger: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messenger.borrow_mut().push(String::from(message))
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // 测试超过70%的情况
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messenger.borrow().len(), 1);
    }
}
```

此时代码测试通过了。我们将`Vec`换成`RefCell<Vec>`之后，然后调用对应数据的`borrow_mut`方法，它可以获得内部值的可变引用，最后调用了`borrow`方法，获得了内部值的不可变引用。

### 2.5 使用RefCell<T>在运行时记录借用信息

RefCell提供了两个安全接口

- borrow方法：返回只能指针`Ref<T>`，它实现了`Deref`
- borrow_mut方法：返回只能指针`RefMut<T>`，它实现了`Deref`

`RefCell<T>`会记录当前存在多少个活跃的`Ref<T>`和`RefMut<T>`只能指针。每次调用`borrow`，不可变借用计数加1，任何一个`Ref<T>`的值离开作用域被释放时，不可变借用计数减1；每次调用`borrow_mut`，可变借用计数加1，任何一个`RefMut<T>`的值离开作用域被释放时，可变借用计数减1。

rust通过上述技术来维护借用检查规则，任何一个给定时间里，值允许拥有多个不可变借用或一个可变借用。而当我们违背这个规则时，`RefCell<T>`就会在运行时来触发panic。

### 2.6 将Rc<T>和RefCell<T>结合使用来实现多重所有权可变数据

将`RefCell<T>`和`Rc<T>`结合使用是一种很常用的做法，`Rec<T>`允许多个所有者持有同一个数据，但只能提供对数据的不可变访问，如果我们在`Rc<T>`里存储`RefCell<T>`，那么就可以定义出拥有多个所有者而且能够进行修改的值。如下示例代码

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));
    // 通过Rc::clone共享value的值给a
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // b、c共享a列表
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // 使用*把Rc<T>解引用为RefCell<T>，通过borrow_mut修改value的值，加10
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

最终运行结果如下

```bash
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

value的值变了，符合预期。

## 三、其他可实现内部可变性的类型

rust还提供了其他可实现内部可变性的类型，如下示例

- `Cell<T>`: 通过复制来访问数据
- `Mutex<T>`: 用于实现跨线程情况下的内部可变性模式
