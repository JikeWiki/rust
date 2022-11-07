# 高级trait

## 一、在rust中使用关联类型来制定占位类型

关联类型（assciated type）是trait中类型占位符，它可以用于trait的方法签名中。具体地说可以定义出包含某些类型的trait，而在实现前无需知道这些类型是什么。如下例子

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    println!("Hello, world");
}
```

## 二、关联类型与泛型的区别

每次实现泛型trait的时候必须标注具体的类型，而且可以为一个类型多次实现某个Trait（使用不同的泛型参数）；而实现关联类型trait的时候，我们无需标注类型，但是要在里面指明具体的关联类型，而且我门无法为单个类型多次实现某个Trait。如下示例代码

```rust
// 关联类型Trait
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// 泛型类型Trait
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

// struct
struct Counter {}

// 实现关联类型的Trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// 实现泛型类型的trait
impl Iterator2<String> for Counter {

    fn next(&mut self) -> Option<String> {
        None
    }
}

// 再次实现泛型类型的trait
impl Iterator2<u32> for Counter {

    fn next(&mut self) -> Option<u32> {
        None
    }
}

fn main() {
    println!("Hello, world!");
}
```

## 三、默认泛型参数和运算符重载

可以在使用泛型参数时为泛型指定一个默认的具体类型，语法为：`<PlaceholderType=ConcreteType>`，这种技术常用于运算符重载（operator overloading）。虽然Rust不允许创建自己的运算符以及重载任意的运算符，但可以通过实现`std::ops`中列出的那些trait来重载一部分相应的运算符。如下示例

```rust
use std::ops::Add;

#[derive(Debug, PartiaEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
              Point { x: 3, y: 3 });
}
```

在上面的示例代码中，`std::ops::Add` trait 使用默认的泛型参数类型为 `<Rhs = Self>`，也就是说，在我们实现`std::ops::Add` trait的过程中，如果没有为`Rhs`指定一个具体的参数类型，那么`Rhs`类型就默认为`Self`，所以上面的代码中`Rhs`就是`Self`即`Point`。

下面我们做一个毫米和米相加的例子，就需要指定泛型参数类型，如下示例代码

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add(Meters) for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Point) -> Point {
        Millimeters(self.0 + (other.o * 1000))
    }
}

fn main() {
    
}
```

## 四、默认泛型参数的主要应用场景

主要有两个应用场景，如下

- 扩展一个类型而不破坏现有的代码
- 允许在大部分用户都不需要的特定场景下进行自定义

## 五、完全限定语法（Fully Qualified Syntax）与如何调用同名方法

我们看如下例子

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your caption speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn mian() {
    let person = Human;
    person.fly();
}
```

在上面示例代码中，两个struct分别实现了`Human`trait，实现了`fly`方法，并且`Human` trait有自己的fly方法。最后在`main`函数中调用了`fly`方法，那么会调用到哪个方法呢？在上面的示例代码中，将调用本身的`fly`方法。此时又有一个问题，那么如何调用实现了两个trait的方法，如下示例代码

```rust
fn mian() {
    let person = Human;
    // 调用本身的方法
    person.fly();
    // 调用Pilot的方法
    Pilot::fly(&person);
    // 调用Wizard的方法
    Wizard::fly(&person);
}
```

需要注意的是，`Pilot`可以被多个struct实现，那么如何知道为什么调的是`Human`的方法，是因为有`self`参数，传入的`&person`参数中它就是`Human`类型。我们再来看一个不一样的示例

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot");
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy");
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

在上面的示例代码中，`Dog`本身有`baby_name`关联方法，它输出的是“Spot”，那么如何调用`Animal`的方法呢？这时候我们可以使用完全限定语法。

完全限定语法的格式为：`<Type as Trait>::function(receiver_if_method, next_arg, ...)`，这种语法可以在任何调用函数或方法的地方使用，并且它允许忽略那些从上下文能推导出来的部分。但要记住的是，当rust无法区分我们调用哪个具体实现的时候，才需使用这种语法，因为这种语法写起来比较麻烦，不该轻易使用。如下示例代码

```rust
// ...

fn main() {
    // 调用本身的方法
    println!("A baby dog is called a {}", Dog::baby_name());
    // 调用实现Animal的方法
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

## 五、使用supertrait来要求trait附带其他trait的功能

其实相当于一个trait继承另外一个trait。有时候我们需要再一个trait中使用其他trait的功能，也就是说需要间接被依赖的trait也被实现，那个被间接依赖的trait就是当前trait的supertrait。加入我们希望一个trait有打印功能的方法，并希望打印的方法能够调用`to_string`方法，实现如下示例代码

```rust
use std::fmt;

// 依赖于fmt::Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", "*".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// 为Point实现OutlinePrint
impl OutlinePrint for Point {}

// Point实现了OutlinePrint就必须实现fmt::Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {}
```

## 六、使用newtype模式在外部类型上实现外部trait

孤儿规则：只有当trait或类型定义在本地包时，才能为该类型实现这个trait。我们可以使用newtype模式来绕过这规则，即使用tuple struct（元组结构体）创建一个新的类型放在本地。如我们想为`Vec`实现`fmt::Display` trait，而`Vec`和`fmt::Display`都在定义在外部包中，所以我们无法直接为`Vec`实现`fmt::Display`。实现方案如下

```rust
use std::fmt;

// 定义Wrapper元组结构体，包住Vec
struct Wrapper(Vec<String>);

// 为Wrapper实现Display
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用selef.0即取出vec
        write!("[{}]", selef.0.join(", "));
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```
