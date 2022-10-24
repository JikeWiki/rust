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
