# 切片

## 1. 概述

Rust的另一种不持有所有权的数据类型：切片（slice）


我们先思考一个问题，编写一个函数，让它实现下列功能：

- 它能接收字符串作为参数
- 返回它在这个字符串里找到的第一个单词
- 如果函数没有找到任何空格，那么整个字符串就返回

我们可以写出如下函数

```rust
fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
```

我们再添加调用的主函数

```rust
fn main() {
    let mut s = String::from("Hello world");
    let wordIndex = first_world(&s);

    s.clear();
    print!("{}", wordIndex)
}
```

我们清空变量`s`之后，如果我们我们还用`wordIndex`去提取字符串内容，将会报错，引起bug。所以这类的函数设计我们必须随时关注`wordIndex`的有效性，确保字符串的索引和字符串具有同步性。但是这类工作往往比较繁琐，而且容易出错。Rust为这类问题提供了一个解决方案，就是我们要学习的字符串切片。


## 2. 概念

字符串切片是指向字符串其中一部分内容的引用


形式：[开始索引..结束索引]

- 开始索引：切片起始位置的索引值
- 结束索引：切片终止索引位置的下一个索引值


如下示例代码

```rust
fn main() {
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}, {}", hello, world);
}
```

如果是切片的开始是字符串的开始位置，切片的结束是字符串的结束位置，我们可以写成如下语法


```rust
let hello = &s[..5];
let world = &s[6..];
```

如果我们的切片是整个字符串，则可以写成如下形式

```rust
let whole = &s[..];
```

## 3. 重写获取字符串第一个单词

我们现在可以使用切片的功能重写1中获取字符串中第一个单词的逻辑，如下代码

```rust
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
```

## 4. 字符串字面值是切片

字符串字面值被直接存储在二进制程序中，如下代码

```rust
let s = "Hello, World!";
```

变量s的类型是&str，他是一个指向二进制程序特定位置的切片

- &str 是不可变引用，所以字符串字面值也是不可变的


## 5. 将字符切片作为参数传递

在上面的代码中，存在如下关键内容

```rust
fn first_world(s:&String) -> str {
```

而有经验的Rust开发者会采用&str作为参数类型，因为这样就可以同时接收 String 和 &str 类型的参数了。如下代码

```rust
fn first_world(s:&str) -> str {
```

- 如果使用字符串切片，直接调用该函数
- 如果使用String，可以创建一个完整的String切片来调用该函数

定义函数时，使用字符串切片来代替字符串引用会使我们的API更加通用，且不会损失任何功能。如下代码


```rust
fn main() {
    let my_string = String::from("Hello world");
    let word_index = first_world(&my_string[..]);

    let my_string_literal = "hello world";
    let word_index1 = first_world(my_string_literal);

    println!("{} {}", word_index, word_index1)
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
```


## 6. 其他类型的切片

处理字符串切片，其他数据类型也可以用切片，如下代码

```rust
fn main() {
	let a = [1, 2, 3, 4, 5];
	let slice = &a[1..3];
}
```












