# Cargo

## 1. Cargo 简介

对于简单的程序，我们使用 `rustc` 命令即可编译，但对于较为复杂的程序，应当使用 `Cargo` 这个工具。`Cargo` 是 Rust 的构建系统和包管理工具。主要用于依赖库安装和代码构建。我们使用脚本安装 Rust 的时候已经安装上了 Cargo，使用以下查看 Cargo 版本的命令可以验证 Cargo 是否已经正常工作

```shell
cargo --version
```


## 2. 使用Cargo的基本使用

创建项目，如下命令

```shell
cargo new hello_cargo
```

命令执行之后，生成一个名为 `hello_cargo` 项目目录，目录中包含以下内容

- `src`: 这个目录用于存放源代码，并默认生成了一个 `main.rs` 文件
- `.gitignore`: 默认生成的 gitignore 文件, 实际上，我们使用 Cargo 创建项目的时候，Cargo 帮我们在项目目录初始化了 git 仓库
- `Cargo.toml`: TOML（Tom's Obvious, Minimal Language）格式，是 Cargo 配置文件的格式，以下是默认的内容

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- `package`: 是一个区域标题，在这里表示下方配置用来配置包（package）的
- `package.name`: 项目名
- `package.version`: 项目版本
- `package.authors`: 项目作者
- `package.edition`: 项目使用的Rust版本

- `dependencies`: 该区域之下的内容，标识项目依赖的代码包或代码库，在 Rust 里，代码的包或库被称为 crate


> Cargo生成的 `main.rs` 在 src 目录下，而 



--- p4 04:06 