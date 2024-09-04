struct CustomSmartPointer {
    data: String,
}

// печатает Dropping CustomSmartPointer!,
// когда экземпляр выходит из области видимости
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

pub fn drops() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);        // Раннее удаление значения с помощью std::mem::drop
    println!("CustomSmartPointer dropped before the end of main.");
    // переменные удаляются в обратном порядке их создания, сначала d потом c
}