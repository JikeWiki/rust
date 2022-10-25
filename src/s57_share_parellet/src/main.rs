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
