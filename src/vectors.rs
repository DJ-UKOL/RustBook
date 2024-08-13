enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vectors() {
    let v:Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    // Изменение вектора
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // Чтение из вектора
    let v3 = vec![1, 2, 3, 4, 5];
    let third = &v3[2];
    println!("The third element is {third}");

    let third = v3.get(2);
    match third {
        None => println!("There is not third element"),
        Some(third) => println!("The third element is {third}")
    }

    // Перебор значений в векторе
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
        println!("{i}");
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text("blue".to_string()),
        SpreadSheetCell::Float(10.12),
    ];
}