use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};
use crate::cycles::List::{Cons, Nil};

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

pub fn cycles() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));   // создаём экземпляр Rc<List>

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {      // получаем ссылку на RefCell<Rc<List>> из переменной a
        *link.borrow_mut() = Rc::clone(&b);     // используем метод borrow_mut из типа RefCell<Rc<List>>,
        // чтобы изменить внутреннее значение типа Rc<List>,
        // содержащего начальное значение Nil на значение типа Rc<List> взятое из переменной b
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing b = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    trees();
}


// дерево с узлами, которые знают о своих дочерних узлах
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,   // ссылки на его дочерние значения
}

fn trees() {
    println!("**********************");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() =
            Rc::downgrade(&branch);     // создать слабую ссылку на значение внутри экземпляра Rc<T>, получить слабый указатль Weak<T>

        println!("branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!("leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }


    println!("leaf parent = {:?}", leaf.parent.borrow()
        .upgrade());        // проверка, что значение существует (Some или None)
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}