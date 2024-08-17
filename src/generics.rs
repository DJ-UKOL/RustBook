pub fn generics() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The number largest is {}", largest_i32(&number_list));
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!("The number largest is {}", largest_i32(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest_char(&char_list));
    //println!("The number largest is {}", largest(&number_list));
}

// Функция, которая находит наибольшее число в списке
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Обобщенная функция
/*fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}*/