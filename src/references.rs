pub fn references() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);    // передаем ссылку на переменную s1, s1 продолжает действовать

    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("Hello"); // изменяемая переменная
    change(&mut s2);
    println!("{s2}");

    let r1 = &mut s2;
    println!("{r1}");
    let r2 = &mut s2;
    println!("{r2}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}