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

泛型每次实现trait的时候必须标注具体的类型。而且可以为一个类型多次实现某个Trait（使用不同的泛型参数）。如下示例代码

```rust
1:11
```
