pub fn iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();        // Создаем итератор и сохраняем в переменной

    for val in v1_iter {                // нет необходимости делать мутабельной переменную,
                                              // т.к. for делает это неявно
        println!("Got: {val}");
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();    // Необходимо сделать мутабельную переменную,
                                             // т.к. каждый вызов next() потребляет элемент из итератора

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // после использования sum мы не можем использовать итератор

    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x+1)// map возвращает новый итератор.
        .collect();                         // потребляет итератор и собирает значения в коллекцию (вектор)
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() // into_iter создает итератор, который становится владельцем вектора
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: "sneaker".to_string(),
                },
                Shoe {
                    size: 10,
                    style: "boot".to_string(),
                }
            ]
        )
    }
}