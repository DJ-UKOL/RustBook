use std::ops::Deref;

// Определяем собственный умный указатель, похожий на Box<T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// реализация типажа deref для MyBox, чтобы знать как разыменовывать * ссылку
impl<T> Deref for MyBox<T> {
    type Target = T;        // определяет связанный тип для использования у типажа Deref

    fn deref(&self) -> &Self::Target {
        &self.0         // возвращает ссылку, получает доступ к первому значению в кортежной структуре
    }
}

pub fn smarts() {
    let x = 5;
    let y = &x;    // ссылка на x на значение 5
    let z = Box::new(x);    // скопированное значение x, а не ссылка, экземпляр Box<T>
    let z1 = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);      // чтобы пререйти от ссылки к значению, используется разыменование *y
    assert_eq!(5, *z);      // чтобы пререйти от Box<T> к значению, используется разыменование *z
    assert_eq!(5, *z1);     // // вызывает *(z1.deref())

    // вызов hello со ссылкой на значение типа MyBox<String>
    let m = MyBox::new(String::from("Rust"));
    hello(&m);  // &m является ссылкой на значение MyBox<String>
    // если бы небыло реализации типажа deref
    hello(&(*m)[..]);
}

// Неявные разыменованные приведения с функциями и методами
fn hello(name: &str) {
    println!("Hello, {name}!");
}