# 安装二进制Crate

## 一、概述

使用`cargo install`来安装二进制crate，安装来源来自<https://crates.io>

只能安装具有二进制目标（binary target）的crate，二进制目标binary target是一个可运行的程序，由`src/main.rs`或其他被指定为二进制文件的crate生成。

通常在对应的crate仓库中，README文件里里有关于crate的描述。

## 二、cargo install

cargo install安装的二进制存放在根目录的bin文件夹。

如果使用`rustup`安装的rust，没有进行任何配置，那么二进制存放目录是`$HOME/.cargo/bin`。为了确保rust安装的二进制程序能够运行，我门要确保该目录在`PATH`环境变量中，我门才能在终端上执行安装的二进制文件。

## 三、使用自定义命令扩展cargo

cargo被设计成可以使用子命令来扩展。例如：如果在PATH中的某个二进制是`cargo-something`，那么我们可以像自命令一样运行，如下

```bash
cargo something
```

类似这样的自定义命令可以通过`cargo list`命令列出来。这种设计的有点在于，我门可以使用`cargo install`来安装扩展，并想内置工具一样运行。
