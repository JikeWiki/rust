# 闭包

## 一、什么是闭包

### 1.1 闭包的特性

闭包：可以捕获其所在环境的匿名函数。闭包具体以下特性

- 闭包是个匿名函数
- 可以保存为变量，或者作为参数传给另外一个函数，或者作为另外一个的返回值
- 可以在某个地方创建闭包，然后在另一个上下文调用闭包来完成运算
- 可从其定义的作用域捕获值

### 1.2 生成自定义运动计划程序

下面有一个程序示例，该程序的算法逻辑我们不关心，重点是算法中的计算过程需要几秒钟的时间。而我们优化的目标是不让用户发生不必要的等待，仅在必要时调用该算法，只调用一次。程序如下

```Rust
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 生成运动计划
fn simulated_expensive_calculation(intensive: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensive
}

/**
 * 参数1:强度
 * 参数2:随机数
 */
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        // 在以下代码中，调用了两次耗时函数，可以优化
        println!(
            "Today, do {} pushups",
            simulated_expensive_calculation(intensity)
        );

        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

我们先进行第一测优化，优化后的`generate_workout`函数如下

```Rust
fn generate_workout(intensity: u32, random_number: u32) {

    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_result
        );

        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
```

此时，针对`intensity < 25`的情况，耗时的函数不再需要调用两次，调用是在`if`判断之前执行的。但是此时也引入了一个其他的问题，在`else`的逻辑里，当随机数等于3时，是不需要调用耗时函数的。而现在的写法是无论任何情况下都会调用函数，这显得有点浪费了。

### 1.3 使用闭包

而我们真正希望的是函数定义在一个地方，在需要结果的时候才执行相关代码，这正是闭包的用武之地。下面我们使用闭包来解决该问题。使用闭包优化之后的`generate_workout`函数如下

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    // 定义一个闭包，使用一个变量来接收
    let expensitive_closure = |num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups", expensitive_closure(intensity));
        println!("Next, do {} situps!", expensitive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!("Today, run for {} minutes!", expensitive_closure(intensity));
        }
    }
}
```

## 二、闭包的类型推断和标注

### 2.1 闭包的类型推断

闭包不强制要求标注参数和返回值类型。而函数强制标注参数和返回值类型，因为函数是暴露给用户的接口的一部分，严格定义接口有助于所有人对参数和返回值的类型取得共识。但是闭包并不会被用于这样的暴露接口，闭包会被存在变量里，在使用的时候不需要命名，也不会暴露给代码库的用户，所以闭包并不强制要求标注参数和返回值类型。

闭包通常很短小，只在狭小的上下文中工作，编译器通常能可靠地推断出类型。如果我们明确标注出参数类型也不是不可以，如下示例代码

