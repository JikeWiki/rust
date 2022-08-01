# 编写第一个Rust程序

## 1. 文件命名规范

我们通常使用一个 英文名称的 空目录作为 `Rust` 项目的目录，并遵循以下规范：

- 程序文件名后缀为： `rs`
- 文件命名应当使用小写，如果名称包含多个单词，每个单词之间使用下划线隔开，如： `hello_world.rs`

## 2. 编写第一个Rust程序

### 2.1. 创建代码

创建一个名为`hello_world`的目录，在目录下创建`main.rs`文件，写入以下内容

```rust
fn main() {
    println!("Hello, world!");
}
```

以上内容就是 Rust 程序最简单的代码：

- 定义函数: `fn main(){}` 代表一个没有参数也没有返回值的名为`main`的函数

- `main`函数: 在 `Rust` 中， `mian` 函数是 `Rust` 可执行程序的入口

- 打印文本: `println!("Hello, world!")`，Rust代码的缩进是4个空格，而不是tab，`println!`是 Rust macro(宏)，如果是函数，就没有`!`，Rust代码以`;`结尾

以上代码的功能是: 在`mian` 函数中调用 `println!`宏 在控制台打印了一条字符串："Hello, world!"

### 2.2. 编译代码

使用以下命令进行编译

```rust
rustc main.rs
```

如果是在windows编译，生成的可执行文件名称为`main.exe`，使用以下命令执行

```bat
.\main.exe
```

如果是在mac/Linux中编译，生成的可执行文件名称为`main`，使用以下命令执行

```shell-script
./main
```

## 3. 总结

Rust 代码先编译成二进制可执行程序才能运行，编译命令为 `rustc 源代码文件名` ，如

```shell-script
rustc main.rs
```

- 编译成功后，会生成一个二进制可执行文件，在 Windows 上还会多出一个后缀为 `.pdb` 的文件，里面包含调试信息

- Rust 是 ahead-of-time 编译的语言，可以先编译程序，然后把可执行文件交给别人运行，无需安装 Rust

- `rustc` 只适合简单的 Rust 程序，如果源代码比较多，使用 `rustc` 并不合适，在后续将会介绍 `Cargo`，用于解决这个问题
