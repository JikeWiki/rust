# Cargo构建工具

## 1. Cargo 简介

对于简单的程序，我们使用 `rustc` 命令即可编译，但对于较为复杂的程序，应当使用 `Cargo` 这个工具。`Cargo` 是 Rust 的构建系统和包管理工具，主要用于依赖库安装和代码构建。我们使用脚本安装 Rust 的时候已经安装上了 Cargo，使用以下查看 Cargo 版本的命令可以验证 Cargo 是否已经正常工作

```shell
cargo --version
```

## 2. 使用Cargo创建项目

### 2.1. 初始化项目

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

### 2.2. 包信息配置

- `package`: 是一个区域标题，在这里表示下方配置内容用来配置包（package）的

`package`下的配置项：

- `name`: 项目名
- `version`: 项目版本
- `authors`: 项目作者
- `edition`: 项目使用的Rust版本

### 2.3. 依赖配置

- `dependencies`: 该区域之下的内容，标识项目依赖的代码包或代码库，在 Rust 里，代码的包或库被称为 `crate`

### 2.4. 文件结构说明

Cargo生成的 `main.rs` 在 src 目录下，而 Cargo.toml 在项目顶层下，源代码都应该在 src 目录下，顶层目录可以放置：README、许可信息、配置文件、其他与程序源代码无关的文件。

如果在创建项目的时候没有使用 Cargo 来创建，也可以把项目转化为 Cargo 项目，操作如下：

- 把源代码文件移到 `src` 目录下
- 创建 Cargo.toml 并填写相应的配置

## 3. 构建项目

### 3.1. 编译代码

使用以下命令编译代码，生成二进制可执行文件

```shell-script
cargo build
```

如果是在Mac/Linux操作系统中，生成的二进制可执行文件路径如下

```shell-script
target/debug/hello_cargo
```

如果是在windows中，生成的二进制可执行文件路径如下

```shell-script
target/debug/hello_cargo.exe
```

第一次运行`cargo build`会在顶层目录生成 `cargo.lock` 文件，该文件负责追踪项目以来的确切版本，不需要手动去修改该文件

### 3.2. 编译并执行代码

我们还可以在编译代码后直接执行程序，使用如下命令

```shell
cargo run 
```

实际上，执行 `cargo run` 命令后，cargo 先编译程序生成可执行文件，再执行程序，如果之前编程成功过，并且源代码没有修改的情况下，那么就会直接运行二进制可执行文件。

### 3.3. 检查代码

使用以下命令可以检查代码，确保编译通过，但不会产生任何可执行文件

```shell
cargo check
```

实际上，`cargo check`要比`cargo build`执行快的多，编写代码的时候开发者可以连续反复的使用`cargo check`检查代码，确保自己的代码写的没有问题，最后需要执行的时候再使用 `cargo build` 生成二进制可执行文件，提高开发效率。

### 3.4. 构建发行版程序

实际上，在上面执行 `cargo build` 命令生成了一个用于开发调试使用的二进制可执行文件，当我们真正需要发版的时候，需要 加上 `--release` 参数进行编译，如下命令

```shell
cargo build --release
```

加上 `--release` 参数后，编译时代码会进行优化，编译的时候变长，但生成的二进制可执行文件执行效率也更高。生成的二进制可执行文件将保存在 `target/release` 目录中，而不是 `target/debug` 目录中。

最后得出以下结论：

- 在项目开发中，代码编译时使用 `cargo build` 命令

- 在项目发布时，代码编译使用 `cargo build --release` 命令