```Rust
let expensitive_closure = |num: u32| {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

在上面的闭包示例代码中，我们手动添加了参数类型标注，如果我们不添加也完全没有问题，因为rust能自动推断出类型。

### 2.2 函数和闭包的定义语法

首先我们看一个简单的函数示例

```Rust
fn add_one_v1 (x: u32) -> u32 { x + 1 }
```

我们可以使用闭包的方式来实现函数的功能，如下示例代码

```rust
let add_one_v2 = |x: u32| -> u32 { x + 1 };
```

由于闭包可以省略参数类型标注，可以优化成以下的闭包代码

```Rust
let add_one_v2 = |x| { x + 1 };
```

又由于闭包中只有一个表达式，所以可以省略大括号，最终优化成以下代码

```Rust
let add_one_v2 = |x|  x + 1;
```

### 2.3 注意

闭包的定义最终只会为参数/返回值推断出唯一具体类型，如下示例代码

```Rust
fn main() {
    let example_closure = |x| x;

    // 执行到以下的一行代码，将闭包参数绑定为String类型
    let s = example_closure(String::from("hello"));

    // 在下面一行，传了int类型，将会导致编译报错，因为rust已将闭包参数推断其为String类型
    let n = example_closure(5);
}
```

## 三、使用泛型参数和Fn Trait来存储闭包

在“一”中的代码还有一个问题，当执行`intensity < 25`的逻辑的时候，会执行两次耗时函数。我们继续来优化“一”中的代码。我们创建一个struct，把闭包和调用结果存到结构体中。则做到：只会在需要结果时才执行闭包，并切可以缓存结果。这种模式通常叫做记忆话（memozation）或延迟计算（lazy evaluation）。需要注意的条目如下

- struct的定义需要知道所有字段的类型，则需指明闭包的类型
- 每个闭包实例都有自己唯一的匿名类型，即使两个闭包签名完全一样

为了在结构体里使用闭包，我们需要使用泛型和Trait Bound约束。在标准库中，提供了一系列的Fn Trait，所有的闭包都至少实现了以下的Trait之一

- Fn
- FnMut
- FnOnce

下面我们来优化“一”中的代码，下面是优化的过程

首先定义一个结构体

```Rust
struct Cacher<T> 
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}
```

我们把闭包叫做`calculation`，其次把要缓存的值叫做`value`，在运行闭包之前，value的值是null，当运行闭包之后，就会把闭包的结果存放到value的字段中，这就是缓存了。如果以后再次请求这个闭包的结果，有值则直接取即可。

现在我们为结构体添加一个关联函数，再实现取值的方法，如下示例代码

```Rust
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

最后修改`generate_workout`函数如下

```Rust
fn generate_workout(intensity: u32, random_number: u32) {
    // 定义一个闭包，使用一个变量来接收
    let mut expensitive_closure = Cacher::new(|num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });



    if intensity < 25 {
        println!("Today, do {} pushups", expensitive_closure.value(intensity));
        println!("Next, do {} situps!", expensitive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!("Today, run for {} minutes!", expensitive_closure.value(intensity));
        }
    }
}
```

## 四、使用缓存器（Cacher）实现的限制

在“三”的示例中，缓存器存在一些问题，下面我们将探讨解决方案

- Cacher实例假定针对不同的参数arg，value方法总汇得到同样的值：可以使用HashMap代替单个值，key为arg参数，value为执行闭包的结果。

- 只能接收一个u32类型的参数和u32类型的返回值：引入两个或两个以上的泛型参数即可。

## 五、使用闭包捕获环境

### 5.1 使用闭包能捕获环境

闭包可以捕获他们所在的环境，在上面的闭包示例中，我们可以把它当作内部的匿名函数来使用。除此之外，闭包还有函数所不具备的功能，闭包可以访问定义它的作用域内的变量，而普通函数则不能。如下示例代码

```rust
fn main() {
    let x = 4;
    let euqal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

但是捕获他们所在的环境是需要内存开销的，而大部分请求我们不需要捕获环境，也不想为它产生额外的内存开销。所以函数就永远不会产生这类的开销。

### 5.2 闭包从所在环境捕获值的方式

与函数获取参数的三种方式一样，如下示例代码

- 取得所有权：FnOnce
- 可变借用：FnMut
- 不可变借用：Fn

当我们创建闭包时，通过闭包对环境的使用，Rust可以推断出具体使用哪个Trait。所有的闭包都实现了FnOnce；那些没有移动捕获变量的闭包实现了FnMut；那些无需可变访问捕获变量的闭包实现了Fn。实际上追踪其底层原理还是比较复杂，我们现在可以这么理解即可：所有实现了FnMut的闭包都实现了Fn，所有的闭包都实现了FnOnce。

### 5.3 move关键字

在参数列表前使用`move`关键字，可以强制闭包取得它所使用的环境值的所有权。用于：当闭包传递给新线程以移动数据使其归新线程所有时。如下示例的错误代码

```Rust
fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    // 在上面的代码中，x 的所有权已经被移动到了闭包里，下面这行代码继续借用，将会发生错误
    println!("can't use x here: {:?}", x);
}
```

当指定Fn Trait bound之一时，首先用Fn，基于闭包里的情况，如果需要FnOnce或FnMut，编译器会再告诉你。
