# match表达式

## 一、概述

match是一个控制流运算符，用于匹配值与模式，并执行相应的代码。

- match允许一个值与一系列模式进行匹配，并执行匹配的模式对应的代码
- 模式可以是字面值、变量名、通配符...

match表达式会一次判断模式列表，如果符合条件，将执行对应的代码。如果包含多行代码，可以使用`{}`，如下示例

```Rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("panny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn main() {}
```

## 二、绑定值的模式

匹配的分支可以绑定到被匹配对象的部分值，因此可以从enum变体中提取，示例代码如下

```Rust
#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("panny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state)
            25
        },
    }
}


fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c));
}
```

## 三、匹配Option<T>枚举

macth可以匹配枚举值，如下示例代码

```Rust
fn main() {
    let five = Some(5);
    let six = puls_one(five);
    let none = puls_one(None);
}

fn puls_one(x :Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
```

## 四、注意

match必须穷枚举所有可能的值，特别是 Option类型的数据，如果不想穷枚举所有可能的值，需要使用`_`来代替，如下示例代码

```Rust
fn main() {
    let v = 0u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
```
