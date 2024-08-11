pub fn matches() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Функция, которая будет получать на вход неизвестную монету Соединённых Штатов и,
// подобно счётной машине, определяет, какая это монета, и возвращает её стоимость в центах
fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}