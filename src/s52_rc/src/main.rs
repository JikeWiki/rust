use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("counter after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("counter after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(3, Rc::clone(&a));
        println!("counter after creating c = {}", Rc::strong_count(&a));
    }

    println!(
        "counter after c goes out of scopes = {}",
        Rc::strong_count(&a)
    );
}
