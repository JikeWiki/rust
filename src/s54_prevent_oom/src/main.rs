use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
// 定义树的节点
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    // 使用Rc<Node>为了让所有的子节点能够共享所有权
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // 创建一个叶子
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // 打印leaf
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // 一个作用域
    {
        // 创建一个树枝
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // 让树叶的parent指向树枝
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // 打印branch
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        // 打印leaf
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    // 使用leaf访问branch
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // 打印leaf
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

}
