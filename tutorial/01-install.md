# Rust 的安装

Rust 的官方网址是: [https://www.rust-lang.org/](https://www.rust-lang.org/)


## 1. 安装 Rust


### 1.1. 脚本安装方式

使用电脑打开到 Rust 的官方网站之后，Rust 会判断我们当前的操作系统并推荐安装的方式，我们可以直接根据 Rust 官网首页的提示进行安装。


### 1.1. 二进制安装包安装

本节，我们介绍二进制文件的安装方式。


打开其他安装方式的页面：[https://forge.rust-lang.org/infra/other-installation-methods.html](https://forge.rust-lang.org/infra/other-installation-methods.html)，我们找到二进制安装包下载链接的页面位置，如下图所示


![01-01.png](./img/01-01.png)


> 在编写此文档的时候，Rust的最新稳定版本是`1.60.0`，你在打开的时候，页面可能有所变化。


找到与自己操作系统匹配的安装，点击下载进行安装即可。


### 1.2. 包管理工具安装

包管理工具可以快速安装Rust，如使用homebrew安装，如下命令

```shell
brew install rust
```


### 1.3. 安装验证

安装完成后，使用以下命令查看版本号，如果正常显示rust版本信息代表安装成功，否则重启终端或者检查其他原因

```shell
rustc --version
```


## 2. IDE开发


我们可以选择 `VsCode` 或者 `Intellij Idea` 进行代码开发时，可以在插件市场搜索 `Rust`，安装上 `Rust` 的官方插件，可以更好地使用 IDE 中的功能。




