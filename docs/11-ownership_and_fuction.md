# 所有权与函数

## 1. 概述

在语义上，将值传递给函数和把值赋给变量是类似的：将值传递给函数将会发生移动或复制。如下示例

```rust
fn main() {
    // 声明String类型变量
    let s = String::from("Hello world");
    // s 被移动到函数里面
    take_ownership(s); // 此行以后，s 不再有效

    // x 是一个i32类型的整数，等于 5
    let x = 5;
    // 因为 i32 实现了 copy trait，所以往函数里传的 x 是一个拷贝
    makes_copy(x); // 此行代码以后，x 依然有效

    println!("x: {}", x)
} // main函数执行完成后，x和s都离开了作用域，因为 s 已经被移动到了 take_ownership 函数里面，所以不在执行内存释放操作

fn take_ownership(some_string: String) {
    println!("{}", some_string)
} // 执行完毕后，some_string 离开了作用域，内存被释放

fn makes_copy(some_number: i32) {
    println!("{}", some_number)
} // 执行完毕后，some_number 离开了作用域，拷贝的值被释放
```

再看以下的代码

```rust
fn main() {
    // s1 变量从 gives_ownershio 函数中获得所有权
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    // s2 将所有权转移给 takes_and_gives_back 函数，并返回一个 String 类型的数据，将所有权转移给 s3
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

一个变量的所有权总是遵循同样的模式：

- 把一个值赋给其他变量时就会发生移动

- 当一个包含 heap 数据的变量离开作用域，他的值就会被 drop 函数清理，除非数据的所有权移动到另一个变量上

## 2. 让函数使用某个值，但不获得所有权

当我们将变量赋给函数时，所有权会丢失，在函数的最后再将所有权返回，即可实现 调用处保留了参数的所有权

```rust
fn main() {
    let s1 = String::from("hello");

    // 在main函数里将s1的所有权移动给calculate_length函数，在 calculate_length 再将所有权移交回来
    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    // calculate_length 函数再将s的所有权移动给 调用它的函数
    (s, length)
}
```

但以上这种做法我们不得不将参数传进函数里，再将参数返回回来，而Rust中的“引用”特性可以完美地解决这个问题。
