use std::collections::HashMap;

pub fn hash_maps() {
    // Создаем новую hashmap
    let mut scores = HashMap::new();
    // Добавляем элементы
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // Доступ к данным
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);       // copied - для получения Option<i32> из Option<&i32>
    println!("The score Blue's command is {score}.");
    // Перебор с помощью for
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("{:?}", map);
    println!("{field_name}, {field_value}");
    // Обновление данных
    // Перезапись старых значений
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");
    // Вставка значения только в том случае, когда ключ не имеет значения
    scores.entry(String::from("Yellow")).or_insert(35); // елси нет такого ключа, то вставить 35
    scores.entry(String::from("Green")).or_insert(90);
    println!("{scores:?}");
    // Создание нового значения на основе старого значения
    // Используем HashMap со словами в качестве ключей и увеличиваем соответствующее слову значение,
    // чтобы отслеживать, сколько раз мы встретили это слово.
    // Если мы впервые встретили слово, то сначала вставляем значение 0
    let text = "hello world wonderful world";
    let mut map_w = HashMap::new();

    for word in text.split_whitespace() {
        let count = map_w.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map_w:?}");

    // Упражнение 1
    let mut list_numbers = vec![43, 22, 64, 2, 13, 66, 34, 74, 34, 1, 66, 34, 64, 64, 13];
    exercise_1(&mut list_numbers);

    // Упражнение 2
    let str_for_2 = "first".to_string();
    pig_latin(&str_for_2);
    let str_for_3 = "apple".to_string();
    pig_latin(&str_for_3);

    // Упражнение 3
    add_staff();
}

// Есть список целых чисел. Создайте функцию, используйте вектор и верните из списка:
// среднее значение; медиану (значение элемента из середины списка после его сортировки);
// моду списка (mode of list, то значение которое встречается в списке наибольшее количество раз;
// HashMap будет полезна в данном случае).
fn exercise_1(list: &mut Vec<i32>) {
    // среднее значение
    println!("Average is {}", list.iter().sum::<i32>() as f64/list.len() as f64);
    // медиана
    list.sort();
    println!("{:?}", list);
    if list.len()%2 == 1 {
        let av = match list.get(list.len()/2) {
            None => 0,
            Some(num) => *num,
        };
        println!("Median is {}", av);
    }

    // Мод списка
    let mut map = HashMap::new();
    for num in list {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let mut max_value = map.values().max().unwrap();
    println!("Mod in list is: ");
    for (k, v) in map.iter() {
        if v == max_value {
            print!("{k}, ");
        }
    }
    println!();
}

// Преобразуйте строку в кодировку "поросячьей латыни" (Pig Latin).
// Первая согласная каждого слова перемещается в конец и к ней добавляется окончание "ay",
// так "first" станет "irst-fay".
// Слову, начинающемуся на гласную, в конец добавляется "hay" ("apple" становится "apple-hay")
fn pig_latin(string: &String) {
    if string.to_lowercase().starts_with(&['a', 'e', 'i', 'o', 'u', 'y']) {
        println!("{} - hay", &string);
    } else {
        if string.to_lowercase().starts_with(['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z']) {
            println!("{} - {}ay", &string[1..string.len()], &string[0..1]);
        }
    }
}

// Используя хеш-карту и векторы, создайте текстовый интерфейс позволяющий пользователю
// добавлять имена сотрудников к названию отдела компании.
// Например, "Add Sally to Engineering" или "Add Amir to Sales".
// Затем позвольте пользователю получить список всех людей из отдела или всех людей в компании,
// отсортированных по отделам в алфавитном порядке.
fn add_staff() {
    let mut staff_map = HashMap::new();

    loop {
        let str_cons = enter_console("Enter something: ".to_string());

        if str_cons.trim().to_lowercase().eq("exit") {
            break;
        }

        let words: Vec<&str> = str_cons.split_whitespace().collect();
        if (words.len() == 4) {
            if words[0].to_lowercase().eq("add") && words[2].to_lowercase().eq("to") {
                staff_map.insert(words[1].to_string(), words[3].to_string());
            } else { println!("Incorrect enter!"); }
        } else { println!("Incorrect enter!"); }

        println!("{:?}", staff_map);
    }

}

fn enter_console(string: String) -> String {
    println!("{string}");
    let mut buff = String::new();
        std::io::stdin().read_line(&mut buff).expect("Failed to read line");
    buff
}