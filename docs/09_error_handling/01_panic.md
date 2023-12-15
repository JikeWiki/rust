# 不可恢复错误与panic

## 一、概述

Rust的可靠性：错误处理，在大部分情况下，编译时提示错误，并处理。

Rust错误的分类如下：

- 可恢复：例如文件未找到，可在此尝试
- 不可恢复：bug的另外所说法，通常是逻辑错误，比如访问的索引超出范围

## 二、Rust的错误处理

Rust没有类似异常的机制，针对可恢复的错误，使用 `Result<T, E>`，针对不可恢复的错误，Rust提供了`panic!`宏，当执行`panic!`宏时，程序会执行以下逻辑

- 打印一个错误信息
- 展开（unwind）、清理调用栈（Stack）
- 退出程序

为了应对panic，展开或终止（abort）调用栈，两种过程有以下区别

程序展开调用栈（工作量大）：Rust沿着调用栈往回走，清理每个遇到的函数中的数据；立即终止调用栈：不进行清理，直接停止程序，内存需要OS进行清理。如果想让二进制文件更小，可以把设置从“展开”改为“终止”。修改步骤如下：

- 在`cargo.toml`中适当的`profile`部分设置`panic='abort'`，如下示例代码

```Rust
[package]
name = "s27_string_cut"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[profile.release]
panic = 'abort'
```

## 三、使用panic!产生的回溯信息

panic!可能出现在我们写的代码中，也可能出现在我们依赖的代码中。通过调用panic!的函数回溯信息来定位引起问题的代码。具体的做法是设置环境变量`RUST_BACKTRACE=full`可得到回溯信息。为了获取带有调试信息的回溯，必须启用调试符号（不带 --release）。
