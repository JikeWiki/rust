# Cargo工作空间

## 一、概述

cargo工作空间可以帮助管理多个相互关联且需要协同开发的crate。实际上，cargo工作空间是一套共享同一个Cargo.lock和输出文件夹的包。

## 二、创建工作空间

有多种方式创建工作空间，我们下面先创建一个二进制crate，再创建2个库crate。二进制crate包含main函数，依赖于其他2个库crate，其中1个库crate提供add_one函数，另一个库crate提供add_two函数。下面是具体的步骤

先创建一个`add`目录，在该目录下创建一个`Cargo.toml`文件，该文件用于配置整个包工作空间，如下内容

```ini
[workspace]

members = [
    "adder"
]
```

上面的代码中，包含`wokspace`节点，在该节点下的`members`字段表示工作空间的成员。现在我们使用`cargo new adder`在工作空间下创建出`adder` crate。

现在就创建出了一个工作空间，并且该空间内包含一个crate。在工作空间里使用`cargo build`命令将会对整个工作空间进行编译，编译后在`target`产生编译产物。即使我们切换到具体的crate目录执行`cargo build`，编译产物也依然会生成在工作空间下的`target`目录。通过共享一个target目录，不同的项目就可以避免不必要的重复编译过程。

## 三、工作空间内的二进制crate依赖库crate

现在我们在工作空间下的`Cargo.toml`文件添加`add-one`项目，该项目是个库crate，所以使用`cargo new add-one --lib`创建出来。我门修改`add-one`项目的`src/lib.rs`代码，修改为如下内容

```Rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

接下来我们让`adder`项目依赖`add-one`这个项目，在`adder`项目中修改`Cargo.toml`文件，追加以下内容即可

```ini
add-one = { path = "../add-one" }
```

最后修改`adder`项目的`src/main.rs`源代码，如下

```Rust
use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    )
}
```

这时候我们要运行`adder`这个项目，需要在工作空间内使用`-p`参数指定项目名即可，如下命令

```bash
cargo run -p adder
```

## 四、在工作空间中依赖外部的crate

工作空间只有一个Cargo.lock文件，在工作空间的顶层目录。这个文件的配置内容保证里工作空间内所有的crate使用的依赖的版本都相同。下面我们做一个测试

我们先在`add-one`项目的`Cargo.toml`添加以下定义

```ini
rand = "0.3.14"
```

我们在`adder`项目的`Cargo.toml`添加以下定义

```ini
rand = "0.3.15"
```

但是我们在工作空间执行`cargo build`之后，通过分析`Cargo.lock`文件，发现两个crate使用的是同一个版本，并覆盖了`Cargo.toml`中定义的版本。这样工作空间内所有的crate相互兼容。

我们再通过`cargo new add-two --lib`创建add-two项目，并在工作空间里引入，如果这时候我门直接在add-two项目里使用`rand` crate是不行的，编译会报错，需要在其`Cargo.toml`先声明才可以。

## 五、为工作空间添加测试

当我们在工作空间运行`cargo test`时，cargo会一次性运行工作空间里的所有crate的测试代码，包括文档注释的测试。如果只需要测试某个crate，使用`-p`参数指定具体crate即可，如下示例命令

```bash
cargo test -p add-one
```

对于发布，没有提供一次性发布工作空间所有crate的功能，如果需要发布，我门必须手动切换到具体的crate目录，再执行`cargo publish`。
