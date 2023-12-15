# 使用trait存储不同类型值

## 一、概述

假设我们有这样的需求：创建一个GUI工具，它会遍历某个元素（指的是GUI原属）的列表，依次调用元素的draw方法进行绘制。例如Button、TextField等元素。

对于以上的需求，在面向对象的语言里，我们通常先定义一个Component父类，里面定义了draw方法，接下来定义各个元素的类（Button、TextField等），它们都继承于Component类，再覆盖draw方法。

而在rust里是没有继承功能的，所以说如果想用rust来构建这个GUI工具，我们就得使用其他的方法，即为共有的行为定义一个trait。

## 二、为共有行为定义一个trait

在rust里，我们避免将struct或enum称为对象，虽然它们是持有数据，但是它们的方法实现是在impl块里，而struct和enum和impl块是分开的。而trait对象有些类似于其他语言中的对象，因为trait对象在某种程度上实际上是组合了数据于行为。trait对象于传统的对象不同的地方在于，我们无法为trait对象添加数据。trait对象被专门用于抽象共有行为的，它没有其他语言中的对象那么通用。

我们将实现的示例代码存在`src/s60_trait_save_different_val`中。

## 三、trait对象执行的是动态派发

将trait约束作用于泛型时，rust编译器会执行单态化。编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现。

通过单态化生成的代码会执行静态派发（static dispatch），在编译过程中确定调用的具体方法。

所谓的动态派发（dynamic dispatch），它无法在编译过程中确定你调用的究竟是哪个方法，编译时会产生额外的代码以便在运行时找出希望调用的方法。如果使用trait对象，就会执行动态派发，那么将会导致产生运行时开销，并且阻止编译器内联方法代码，使得部分优化无法进行。

## 三、Trait对象必须保证对象安全

只能把满足对象安全（object-safe）的trait转化为trait对象。rust采用了一系列规则来判定某个对象是否安全，我们只需要记住两条规则

- 方法的返回类型不是Self
- 方法中不包含任何泛型类型参数

我们来看一个示例，在标准库中，Clone trait就是不符合对象安全的例子，Clone trait的`clone`方法如下

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

如果我们在“二”中的示例代码使用`Clone` trait作为 `Screen` 结构体 conponents 的对象，如下代码

```rust
pub struct Screen {
    pub commponents: Vec<Box<dyn Clone>>
}
```

程序将会报错，因为Clone trait的`clone`方法返回`Self`，换句话说，它不是对象安全的。
