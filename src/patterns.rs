use crate::patterns::Message::Quit;

pub fn patterns() {
    if_let();
    while_let();
    for_s();

    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    matching_literals();
    matching_named_variables();
    multiply();
    destructuring_structs();
    destructuring_enums();
    destructuring_nested_structs_and_enums();
    ignored();
    ignored_parts();
    ignoring_remaining_parts();
    extra_conditionals();
    bindings();
}

fn if_let() {
    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "32".parse();

    if let Some(color) = favourite_color {
        println!("Using your favourite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn for_s() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn matching_literals() {
    let x = 1;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Go 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

fn multiply() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),      // тоже что и 1|2|3|4|5
        _=> println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_structs() {

    let p = Point{x: 0, y: 7};

    let Point { x:a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p2 = Point{x: 2, y: 8};

    let Point {x, y} = p2;
    assert_eq!(2, x);
    assert_eq!(8, y);

    let p = Point { x: 0, y: 7 };
    
    match p {
        Point {x, y: 0} => println!("On the x axis at {x}"),    // если y = 0
        Point {x: 0, y} => println!("On the y axis at {y}"),    // если x = 0
        Point {x, y} => println!("On neither axis: ({x}, {y}"), // если ни x ни y не равны 0

    }
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructuring_enums() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn destructuring_nested_structs_and_enums() {
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}

fn ignored() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }

    foo(3, 4);
}

fn ignored_parts() {
    let _x = 5;     // неиспользуемая переменная _x, компилятор не будет предупреждать
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4 ,8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}
fn ignoring_remaining_parts() {
    let origin = Point3D{ x: 0, y: 0, z: 0 };
    
    match origin {
        Point3D{x, ..} => println!("x is {x}"),
    }

    let numbers = (2, 4, 6, 8, 10, 12);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

fn extra_conditionals() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;
    
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn bindings() {     // at @ позволяет создать переменную, которая содержит значение, одновременно с тем, как мы проверяем, соответствует ли это значение шаблон
    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello {id: 5};
    
    match msg {
        Message::Hello {
            id: id_variable @ 3..7
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id : 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found same other id: {id}"),
    }

}