pub fn strings() {
    // Создание новой строки
    let mut s = String::new();

    let data = "initial contents";
    let sd = data.to_string();

    // метод также работает с литералом напрямую:
    let str_data = "initial content".to_string();
    let str_from = String::from("initial content");

    // Присоединение к строке
    let mut sadd = String::from("foo");
    s.push_str("bar");
    let s1 = " go";
    sadd.push_str(s1);
    println!("{s1}");

    let mut s2 = String::from("lo");
    s2.push('l');
    println!("{s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Примечание s1 был перемещен сюда и больше не может быть использован
    println!("{s3}");

    // Объединение строк с помощью format!()
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}