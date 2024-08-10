pub fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    loop_with_label();
    loop_with_while();
    loop_with_for();
}

fn loop_with_label() {
    println!("Loop with label");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn loop_with_while() {
    println!("Loop with while");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn loop_with_for() {
    println!("Loop with for");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    println!("While:");
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    println!("For:");
    for element in a {
        println!("The value is: {element}");
    }

    println!("For inversion:");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}