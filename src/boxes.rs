use crate::boxes::List::{Cons, Nil};

// cons список (cons list)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn boxes() {
    let b = Box::new(5);    // размещаем в куче, а не в стеке
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}