#![allow(dead_code)]

use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::ch15_smart_pointers::main::List::{Cons, Nil};

pub fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // By using box we can define it recursively since its just pointers
    let _list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    // -------------------------------------------------

    let x1 = 5;
    let y1 = &x1;

    println!("Testing comparison between x and y.");
    assert_eq!(5, x1);
    assert_eq!(5, *y1);

    // -------------------------------------------------

    let x2 = 5;
    let y2 = Box::new(x2);

    println!("Testing comparison between x2 and y2.");
    assert_eq!(5, x2);
    assert_eq!(5, *y2);

    // -------------------------------------------------

    let x3 = 5;
    let y3 = MyBox::new(x3);

    println!("Testing comparison betwen x3 and y3.");
    assert_eq!(5, x3);
    assert_eq!(5, *y3);

    // -------------------------------------------------

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
