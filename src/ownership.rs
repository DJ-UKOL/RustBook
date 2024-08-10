pub fn ownership() {

    let s = String::from("Hello");
    takes_ownership(s);     // s переходит в функцию и больше не действует

    let x = 5;

    makes_copy(x);      // переменная x передается как копия, и остается действовать здесь.

    // println!("{s}");   // ошибка, т.к. переменная уже не действует
    println!("{x}");        // ошибки нет, т.к. переменная x была скопированна.


    let s1 = gives_ownership();
    println!("{s1}");
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);  // s2 переходит в функцию и больше не действует
    // println!("{s2}");    // ошибка, т.к. переменная уже не действует
    println!("{s3}");

    let k1 = String::from("Hello");
    let (k2, len) = calculate_length(k1);
    println!("The length of '{k2}' is {len}.");

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}