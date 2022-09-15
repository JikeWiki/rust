# struct的方法

## 一、概述

方法和函数很类似：使用fn作为关键字，有方法名称、参数、返回值

方法和函数不同之处：

- 方法是在struct（或enum、trait对象）的上下文定义
- 方法的第一个参数总是self，表示方法被调用的struct实例

## 二、方法的使用

### 2.1 方法的实现

我们可以把计算长方形面积使用方法来实现，如下代码

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn main() {
    let ract = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", ract.area())
}
```

在main函数中，我们也可以拥有 ract 可变的借用，如下代码

```rust
impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width * self.length
    }
}

fn main() {
    let mut ract = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", ract.area())
}
```

在impl块里定义方法，方法的第一个参数可以是 `&self`，也可以获得其所有权或可变借用。和其他参数一样。使用方法可以让代码拥有更好的组织。每个struct可以允许拥有多个`impl`块。

### 2.2 方法的运算符

- C/C++：object->something() 和 (*object).something() 一样

- Rust没有 `->` 运算符

- Rust会自动引用或者解引用这种行为

在调用方法时，Rust根据情况自动添加`&`、`&mut`或`*`，以便 object 可以匹配方法的签名，如下来两行代码时等价的

```rust
p1.distance(&p2)
(&p1).distance(&p2)
```

## 三、关联函数

可以在impl 块里定义不把self作为第一个参数的函数，它们叫做关联函数（不是方法）

例如

```rust
String::form()
```

关联函数通常用于构造器，如我们可以创建 长方形的关联函数`square`，如下代码

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}

fn main() {
    let ract = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", ract.area());

    let s = Rectangle::square(20);
    println!("{}", s.area());
}
```

`::`符号用于关联函数，还可以用于模块创建命名空间。
