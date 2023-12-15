# Deref Trait

## 一、概述

如果一个类型实现了`Deref Trait`使我门可以自定义解引用`*`的行为。通过使用Deref，智能指针可以像常规引用一样来处理。

## 二、解引用运算符

常规的引用也是一种指针，我们先看一个示例

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

x存一个i32类型的整数，y是x的引用，所以y相当于是一个指针。第一个断言中`5`和`x`是相等的，没有问题。y是个指针，这里是正数`5`的引用，它指向一个值: `5`。如果想把它指向的值取出来，那就是在前面加一个解引用符号`*`，所以`*y`和`5`也是相等的。

## 三、使用Box<T>代替上例中的引用

我门可以原因的引用换成`Box<T>`，如下代码

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

以上代码也没有任何问题，所以说`Box<T>`可以像引用一样来处理。

## 四、定义自己的智能指针

`Box<T>`被定义成一个拥有元素的tuple struct，下面我们定义一个`MyBox<T>`，它也是一个tuple struct，即元组结构体。如下代码

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}
```

`MyBox`实际上是一个有名称的元组，该元组只有一个元素。不过如果我们用于替代`Box`，是不能实现的，它将不能为解引用。如果需要能够解引用，我门需要实现Deref Trait。

## 五、实现Deref Trait

标准库中的Deref trait要求我们实现一个deref的方法，该方法会借用self，并返回一个指向内部数据的引用。如下示例代码

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.d));
}
```

因为`MyBox<T>`实现了`Deref` trait，所以使用`*`可以进行解引用。实际上，`*y`就等同于`*(y.deref())`，因为rust编译器在编译时会将`*y`会隐式地展开成`*(y.deref())`。

## 六、函数和方法的隐式解引用转化（Deref Coerion）

隐式解引用转化（Deref Coercion）是为函数和方法提供的一种便捷特性。假设`T`实现了`Deref` trait，Deref Coerion可以把`T`的引用转化为`T`经过`Deref`操作后的生成引用。

当把某类型的引用传递给函数或方法时，但它的类型于定义的参数不匹配，Deref Coerion就会自动发生。编译器会对deref进行一系列调用，来把它转为所需的参数类型。这个操作在编译时完成，没有额外的性能开销。我们在五中的示例代码中追加一个函数，如下

```rust
fn hello(name: &str) {
    println!("Hello, {}", name);
}
```

接着在`main`函数中调用

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));

    hello("Rust");
    hello(&m);
}
```

 在上面的代码中，`hello`方法需要接收一个字符串切片。`m`是`MyBox<String>`的一个引用，由于`MyBox<T>`实现了`deref` trait，所以rust可以调用`deref`方法把`MyBox<String>`的引用转化为`String`的引用。而`String`也实现了`deref` trait，它的实现是返回字符串切片。所以上面`hello`方法传入的`&m`经过隐式解引用转化之后，类型就匹配了。

如果rust没有实现隐式解引用，我们的调用方法如下

```rust
hello(&(*m)[..]);
```

充满了各种符号，难以阅读。 所以说只要这个类型实现了`Deref` trait，rust就会自动分析类型，并不断尝试调用`deref`方法，来让它与函数或是方法定义的参数类型匹配。而且这个过程是在编译的时候完成的，对程序的运行时不会额外的性能开销。

## 七、解引用与可变性

可使用DerefMut trait重载可变引用`*`运算符。在类型和trait在下列三种情况发生时，rust会执行defref coercion

- 当`T`实现了`Deref<target=U>`，允许`&T`转为`&U`
- 当`T`实现了`DerefMut<target=U>`，允许`&mut T`转为`&mut U`
- 当`T`实现了`Deref<target=U>`，允许`&mut T`转为`&U`

rust可以把一个可变引用转为不可变引用，但是反过来不行。因为将不可变引用转为可变引用，借用规则要求这个引用必须时唯一的，但这点无法保证。上面列出的情况稍微记一下就行，这里还有些知识为涉及到。
