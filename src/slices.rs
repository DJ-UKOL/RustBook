pub fn slices() {
    let mut s = String::from("Hello World!");
    println!("{}", s);
    let word = first_word(&s);
    println!("{word}");

    let hello = &s[0..5];
    let world = &s[6..11];
}

// функция, которая принимает строку слов,
// разделённых пробелами, и возвращает первое слово,
// которое она находит в этой строке
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();   // преобразуем string в массив байтов

    // проходимся по массиву байт
    for(i, &item) in bytes  // (i, &item) - кортеж, i-индекс, &item-ссылка
        .iter()     // возвращает каждый элемент в коллекции
        .enumerate() {                  // оборачивает результат и возвращает кортеж
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}