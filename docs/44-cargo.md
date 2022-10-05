# cargo

## 一、通过release profile来自定义构建

release profile是预定义的，也是可定义的，可使用不同的配置，对编译代码拥有更多的控制。每个profile的配置都独立于其他的profile。在cargo里主要有两种profile，如下

- dev profile：适用于开发，cargo build
- release profile：适用于发布，cargo build --release

我们在编译的时候，如果执行的是`cargo build`命令，那么将使用dev profile；如果使用`cargo build --release`命令，将使用release profile。

## 二、自定义profile

针对每个profile，cargo都提供了默认的配置，如果想自定义某个xxx profie，我门可以在`Cargo.toml`里添加`[profile.xxx]`区域，在里面覆盖默认的子集。通常我门不会覆盖所有的配置，只需要覆盖想修改的配置即可。如果我门想针对`dev`和`release`的 profle进行修改，`Cargo.toml`文件示例内容如下

```ini
[package]
name = "s43_minigrep_optmize"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level`参数决定是否对编译进行优化，会影响编译时间。dev profile的默认值是0，编译没有优化，这样编译速度更快，但编译产物效率较低；release profile的默认值是3，这样编译时间更长，但是得到的产物运行效率更高。

对于每个配置的默认值和完整选项，可以参考<https://doc.rust-lang.org/stable/cargo/>

## 三、发布crate到crate.io

之前我门通过crates.io下载了第三方库，我门也可以通过crates.io发布包来共享我们的代码。crate的注册表在<https://crates.io>，它会分发已注册的包的源代码，主要托管开源的代码。

## 四、文档注释

### 4.1 生成文档注释

rust使用一种特殊的文档注释，用于生成文档，可以生成项目的HTML文档，将显示公共API的文档注释，表明如何使用API。使用`///`，而且在文档中可以使用Markdown语法，文档注释放在被说明条目之前。

使用`cargo doc`生成html文档，它会运行rustdoc工具，该工具在安装rust的时候就自带了。生成的html文档放在`target/doc`目录下。生成文档之后，我门可以使用`cargo doc --open`命令会直接在浏览器打开文档。

### 4.2 常用章节

在文档注释中，会经常使用一些章节标注，常用章节如下

`Example`: 示例代码
`panics`: 函数可能发生panic的场景
`Errors`: 如果函数返回Result，描述可能的错误种类，以及可导致错误的条件
`Safety`: 如果函数处于unsafe调用，就应该解释函数unsafe的原因，以及调用者应当确保使用的前提

### 4.3 文档注释作为测试

实际上，运行`cargo test`时，会把文档注释中的示例代码作为测试来运行，如下是的示例代码

```rust
/// Adds one to the number given.
/// 
/// # Examples
///
/// ```
/// let args = 5;
/// let answer = my_crate::add_one(args);
/// 
/// assert_eq!(6, anwser);
/// ```dotnetcli
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

运行`cargo test`之后，文档注释中的示例测试代码将会被执行。

### 4.4 为包含注释的项添加文档注释

符号: `//!`，这类注释通常用于描述crate和模块，如一个crate root（通常是src/lib.rs），另外是在一个模块内，将crate 或模块作为整体进行记录。如下的示例代码

```rust
//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain calculations more convenient

/// Adds one to the number given.
/// 
/// # Examples
///
/// ```
/// let args = 5;
/// let answer = my_crate::add_one(args);
/// 
/// assert_eq!(6, anwser);
/// ```dotnetcli
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

## 五、pub use

使用use可以到处方便使用的公共API，不过问题的在于，crate的程序结构在开发时对开发者很合理，但对于它的使用和不方便。开发者通常会把程序结构分为很多层，使用者想找到这种深层结构中的某个类型很费劲。如下的示例

```rust
my_crate::some_module::another_module::UsefulType;
```

上面的示例中存在多个层级，使用者很不方便对于用户来说，更为方便的应该是`my_crate::UsefulType`，但也有解决办法。我门不希要重新组织内部的代码结构，而是使用`pub use`，可以重新导出，创建与内部私有结构不同的对外公共结构。

下面我门看个示例，先创建`src/lib.rs`文件，写入如下的内容

```rust
//! # Art
//! 
//! A library for modeling artistic concept.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The second colors according to RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to crate
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) ->SecondaryColor {
        SecondaryColor::Green
    }
}
```

然后在`src/main.rs`调用，如下代码

```rust
use s44_cargo::kinds::PrimaryColor;
use s44_cargo::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

可以看到我们在引入函数的时候，层级结构比较深。我门下面做一个优化，使用`pub use`关键字在`src/lib.rs`里重新导出需要使用的内容到顶层结构，如下示例

```rust
//! # Art
//! 
//! A library for modeling artistic concept.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```

这时候我们使用`cargo doc --open`生成并查看文档，这时候文档已经表明了导出的部分，如下图

[44-01](./img/44-01.png)

这对于crate的用户来说，查找对应的类型和函数就非常方便了。这时候我们在使用的地方直接简化导入的层级，如下示例代码

```rust
// use s44_cargo::kinds::PrimaryColor;
// use s44_cargo::utils::mix;
use s44_cargo::PrimaryColor;
use s44_cargo::mix;
```
