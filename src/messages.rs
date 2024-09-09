use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn messages() {
    let (tx, rx)
        = mpsc::channel();  // создаем новый канал (mpsc - multiple producer, single consumer - несколько производителей, один потребитель)

    let tx1 = tx.clone();

    thread::spawn(move || {         // используем move для перемещения tx в замыкание, чтобы порожденный поток владел tx
       let vals = vec![
           "hi".to_string(),
           "from".to_string(),
           "the".to_string(),
           "thread".to_string(),
       ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
       let vals = vec![
           "more".to_string(),
           "message".to_string(),
           "for".to_string(),
           "you".to_string(),
       ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }


}