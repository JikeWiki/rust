# 多线程同时运行代码

## 一、概述

### 1.1 并发

Concurrent: 程序不同的部分之间独立执行
Parallet（并行、并发）: 程序不同部分同时运行

Rust允许我们编写没有细微Bug的代码，并在不引入新bug的情况下易于重构。本文中的“并发”泛指concurrent和parallet。

### 1.2 进程与线程

在大部分OS里，代码运行在进程（process）中，OS同时管理多个进程。

在你的程序里，各个独立的部分可以同时运行，运行这些独立的部分就是线程（thread）。由于多个线程是可以同时运行的，所以我们通常把程序的计算拆分为多个线程同时来运行。这样的好处在于提升程序的性能表现，但也增加了复杂性，无法保证各程序的执行顺序，因为我们无法保证各个线程之间的顺序。

### 1.3 多线程可导致的问题

- 竞争状态，线程以不一致的顺序访问数据或资源
- 死锁，两个线程彼此等待对方使用完持有的资源，导致线程无法继续
- 出现只在某些情况下发生的bug，而且很难k可靠地复现和修复

### 1.4 实现线程的方式

第一种，通过调用OS的API来创建线程，这就是所谓的1:1模型，即一个操作的线程对应一个编程语言的线程，优点是需要较小的运行时。

第二种，编程语言实现自己的线程（绿色线程），这就是所谓的M:N模型，即M个绿色线程对应N个系统线程，但是需要比较大的运行时。

Rust标准库仅提供1:1的线程支持，但是由于rust拥有良好的底层抽象能力，所以在社区里也涌现出了很多M:N线程模型的第三方包。

## 二、多线程的使用

### 2.5 使用spawn创建新线程

rust通过`thread::spawn`函数可以创建新线程，参数为一个闭包，在新线程将运行闭包的代码。如下示例

```rust
use std::thread::{self, Thread};
use std::time::Duration;

fn main() {
    thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawnd thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

我们将在程序中看到交替打印主线程和子线程的输出内容，不过当主线运行程结束后，整个程序运行结束。但实际上我们希望子线程能完整地执行所有逻辑。

### 2.2 通过join handle来等待所有线程完成

`thread::spawn`函数的返回值类型是`JoinHandle`，`JoinHandle`持有值的所有权，通过调用其`join`方法，可以等待对应的其他线程的完成。调用`JoinHandle`的`join`方法会阻止当前运行线程的执行，直到`handle`所表示的这些线程结束。我门可以修改"2.1"中的代码如下，以保证子线程能完整地执行所有逻辑

```rust
use std::thread::{self, Thread};
use std::time::Duration;

fn main() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawnd thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

### 2.3 使用move闭包

move闭包通常和`thread::spawn`函数一起使用，它允许你使用其他线程的数据。也就是说，在创建线程的时候，把值的所有权从一个线程转移到另一个线程。我们先来看一个错误的示例代码

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v);

    handle.join().unwrap();
}
```

以上代码之所以会报错，是因为子线程借用了v，但是v的生命周期可能会比子线程短。所以rust不允许编译通过，此时我们只需要加上`move`关键字转移v的所有权到子线程，但是加了`move`关键字之后在主线程中不能再调用`drop(v)`。如下示例代码

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```
