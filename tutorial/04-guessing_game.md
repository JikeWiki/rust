# 猜数游戏

## 1. 概述

使用参数游戏的程序，来学习一下内容：

- let、match等的方法
- 相关的函数
- 外部crate


游戏目标：
- 生成1-100之间的随机数
- 提示玩家做一个猜测
- 猜测完成之后，程序会提示太大或者太小
- 如果猜测正确，那么打印一个庆祝信息，并退出程序


## 2. 实战过程

新建名为 `guessing_game` 的项目，如下命令

```shell
cargo new guessing_game
```

我们在`main.rs`文件中修改为以下内容

```rust
use std::io;

fn main() {
    println!("猜数!");

    println!("猜一个数字");

    // 使用let声明变量
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜的数字是: {}", guess);
}
```


相关知识点：

**use**

我们在上面的第一行代码中使用了`use`关键字，该关键字是用于导入库。在main函数中我们使用到了`read_line`函数用于读取终端输入，该函数在`io`库中，而`io`库属于rust的标准库，所以我们使用了`use std::io;`进行引入。


> rust会把一个叫做`prelude`的库导入到程序中，包含了一些基本的函数。如果我们使用的函数不在`prelude`中，那么我们则需要显式地导入到程序中，导入的关键字就是以上代码中的`use`关键字。

**let**

在rust中使用`let`关键字声明变量，但是变量默认不可变，属于`immutable`状态，如果我们在声明变量的时候希望能修改它，那么还需加上`mut`关键字


**关联函数**

在以上示例代码中，包含以下的一行

```rust
let mut guess = String::new();
```

String是一个类，`::`关键符号用于调用关联函数`new()`，这样就生成了一个空的字符串。rust里的关联函数类似于`Java`的中静态方法。在Rust里，很多地方会有new函数名，因为在rust里，rust是初始化对象的惯例函数名。

**取地址符号**

在上面的`io::stdin().read_line(&mut guess)`这部分，read_line函数需要传入了一个可变的变量，并且使用`&`符号，代表传入了这个变量的引用，指向变量的原始内存地址。把引用传到函数里面，让其在函数里也能修改在函数外面的定义值。引用在rust里也是默认不可变的，需要添加`mut`让变量可变。


`read_line`这个方法会返回一个`io::Result`的结果，在rust的标准库里都有一个叫`Result`的类型，有泛型`Result`，也有子模块`Result`，如上面`read_line`方法返回的`io::Result`。这里的`Result`是一个枚举(enum)类型，一个枚举类型通常有几个固定的值，这些值称作这个枚举类型的变体，`io::Result`这个枚举类型一共有两个变体，一个是`OK`，另一个是`Err`





