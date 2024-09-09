use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutexes() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // создаем 10 потоков
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
           let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // вызываем join для каждого дескриптора, чтобы убедиться в завершении всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
