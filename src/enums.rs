use crate::enums::IpAddrKind::V4;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // пустой элемент без ассоциированных данных
    Move { x: i32, y: i32 },    // имеет именованные поля, как и структура
    Write(String),              // элемент с единственной строкой типа String
    ChangeColor(i32, i32, i32), // кортеж из трёх значений типа i32s
}

impl Message {
    fn call(&self) {
        println!("This is a call method from enum Message.");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr2::V4(127, 0, 0 , 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}