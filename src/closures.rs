use std::thread;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Доступные цвета
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

// Запасы компании (Инвентаризация)
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor { // user_preference - необязательный параметр, выбор цвета пользователем
        user_preference.unwrap_or_else(|| self.most_stocked())  // возвращает цвет который получит пользователь
        // если пользователь ввел цвет Some(), то возвращаем этот цвет
        // если не ввел None(), то вызываем замыкание
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn closures() {
    // Магазин
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],  // две синии, одна красная рубашка
    };

    let user_pref1 = Some(ShirtColor::Red);     // пользователь предпочитает красную рубашку, тип Option
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;  // пользователю без разницы цвет
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    capturing();


    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12},
        Rectangle {width: 6, height: 7},
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    // Подсчет количества вызовов sort_by_key() при сортировке
    // let mut sort_operation = vec![];
    let value = String::from("closure called");

   /* list.sort_by_key(|r| {
        sort_operation.push(value); // Не сработает, т.к. замыкание реализует fnOnce, а надо fnMut
        r.width
    });
    println!("{list:#?}");*/

    let mut num_sort_operation = 0;
    list.sort_by_key(|r| {
        num_sort_operation += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operation} operations");

}

// Время от времени наша компания по производству футболок в качестве акции дарит эксклюзивные футболки,
// выпущенные ограниченным тиражом, каким-нибудь пользователям из нашего списка рассылки.
// Люди из списка рассылки при желании могут выбрать любимый цвет в своём профиле.
// Если человек, выбранный для получения бесплатной футболки, указал свой любимый цвет,
// он получает футболку этого цвета. Если человек не указал свой любимый цвет,
// он получит рубашку того цвета, которых у компании на данный момент больше всего.

// Захват ссылок или передача владения
fn capturing() {
    println!("\nCapturing References or Moving Ownership");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // замыкание, которое захватывает неизменяемую ссылку на вектор с именем list
    let only_borrows = || println!("From closure: {list:?}");   // переменной привязываем замыкание

    println!("Before calling closure: {list:?}");
    only_borrows(); // вызываем замыкание, используя имя перменной и скобки
    println!("After calling closure: {list:?}");

    println!("\nMutable:");
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {list2:?}");
    let mut borrows_mutably = || list2.push(7); // захватываем ссылку на list2, больше она не используется.
    borrows_mutably();  // теперь ссылку на list2 можно использвовать
    println!("After calling closure: {list2:?}");

    println!("\nUsing move");
    let list3 = vec![1, 2 ,3];
    println!("Before defining closure: {list3:?}");
    // Передача замыкания новому потоку
    thread::spawn(move || println!("From thread: {list3:?}"))   // порождаем новый поток
        // move преобразует любые переменные, захваченные по ссылке или изменяемой ссылке, в переменные, захваченные по значению.
        .join()
        .unwrap();
}