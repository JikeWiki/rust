# trait

## 一、概述

Trait告诉Rust编译器，某种类型具有哪些并且可以与其他类型共享的功能，可以使用抽象的方式定义共享行为。`Trait bounds`（约束）指的是泛型类型参数指定实现了特定行为的类型。

Trait与其他语言的借口（interface）类似，但是有区别。

## 二、定义Trait

定义Trait的目的是把方法签名放在一起，来实现某种目的所必须的一组行为。定义trait的相关规则如下

- 关键字：trait
- 只有方法签名，没有具体实现
- trait可以有多个方法，每个方法占一行，以`;`结尾
- 实现该trait的类型必须提供具体方法的实现

如下定义trait的示例代码

```Rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## 三、在类型上实现trait

与类型的实现方法类似，如下示例代码

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

## 四、实现某个trait的约束

可以在某个类型上实现某个trait的前提条件为：这个类型或这个trait在本地crate里定义的。

无法为外部类型来实现外部的trait，这个限制是程序属性的一部分（也就是一致性），更具体地说是**孤儿原则**，之所以这样命名是因为父类型不存在。此规则确保他人的代码不能破坏你写的代码，反之亦然。如果没有这个规则，两个crate可以为同一个类型实现同一个trait，rust就不知道应该使用哪个实现了。

## 五、默认实现

trait可以拥有默认实现，当我们为某个类型实现trait时，可以选择重载或者保留原有的默认实现。默认实现的方法也可以调用trait中其他的方法，即使这些方法没有默认实现。如下示例代码

```Rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("@{} ", self.author)
    }
}
```

需要注意的是，我们无法在方法的重写实现里调用默认的实现。
