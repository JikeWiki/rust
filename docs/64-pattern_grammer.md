# 模式匹配的语法

## 一、匹配字面值

模式可以直接匹配字面值，如下例子

```rust
fn main() {
    let x = 1;

    match x {
        1 => println("one"),
        2 => println("two"),
        3 => println("three"),
        _ => println("anything"),
    }
}
```

## 二、匹配命名变量

命名的变量是可匹配任何值的无可辩驳模式，如下示例代码

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println("Got 50"),
        Some(y) => println("Matched, y = {:?}", y),
        _ => println("Default case, x = {:?}", x),
    }

    println("at the end: x = {:?}, y = {:?}", x, y);
}
```

## 三、多重模式

在match表达式中，使用`|`语法（就是或的意思），可以匹配多种模式，如下例子

```rust
fn main() {
    let x = 1;

    match x {
        1 | 2 => println("one or two"),
        3 => println("three"),
        _ => println("anything"),
    }
}
```

## 四、使用..=来匹配某个范围的值

如下示例

```rust
fn main() {
    let x = 5;
    match x {
        1..=5 => println("one through five"),
        _ => println("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println("early ASCII letter"),
        'k'..='z' => println("late ASCII letter"),
        _ => println("something else"),
    }
}
```

## 五、结构以分解值

可以使用模式来解构struct、enum、tuple，从而引用这些类型值的不同部分。

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(0, b);
}
```

不过，以上在main的代码可以简写成如下

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(0, y);
}
```

我们还可以灵活地使用解构，如下代码

```rust
match p {
    // 要求y的值是0，x随意
    Point { x, y: 0 } => println("On the x axis at {}", x),
    // 要求x的值是0，y随意
    Point { x: 0, y } => println("On the y axis at {}", y),
    // 如果前两个模式都无法匹配的话，将匹配最后一个
    Point { x, y } => println("On neither axis: ({}, {})", x, y),
}
```

## 六、解构枚举

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}.", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}
```

## 七、解构嵌套的struct和enum

示例代码如下

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => (),
    }
}
```

## 八、解构struct和tuple

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
```

## 九、在模式中忽略值

有几种方式可以在模式中忽略整个值或部分值，如下

- `_`忽略整个值
- `_配合其他模式`可以忽略部分值
- `_开头的名称`
- `..`忽略值的剩余部分

### 9.1 使用“_”来忽略整个值

函数有两个参数，但是函数体中只用到第二个参数，如下示例代码

```rust
fn foo(_: i32, y: i32 ) {
    prinltln!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

### 9.2 使用嵌套的“_”来忽略值的一部分

如下例子

```rust
fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // 内配的值：只要两个值都是Some即可
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value 
        }
    }

    println("setting is {:?}", setting_value);

    let numbers = (2, 4, 6, 16, 32);

    match numbers {
        // 只需要元组的某部分值
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
}
```

### 9.3 通过使用“_”开头命名来忽略未使用的变量

正常情况下，创建一个变量，但是没有使用，rust编译器就会发出警告。有时候我们想要创建一个临时变量，但有不想让编译器发出警告，就可以使用下划线开头的方式。如下例子

```rust
fn main() {
    let _x = 5;
    ley y = 10;
}
```

以上代码编译之后，`x`不会发生警告，而`y`会发出警告。我们再来看一个示例

```rust
fn mian() {
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}
```

以上代码编译将会发生报错，因为在嗲用最后的`println!`之前，`s`已经将所有权移动到了`_s`，如果将代码修改为如下内容得以解决问题

```rust
fn mian() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}
```

### 9.4 使用..来忽略值的剩余部分

如下例子

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        // 只需要用到struct的x字段
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // 只需要用到元组中的第一个数和最后一个数
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}
```

### 9.5 使用match守卫来提供额外的条件

match守卫就是match arm模式后额外的if条件，想要匹配该条件也必须满足，macth守卫适用于闭单独的模式更复杂的场景。如下示例

```rust
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x);
        Some(x) => println("{}", x),
        None => (),
    }
}
```

再有另外一个示例，如下代码

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match y {
        Some(50) => println!("Got 50");
        Some(n) if n == y => println("Matchd, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

我们还可以在多重模式中使用match守卫，如下示例

```rust
fn main() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println("yes"),
        _ => println!("no"),
    }
}
```

### 9.6 @绑定

`@`符号让我们可以创建一个变量，该变量可以在测试某个值是否与模式匹配的同时保存该值。如下示例代码

```rust
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable);
        }
        Message::Hello {
            id: 10..=12,
        } => {
            println!("Found an id in another range");
        }
        Message::Hello { id } => {
            println!("Found an other id: {}", id);
        }
    }
}
```

最后输出结果为`5`，即变量`id_variable`存储了匹配的值。
