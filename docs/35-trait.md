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

## 六、使用trait作为参数

### 6.1 impl Trait

使用`impl Trait`的语法，可以适用于简单的函数传参情况。加入我们希望传入的参数实现了一个`Summary`这个trait，那么我们可以使用如下的参数声明方式

```Rust
pub fn notify(item: impl Summary) {
    println("breaking news! {}", item.summarize());
}
```

那么我们在调用此函数的使用，可以传入`Tweet`，也可以传入`NewArticle`。

### 6.2 Trait bound

使用`Trait bound`可以与复杂的情况，如下示例代码

```Rust
pub fn notify<T: Summary>(item: T) {
    println("breaking news! {}", item.summarize());
}
```

如果宝航两个参数，使用`Trait bound`的写法如下

```Rust
pub fn notify<T: Summary>(itema: T, item2: T) {
    println("breaking news! {}", item1.summarize());
}
```

实际上`impl Trait`是`Trait bound`的语法糖。

### 6.3 使用+指定多个trait bound

如果使用`impl Trait`的方式，示例代码如下

```Rust
use std::fmt::Display;

pub fn notify(item: impl Summary + Display) {
    println("breaking news! {}", item.summarize());
}
```

使用`Trait bound`的写法如下

```Rust
use std::fmt::Display;

pub fn notify<T: Summary + Display>(item: T) {
    println("breaking news! {}", item.summarize());
}
```

### 6.4 在方法签名后面使用where子句

如果一个函数要求的变量需要实现过多的Trait，代码的可读性会变低，很不直观，如下示例代码

```Rust
pub fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("breaking news! {}", a.summarize());
}
```

我们可以使用where子句还简化Trait的约束，如下示例代码

```Rust
pub fn notify<T, U>(a: T, b: U) -> String
where T: Summary + Display,
    U: Clone + Debug,
{
    format!("breaking news! {}", a.summarize());
}
```

## 七、使用trait作为返回值类型

使用`impl Trait`声明返回的类型为Trait，如下示例代码

```Rust
pub fn notify(flag: bool) -> impl Summary {
    // ......
}
```

使用`imple Trait`只能返回确定的同一种类型，返回可能不同类型的代码会报错，如下代码将报错

```Rust
pub fn notify(flag: bool) -> impl Summary {
    if flag {
        NewArticle {
            // ......
        }
    } else {
        Tweet {
            // ......
        }
    }
}
```

## 八、使用PartialOrd实现数据比较

如下示例代码

```rust
use std::result;

fn largest<T: PartialOrd +Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest(&str_list);
    println!("The largest word id {}", result);
}
```

## 九、使用Trait Bound有条件的实现方法

在使用泛型类型参数的 impl 块上使用Trait bound，我们可以有条件的胃了实现特定 trait 的类型来实现。如下示例代码

```Rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 所有Pair<T> 都拥有new函数
impl <T> Pair<T>{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 实现了Display 和 PartialOrd 的Pair<T> 猜拥有com_display函数
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
}

fn main(){

}
```

也可以为实现其他Trait的类型有条件的实现某个Trait。为满足Trait Bound的所有类型上实现Trait叫做覆盖实现（Blanket implementations）。
