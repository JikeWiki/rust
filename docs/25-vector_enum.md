# vector 和 enum

在vector里只能存放相同的数据，当我们需要存储不同的数据时。这时候我们就使用使用enum加上vector来存储数据。enum的变体可以附加不同类型的数据，enum的变体定义在同一个enum类型下。示例代码如下

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(12.12),
    ];
}
```
