# rust示例

## 1. 概述

我们先编写一个函数，用于计算长方形的的面积，如下代码

```rust
fn main() {
    let width = 30;
    let length = 50;
    println!("{}", area(width, length));
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}
```

 在上面代码中，我们可以看到 width 和 length 似乎没有关联，它们两看起来没有任何关系。如果我们能把长和宽组合到一起，代码将更容易理解和维护。我们可以使用学习过的元组来行来存储长和宽，如下代码

```rust
fn main() {
    let rect = (30, 50);
    println!("{}", area(rect));
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
```

然而换成元组之后，虽然两个变量在一起了，但是每一个元素没有名称，我们不知道哪个是长，哪个是宽。此时我们就可以使用结构体来存储长放心的长与宽。

## 2. 结构体的使用

我们使用结构体来实现计算长方形面积的功能，如下代码

```rust
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
```

如果我们使用`println!`宏和`{}`占位符直接打印`reat`，将会报错，如下代码

```rust
println!("{}", rect);
```

因为可以使用`println!`来打印的数据，它们都实现了`std::fmt::Display`这个接口，rust里的基本数据类型已实现了该接口，但是结构体没有实现该接口。不过我们可以使用`{:?}`或者`{:#?}`来打印结构体，但使用这种方法需要实现debug，如下代码

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    // area 函数借用了area的实例，调用完成之后主函数仍然可以调用reat
    println!("{}", area(&rect));

    println!("{:?}", rect)
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
```

在上面的代码中，我们用到`derive`这个关键词，代表派生一个Trait，实际上rust提供很多可派生的Trait，以上用到的`Debug`只是其中的例子。

## 3. 总结

在这篇文章，我们介绍了权限的打印方法，下面做一个总结

rust的基本数据类型实现了`std::fmt::Display`这个接口，可以使用`{}`占位符进行打印；如果我们希望自定义的结构体可以实现打印功能，需要实现`std::fmt::Debug`接口，实现的方式是在结构体定义的头部添加上以下一行代码

```rust
#[derive(Debug)]
```

加入一个结构体实现了`std::fmt::Debug`接口，那么在打印的时候就可以使用`{:?}`占位符或者`{:#?}`占位符。
