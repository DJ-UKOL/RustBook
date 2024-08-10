// Висячая ссылка
pub fn dangling() {
    let references_do_nothing = dangle();
    println!("{references_do_nothing}");
}

// Этот код не будет работать
/*fn dangle() -> &String {            // Нельзя возвращать ссылку на String потому что
    let s = String::from("Hello");      // время жизни переменной s прекращается
    &s
}*/ // в этом месте

fn dangle() -> String {
    let s = String::from("Hello");
    s
}