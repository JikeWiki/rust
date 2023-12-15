# 发布crate

## 一、创建账号

发布crate.io前，需要在<crates.io>创建账号并获得API Token。运行命令`cargo login $API_TOKEN`，将会通知cargo，把你的API token存储在本地`~/.cargo/credentials`。API Token可以在<crates.io>上进行撤销。

## 二、为新的crate添加元数据

在发布crate之前，需要在`Cargo.toml`的`[package]`区域为crate添加一些元数据：

- name: crate需要指定位于的名称
- description: 一两句话即可，会出现在crate搜索的结果里
- license: 需要提供许可证的表示值，可以到<http://spdx.org/licenses/>查找，可以指定多个许可，使用`OR`隔开
- version: 版本号
- author: 作者

更多的原数据可以参考<https://doc.rust-lang.org/cargo/reference/manifest.html>，下面是一个示例的原数据

```ini
[package]
name = "s44_cargo"
version = "0.1.0"
author = ["kotlindev"]
edition = "2021"
description = "A Rust tutorials"
license = "MIT"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```

## 三、发布crate

最后，可以使用`cargo pushlish`进行发布。要注意的是，如果发布失败，原因可能是因为你的<crates.io>的账号还没绑定验证的邮箱，或者你的crate的元信息缺失，需要根据错误提示补充即可。

crate一旦发布，就是永久性的：该版本无法覆盖，代码无法删除。因为<crates.io>希望成为永久的代码托管服务器，并且依赖于该版本的项目可以继续正常工作。如果允许开发者删除已经发布的代码，那么将无法达到这个目的。

## 四、发布已存在的crate新版本

在修改crate的代码之后，当我们需要发布新版本时，需要先修改`Cargo.toml`里面的`version`值。可以参考语义化版本控制规范<http://semver.org>来定义你的语意版本。最后执行`cargo publish`进行发布。

## 五、使用cargo yank从crates.io撤回版本

我们不可以删除之前的版本，不会删除任何代码，但可以防止其他项目把某个版本最为新的依赖，使用yank可撤回一个crate版本即可。该操作防止新项目依赖于该版本，但已经存在的项目可以继续将其作为依赖并可以下载。

yank意味着：所有已经产生`Cargo.lock`的项目不会中断，同时任何将来新生成的`Cargo.lock`文件都不会使用被yank的版本。撤回命令如下示例

```bash
cargo yank --version 1.0.1
```

取消撤回如下命令

```bash
cargo yank --version 1.0.1 --undo
```
