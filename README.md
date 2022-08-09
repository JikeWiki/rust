# Rust知识文档

## 一、Rust相关概念与简介

## 1. 为什么要使用Rust？

Rust是一种令人兴奋的新兴编程语言，它可以让每个人编写可靠且高效的软件。Rust可以用来替换C/C++，Rust和他们具有相同的性能，但是很多常见的bug在编译时就可以被消灭。Rust是一种通用的编程语言，但是它更善于以下场景：

- 需要运行时的速度
- 需要内存安全
- 更好地利用多核处理器

## 2. Rust与其他语言的比较

C/C++性能非常好，但类型系统和内存都不太安全。Java/C#拥有GC（内存回收器），能保证内存安全，也有很多优秀特性，但是性能相对C/C++不足。Rust则具有这两类语言的共同优点：

- 安全
- 无需GC
- 易于维护、调试，代码安全高效

## 3. Rust擅长的领域

- 高性能的 Web Service
- WebAssembly
- 命令行工具
- 嵌入式设备
- 系统编程

## 4. Rust与Firefox

Rust最初是 Mozilla 公司一个研究性项目。Firefox 是 Rust 产品应用的一个重要的例子。Mozzilla一直以来都在使用 Rust 创建一个名为Servo 的实验性浏览器引擎，其中所有内容都是并行执行的。目前 Servo 的部分功能已经被集成到 Firefox 里面了。Firefox原来的量子版就包含了 Servo 的CSS渲染引擎，Rust使得 Firefox 在这方面得到了巨大的性能改进。

## 5. Rust的用户案例

- Google： 新操作系统 Fuschia ，其中 Rust 代码量大约占30%

- Amazon： 基于Linux开发了可以在直接裸机、虚拟机上运行容器的操作系统

- System76: 使用纯Rust开发了下一代安全操作系统 Redox

- 蚂蚁金服： 库操作系统 Occlum

- 斯坦福和密歇根大学： 使用 Rust 开发嵌入式实时操作系统，应用于 Google 的加密产品

- 微软： 正在使用 Rust 重写 Windows 中的一些低级的系统组件

- 微软： WinRT/Rust 项目

- Drobox、Yelp、Coursera、LINE、Cloudflare、Alassian、npm、Ceph、百度、华为、Sentry、Deno...

## 6. Rust的优点

- 性能高

- 安全性高

- 无所畏惧的并发

## 8. 注意

Rust 有很多独有的概念，它们和现在大多数主流语言都不同，所以学习 Rust 的时候必须从基础概念一步一步地学

## 二、Rust知识目录

- [01-Rust的安装](./docs/01-install.md)

- [02-编写第一个Rust程序](./docs/02-helloword.md)

- [03-rust项目的构建工具](./docs/03-cargo.md)

- [04-猜数字游戏](./docs/04-guessing_game.md)

- [05-变量与可变性](./docs/05-variables.md)

- [06-数据类型](./docs/06-data_type.md)

- [07-函数](./docs/07-function.md)

- [08-流程控制](./docs/08-control.md)

- [09-栈内存与堆内存](./docs/09-stack_vs_heap.md)

- [10-所有权规则](./docs/10-ownership_rule.md)

- [11-所有权与函数](./docs/11-ownership_and_fuction.md)

- [12-引用与借用](./docs/12-reference.md)

- [13-切片](./docs/13-slice.md)

- [14-定义和实例化struct](./docs/14-struct.md)

- [15-定义和实例化struct](./docs/15-struct_sample.md)

- [16-struct的方法](./docs/16-struct_method.md)

- [17-定义枚举](./docs/17-enum_define.md)

- [18-option枚举](./docs/18-option_enum.md)

- [19-控制流运算符-match](./docs/19-match.md)

- [20-控制流运算符-if_let](./docs/20-if_let.md)
