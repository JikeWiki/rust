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

> p21 04:17
