# 猜数游戏

## 一、概述

使用猜数游戏的程序，来学习以下内容：

- let、match等的方法
- 相关的函数
- 外部crate

游戏目标：

- 生成1-100之间的随机数
- 提示玩家做一个猜测
- 猜测完成之后，程序会提示太大或者太小
- 如果猜测正确，打印一个庆祝信息，并退出程序

## 二、实战过程

### 2.1 生成随机数

新建名为 `guessing_game` 的项目，如下命令

```shell
cargo new guessing_game
```

我们编辑`main.rs`文件，修改为以下内容

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

#### 2.1.1 use

我们在上面的第一行代码中使用了`use`关键字，该关键字是用于导入库。在main函数中我们使用到了`read_line`函数，该函数用于读取终端输入，该函数在`io`库中，而`io`库属于rust的标准库，所以我们使用了`use std::io;`进行引入。

> rust默认会把一个叫做`prelude`的库导入到程序中，包含了一些基本的函数。如果我们使用的函数不在`prelude`库中，那么我们需要显式地导入到程序中，导入包的关键字就是`use`。

#### 2.1.2 let

在rust中使用`let`关键字声明变量，但是变量默认不可变，属于`immutable`状态，如果我们在声明变量的时候希望能修改它，那么还需加上`mut`关键字

#### 2.1.3 关联函数

在以上示例代码中，包含以下的一行

```rust
let mut guess = String::new();
```

String是一个类，`::`符号用于调用关联函数`new()`，这样就生成了一个空的字符串。rust里的关联函数类似于`Java`的中静态方法。在Rust里，很多地方会出现`new()`函数，用于初始化对象。

#### 2.1.4 取地址符号

在上面的`io::stdin().read_line(&mut guess)`这部分代码中，`read_line`函数需要传入了一个可变的变量，并且我们在变量前面加了`&`符号，代表传入的是这个变量的引用，变量的饮用指向变量的原始内存地址。把引用传到`read_line`函数里面，让函数能修改指针地址所存储的值。引用在rust里也是默认不可变的，需要添加`mut`关键字让变量可变。

#### 2.1.5 io::Result

`read_line`这个方法会返回一个`io::Result`的结果，在rust的标准库里都有一个叫`Result`的类型，有泛型`Result`，也有子模块`Result`，如上面`io::Result`就属于`io`这个子模块的`Result`。

这里的`Result`是一个枚举(enum)类型，一个枚举类型通常有几个固定的值，这些值称作这个枚举类型的变体。`io::Result`这个枚举类型一共有两个变体，一个是`OK`，另一个是`Err`。`Result`这个类型通常定义了一些方法，比如我们在代码里调用的`expect`方法。

假如返回的`io.Result`值是`Err`，那么`expect`就会中断当前的程序，并将传入的字符串信息写出来；如果返回的值是`OK`，那么expect就会提取`OK`中附加的值，并将这个值作为结果返回给用户。

> 如果我们调用read_line之后没有调用expect，那么编译代码时rust将会提示相应的警告，因为返回的`io::Result`是一个未使用的变量，程序可能存在潜在性的错误。这是rust的安全机制引起的，也因此我们在编写代码的时候就可以避免很多错误。

#### 2.1.6 占位符

在以下的这个行代码中，我们使用到了`{}`，在rust的双引号内，是一个占位符，在编译的时候会替换成后面的变量。如果双引号中里有多个花括号，在编译时会替换成后面的多个变量值。

```rust
println!("你猜的数字是: {}", guess);
```

### 2.2 生成神秘数字

#### 2.2.1 下载依赖包

