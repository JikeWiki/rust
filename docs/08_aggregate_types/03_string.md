# 创建和使用字符串

## 一、概述

Rust开发者特别时新手经常被字符串所困扰，二困扰的原因主要有三个

- Rust倾向于包喽可能的错误
- 字符串数据结构复杂
- 使用UTF8编码

### 1.1 字符串是什么？

字符串是 Byte 的集合，并且提供了一些方法，这些方法能够将 byte 解析为文本。

Rust的核心语言层面，只有一个字符串类型：字符串切片 str （或&str）。

字符串切片是 存储在其他地方、并且使用UTF8编码的字符串的引用。字符串字面值存储在二进制文件中，也是字符串切片。

### 1.2 String类型

String类型来自标准库而不是核心语言，它是可增长、可修改、可拥有（可获得所有权）的一种数据类型，采用UTF8编码。

- 在rust中说的字符是指？

通常指的是 String 或者 字符串切片（&str）这两种类型，而不是其中的一种。

Rust标准库中还包含了很多其他的字符串类型，例如：OsString、OsStr、CString、CStr。以`String`结尾的通常与`String`相关，通常拥有所有权，以`Str`结尾通常与`str`相关，通常可以借用所有权。它们可以存储不同编码的文本或在内存中以不同的形式展现。

## 二、创建一个新的字符串（String）

很多`Vec<T>`的操作都可用于String。使用`String::new()`函数创建一个空的字符串。但很多时候使用初始值创建String，以下介绍两种方法

### 2.1 使用 to_string方法

`to_string`方法可用于实现了Display trait的类型，包括字符串字面值，如下示例代码

```Rust
fn main() {
    let data = "initial contents";
    let s = data.to_string();

    let s1 = "initial contents".to_string();
}
```

### 2.2 使用String::from函数

`String::from`函数可以从字面值来创建字符串，如下示例代码

```Rust
fn main() {
    let s = String::from("initial contents");
}
```

## 三、如何更新String

### 3.1 push_str

`push_str()`方法，把一个字符串切片附加到String上，如下示例代码

```Rust
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s);
}
```

### 3.2 push

`push()`方法，把单个字符添加到String上，如下示例代码

```rust
fn main() {
    let mut s = String::from("lo");
    s.push('l');
}
```

### 3.3 +运算符

`+`运算符，可以对字符串进行拼接，`+`号前面的变量是String类型，`+`后面的变量是字符串切片，如下示例代码

```Rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World");

    let s3 = s1 + &s2;

    println!("{}", s3);
}
```

`+`运算符实现了类似这个签名的方法`fn add(self, s: &str) -> String {...}`。上面示例代码中，`s2`是一个String类型，则`&s2`是一个String类型的引用。

`+`运算符之后的希望是一个字符串切片， 这里之所以能够编译通过，是因为rust在编译的时候，使用了一种“解引用强制转换”（deref coercion）的技术。它把String的引用转为了字符串切片，所以编译可以顺利通过。

在执行拼接代码之后，`s1`的所有权被移到`add`函数，如果在下面的代码中继续使用将会报错；而`s2`使用了`&`来标注，`add`函数只是借用了`s2`的所有权，在下面的代码仍然可以继续使用。

### 3.4 format!宏

`format!`宏可以连接多个字符串，假设我们希望将多个单词使用特定符号进行拼接，使用`+`运算符的实现如下代码

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s4);
}
```

如果我们使用`format!`宏将字符串拼接，如下示例代码

```rust
let s = format!("{}-{}-{}", s1, s2, s3);
println!("{}", s);
```

`format!`不会取得任何参数的所有权，而是将多个参数格式之后返回一个新的字符串。
