// Программа, которая вычисляет площадь прямоугольника

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Имплементация (реализация) структуры
impl Rectangle {
    fn area(&self) -> u32 {     // self - экземпляр структуры
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // метод для создания квадрата
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn rectangles() {
    // First
    let width1 = 30;
    let height = 50;
    println!("The area of the rectangle 1 is {} square pixels.",
        area(30, 50)
    );
    // Second
    let rect2 = (40, 60);
    println!("The area of the rectangle 2 is {} square pixels.",
        area2(rect2)
    );
    // Third
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(50 * scale),
        height: 70
    };
    dbg!(&rect3);

    println!("The area of the rectangle 3 is {} square pixels.",
        area3(&rect3)
    );

    println!("{:#?}", rect3);

    // Fourth
    let rect4 = Rectangle {
        width: 60,
        height: 80,
    };

    println!("The area of the rectangle 4 is {} square pixels.",
        rect4.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Fifth
    let square1 = Rectangle::square(50);  // Создаем квадрат соо стороной 50
    println!("The area of the square 1 is {} square pixels.",
             square1.area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}