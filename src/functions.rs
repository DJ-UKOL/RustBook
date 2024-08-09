pub fn functions() {
    println!("This is a main function.");
    another_function();
    function_with_arguments(5);
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is: {x}");
    let y = plus_one(5);
    println!("The value of y is: {y}");
}

fn another_function() {
    println!("This is an another function.");
}

fn function_with_arguments(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(y: i32) -> i32 {
    y + 1
}