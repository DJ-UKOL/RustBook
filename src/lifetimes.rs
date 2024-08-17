struct ImportantExcerpt<'a> {
    part: &'a str,  // Данная аннотация означает, что экземпляр ImportantExcerpt
                    // не может пережить ссылку, которую он содержит в своём поле part
}

// Валидация ссылок при помощи времён жизни
pub fn lifetimes() {
    let string = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // Определение времён жизни при объявлении структур
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();      // содержит ссылку на первое предложение
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

// Функция, которая возвращает более длинный из двух срезов строки.
// Эта функция принимает два среза строки и возвращает один срез строки.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {     // возвращаемая ссылка будет действительна до тех пор, пока валидны оба параметра
    if x.len() > y.len() {
        x
    } else {
        y
    }
}