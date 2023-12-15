# if let

if let 是一个简单的控制流，它值关心一种匹配忽略其他匹配的情况。如下示例代码

```Rust
fn main() {
    let v = Some(0u8);

    if let Some(3) = v {
        println!("three");
    }
}
```

在只关心一种控制流匹配值的情况下，使用 `if let` 代替 `match` 让我们编写更少的代码。也放弃了穷枚举的可能。我们可以把`if let`看作`match`的语法糖。也就是针对`match`的某一种特定模式，可以使用 `if let`来代替。`if let`还可以搭配`else`来使用，如下代码

```rust
fn main() {
    let v = Some(0u8);

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}
```

如果我们使用 `match` 表达式来是实现，则写法如下

```rust
match v {
    Some(3) => println!("three"),
    _ => println!("others");
}
```
