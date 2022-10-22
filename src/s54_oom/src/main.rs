use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // 打印a的强引用
    println!("a initial rc count = {}", Rc::strong_count(&a));
    // 打印a的第二个元素
    println!("a text item = {:?}", a.tail());

    // b的第一个元素是10，第二个元素共享a的数据
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    // 打印a的强引用
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // 打印b的强引用
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // 打印b的第二个元素
    println!("b next item = {:?}", b.tail());

    // 取出a的第二个元素
    if let  Some(link) = a.tail() {
        // 把a原来存储的Nil改为B存储的值
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
}
