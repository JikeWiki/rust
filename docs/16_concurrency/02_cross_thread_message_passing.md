# 跨线程消息传递

## 一、概述

消息传递是一种很流行且能保证安全并发的技术，在这种机制里线程（或Actor）通过彼此发送消息（数据）来进行通信。Go语言中有一句名言：“不要用共享内存来通信，要用通信来共享内存”，Go语言这种并发机制就体现了这个思想。

Rust也提供了一种基于消息传递的并发方式，在rust里使用标准库提供的`Channel`来实现。`Channel`包含发送端和接收端，我门可以通过调用发送端的方法来发送数据，接收端会检查和接收到达的数据。如果发送端和接收端的任意一端被丢弃了，那么`Channel`就关闭了。

## 二、使用Channel

### 2.1 在不同线程之间创建和接收数据

使用`mpsc::channel`函数来创建Channel，`mpsc`表示multiple producer, singer consumer（多个生产者、一个消费者），即有多个发送端，但只有一个接收端。调用该函数将返回一个元组，元组里的元素分别是发送端、接收端。如下示例代码

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

消费者的`recv`方法一直会阻塞当前线程，直到接收到消息为止。

### 2.2 发送端的send方法

该方法的参数为想要发送的数据，返回值为`Result<T, E>`，如果有问题（例如接收端已经被丢弃），将返回一个错误。

### 2.3 接收端的方法

`recv`方法阻止当前线程执行，直到Channel中有值被送来。一旦收到值，就会返回`Result<T>`，所有这个管道的所有发送端都关闭了，就会收到一个错误。

`try_recv`方法不会阻塞当前的线程，如果有数据到达，返回`OK`，里面包含着数据，否则返回错误。我们通常会使用循环来检查`try_recv`的结果，如果消息还没有来，我们也可以执行其他的操作。

### 2.4 channel和所有权转移

所有权先消息传递中非常重要，能帮你补全编写安全、并发的代码。我们先看以下的示例代码

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // 下面一行代码将会报错，因为所有权已经被转移
        println("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

在上面的示例代码中，借用了已移动的值，因此会发生编译错误。所以所有权机制会帮我我们编写编写安全、并发的代码。

### 2.5 发送多个值

我们通过发送多个值，就可以看到接收者在等待的过程。如下示例代码

```rust
use std::sync::mpsc;
use std::{thread, vec};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // 循环分别发送四个字符串
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // 我们把接收端当作迭代器来使用，这样就不需要显式调用recv方法
    for received in rx {
        println!("Got: {}", received);
    }
}
```

运行以上的代码，我们将看到接收端在等待消息的过程。

### 2.6 通过克隆创建多个发送者

通过调用`mpsc::Sender::clone`函数可以克隆发送者，如下示例代码

```rust
use std::sync::mpsc;
use std::{thread, vec};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread"),
        ];

        // 循环分别发送四个字符串
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // 循环分别发送四个字符串
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 我们把接收端当作迭代器来使用，这样就不需要显式调用recv方法
    for received in rx {
        println!("Got: {}", received);
    }
}
```

在以上的示例代码中，我们通过两个子线程由两个发送者来发数据。并在主线程中使用接收者接收数据，可以通过程序运行结果看到由两个发送者发送的数据被交替输出。
