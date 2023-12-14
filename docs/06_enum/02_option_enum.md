# option 枚举

## 一、概述

- option枚举定义于标准库中
- 在prelude（预导入模块）中
- 描述了：某个值可能存在（某种类型）或不存在的情况

## 二、Rust没有NULL

在很多编程语言都有NULL，NULL是一个值，它表示“没有值”。这些语言中，一个变量可以处于两种状态：空值（NULL）、非空。NULL的问题在于，当你尝试使用NULL值那样使用NULL值的时候，就会引起某种错误。rust没有NULL，但是NULL的概念还是有用的，因为某种原因变得无效或缺省的值。

rust中提供了类似NULL概念的枚举：`Option<T>`，在标准库中定义如下

```rust
enum Option<T> {
    Some(T),
    None,
}
```

该枚举包含在Prelude（预导入模块）中。可以直接使用：

```rust
Option<T>
```

该枚举的两个变体也可以直接使用

```rust
Some(T)
None
```

具体示例如下代码

```rust
fn main() {
    // 有效的值
    let some_number = Some(5);
    let some_string = Some("A String");

    // 无效的值
    let some_number: Option<i32> = None;
}
```

## 三、`Option<T>`比`Null`好在哪里？

`Option<T>`和`T`是不同的类型，不可以把`Option<T>`直接当成`T`，若想使用`Option<T>`中的`T`，必须将它转换为`T`。避免了使用非有效值进行计算而导致错误，如在`C#`中假设某个变量值为`NULL`，参与计算时将导致错误，如下代码

```C#
string a = null;
string b = a + "12345";
```

所以rust的`Option<T>`的设计就避免了NULL值泛滥的情况，使得程序更加安全！
