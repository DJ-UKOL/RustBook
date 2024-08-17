use std::fmt::{Debug, Display};
use std::slice::IterMut;

// Объявляем типаж с именем Summary
pub trait Summary {
    // Каждый тип реализующий данный типаж, должен предоставить собстенное поведение метода summarize
    // Что типа интерфейса в java
    fn summarize(&self) -> String {
        String::from("(Read more...)")  // реализация по умолчанию
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content:String,
}

// Реализация типажа Summary у структуры NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Реализация типажа Summary у структуры Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Типажи как параметры
// Синтаксический сахар
pub fn notify(item: &impl Summary) {
    println!("Braking new! {}", item.summarize());
}

// Ограничение типажа
// То же самое но более длинная запись
pub fn notify_2<T: Summary>(item: &T) {
    println!("Braking new! {}", item.summarize());
}

// Функция с двумя параметрами реализующие Summary (могут иметь разные типы)
pub fn notify_3(item: &impl Summary, item2: &impl Summary) {}

// Параметры с одинаковыми типами
pub fn notify_4<T: Summary>(item1: &T, item2: &T) {}

// Задание нескольких границ типажей с помощью синтаксиса +
pub fn notify_5(item: &(impl Summary + Display)) {}

// синтаксис с ограничениями типажа
pub fn notify_6<T: Summary + Display>(item: &T) {}

// Более ясные границы типажа с помощью where
// без where
pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// с where
pub fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{0}

// Возврат значений типа реализующего определённый типаж
pub fn returns_summarize() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
}

// Использование ограничений типажа для условной реализации методов