我们首先需要生成1到100之间的随机数，但是rust标准里并不包含生成随机数字的功能，不过官方提供了一个`crate`用于生成随机数，`crate`名称叫做`rand`。`crate`仓库的官方网站是：[https://crates.io/](https://crates.io/)，我们可以在crate官方网站中找到相关的`crate`。

在rust里， `crate`我们可以叫做“库”或者“包”，crate可以分为两种，一是我们构建好的二进制可运行文件，另一种是library，library的功能就是给其他程序使用。

我们在项目的`Cargo.toml`文件中引入`rand`库，如下

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.3.14"
```

在上面的配置文件中，我们在`dependencies`中新加了一行`rand = "0.3.14"`，左边代表库的名称，右边代表库的版本号，表示的是我们项目需要依赖这个库。当我们执行`cargo build`之后，cargo会去[crates.io](crates.io)下载我们定义的`rand`库到本地，同时`rand`库依赖的库也会被下载到本地。下载完成之后会执行编译，如果本地存在了对应的库，则直接执行编译。

`cargo`第一次执行`cargo build`命令的时候，会生成`Cargo.lock`文件，该文件保存了当前项目依赖的所有库以及对应的库版本，如果下次继续执行`build`指令，`cargo`将直接从`Cargo.lock`中直接读取依赖库信息，并加载到我们的项目中。

当我们希望`cargo`读取的是`Cargo.toml`文件获取版本信息，而不是`Cargo.lock`获取版本信息时，我们可以通过`cargo update`命令更新依赖，`cargo`会根据`Cargo.toml`提供的版本信息更新依赖,并且再次写入到`Cargo.lock`中。

#### 2.2.2 引入trait

trait类似于Java中的接口，但trait不是用于继承，而是引用，引用trait之后，可以使用trait定义的相关方法。下面我们引入`rand`库中的`Rng`，如下

```rust
use rand::Rng;
```

引入之后我们就可以使用生成随机数的函数了，我们先将生成的随机数打印出来，如下代码

```rust
let secret_number = rand::thread_rng().gen_range(1, 101);
println!("神秘数字是{}", secret_number);
```

在上面代码中，我们调用`rand::thread_rng()`生成了一个随机数生成器，再调用`gen_range()`方法实现了随机数的生成，这个方法的第一个参数是最小值闭区间，第二个参数是最大值开区间。

### 2.3 比较猜测的数字与神秘数字

我们需要比较`gues` 和 `secret_number`这两个变量，首先我们需要从标准库中引入如下`cmp::Ordering`，如下代码

```rust
use std::cmp::Ordering;
```

这里的`Ordering`是个枚举类型，包含三个值，如下

- Less: 小于
- Greater: 大于
- Equal: 等于

比较的代码如下

```rust
// 先将guess变量转换为整数类型
let guess: u32 = guess.trim().parse().expect("Please type a number")


match guess.cmp(&secret_number){
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => print!("To big!"),
    Ordering::Equal => println!("You win"),
}
```

在上面的代码中，我们使用一个同名的u32类型的变量`guess`，这样，在该行代码之上定义的String类型的`guess`变量就会被隐藏（shadow）。在该行代码以后，遇到的`guess`则是u32类型的`guess`。这个特性通常使用在我们需要类型转换的场景中，这样我们无需新起一个变量名，而是直接使用之前的变量名。

`trim`: trim函数会把字符串两边的空白内容去掉
`parse`: parse函数会把将字符串解析成u32类型

实际上，我们生成的随机数`secret_number`默认是i32类型，但是在往下的代码中，`secret_number`被用于与`guess`做对比，所以根据rust类型解析与推倒的机制，`secret_number`在编译的时候转变为了u32。

### 2.4 多次猜测

在以上的程序中，用户只能猜测一次数据是否正确，为了能实现多次猜测，我们需要做一个无线循环。我们吧才猜测的逻辑放在`loop`中，即可实现无限循环。

```rust
loop {
    println!("猜一个数字");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜的数字是: {}", guess);

    // 先将guess变量转换为整数类型
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => print!("To big!"),
        Ordering::Equal => println!("You win"),
    }
}
```

但上面的程序中，即使我们猜对了，也不会停止。我们应该将逻辑改为，让输入正确之后，我们应当跳出循环，改为如下代码

```rust
loop {
    println!("猜一个数字");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜的数字是: {}", guess);

    // 先将guess变量转换为整数类型
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => print!("To big!"),
        Ordering::Equal => {
            println!("You win");
            break;
        }
    }
}
```

### 2.5 完善程序

#### 2.5.1 完善健壮性

如果我们在在猜数的时候，输入的是一个非数字的字符，程序将被异常退出，我们在解析数字的时候使用`match`表达式，表达式里针对不同的枚举结果进行不同的处理，如下代码

```rust
let guess: u32 = match guess.trim().parse(){
    // 解析正确时将数字返回
    Ok(num) => num,
    // 解析错误时，使用 continue 跳出本次循环，进行下一次循环
    Err(_) => continue
};
```

#### 2.5.2 取消提示

还需注意的是，如果我们的目的是让用户玩这个游戏，那么就不应该把结果打印出来，所以去掉下面这行代码

```rust
println!("神秘数字是{}", secret_number);
```

## 三、运行程序

最终`main.rs`文件编写的程序代码如下

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("猜一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜的数字是: {}", guess);

        // 先将guess变量转换为整数类型
        let guess: u32 = match guess.trim().parse() {
            // 解析正确时将数字返回
            Ok(num) => num,
            // 解析错误时，使用 continue 跳出本次循环，进行下一次循环
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => print!("To big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
```

编译

```shell
cargo build
```

运行

```shell
target/debug/guessing_game
```
