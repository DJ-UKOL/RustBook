use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};
use crate::rectangles::rectangles;

pub fn errors() {
/*    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => panic!("Problem opening the file: {err:?}")
        }
    };*/

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
       if error.kind() == ErrorKind::NotFound {
           File::create("hello.txt").unwrap_or_else(|error| {
               panic!("Problem creating the file: {:?}", error);
           })
       } else {
           panic!("Problem opening the file: {:?}", error);
       }
    });
}

// Проброс ошибок
// Эта функция читает имя пользователя из файла.
// Если файл не существует или не может быть прочтён,
// то функция возвращает ошибку в код, который вызвал данную функцию.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// сокращение для проброса ошибок: оператор ?
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Укороченный вариант
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Еще короче
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}