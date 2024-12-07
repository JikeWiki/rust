# 共享状态的并发

## 一、概述

“不要用共享内存来通信，要用通信来共享内存”。实际上，在上节中我们就是使用通信的方式来实现并发的，在本节我们要使用共享内存的方式来实现并发。

rust支持通过共享状态来实现并发。channel类似于单所有权，一旦将值的所有权转移至channel，就无法使用它了。而共享内存的并发方式类似多所有权，多个线程可以同时访问一块内存。

在rust里使用`Mutex`来每次只允许一个线程来访问数据，是`mutual exclusion`(互斥锁)的简写。在同一时刻，Mutex只允许一个线程来访问某些数据。想要访问数据，线程必须首先获取互斥锁（lock）。`lock`数据结构是mutex的一部分，它能跟踪谁对数据拥有独占访问权。mutex通常被描述为：通过锁定系统来保护它锁持有的数据。

## 二、Mutext的两条规则

- 在使用数据之前，必须尝试获取锁（lock）。
- 使用完mutex所保护的数据，必须对数据进行解锁，以便其他线程可以获取锁。

## 三、Mutex<T>的API

通过`Mutex::new(数据)`来创建`Mutex<T>`，`Mutex<T>`是一个智能指针，在访问数据之前，通过`lock`方法来获取锁。这个方法会阻塞当前线程的执行，返回的是`MutexGuard`（智能指针，实现了Deref和Drop），但`lock`方法可能会失败。如下示例代码

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // 获取数据的可变引用
        let mut num = m.lock().unwrap();
        // 变更数据
        *num = 6;
    }
    // 由于MutexGuard实现了 Drop trait，当作用域走完，m被自动解锁

    println("m = {"?"}", m);
}
```

## 四、多线程共享Mutex<T>

在rust里使用`Arc<T>`来进行原子引用计数，`Arc<T>`和`Rc<T>`类似，它可以用于并发场景。A是atomic的简称，即原子的。这时候你可以有一个问题，为什么所有的基础类型都不是原子的，为什么标准库类型不默认使用`Arc<T>`？因为需要付出性能代价。`Arc<T>`和`Rc<T>`的API是相同的。如下示例代码

```rust
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 创建10个线程去修改数据，再把返回的10个JoinHandle放到handles中
    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    // 等待所有线程运行完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

最终运行结果为`10`。

## 五、RefCell<T>/Rc<T>和Mutex<T>/Arc<T>

`Mutext<T>`提供了内部可变性，和`Cell`家族一样。我们可以使用`RefCell<T>`改变`Rc<T>`里面的内容，同样可以使用`Mutex<T>`来改变`Arc<T>`里面的内容。

但要注意的是，使用`Rc<T>`可能会造成循环引用，造成内存泄漏的风险；而使用`Mutex<T>`也有死锁的风险。所谓的死锁，就是当某个操作需要同时锁住两个资源，两个线程分别持有其中一个锁，并相互请求另外一个锁的时候，这两个线程就会陷入无穷无尽的等待。
