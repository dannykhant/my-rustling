#![allow(unused)]

use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // unwrap
    // let greet_file = File::open("hello.txt").unwrap();

    // expect
    // let greet_file = File::open("hello.txt").expect("hello.txt should be included...");


    let greet_file = File::open("hello.txt")?;
    Ok(())
}

fn read_user_from_file() -> Result<String, io::Error> {
    let user_file_res = File::open("hello.txt");

    let mut user_file = match user_file_res {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut user = String::new();

    match user_file.read_to_string(&mut user) {
        Ok(_) => Ok(user),
        Err(e) => Err(e)
    }
}

fn read_user_from_file_v2() -> Result<String, io::Error> {
    let mut user = String::new();

    File::open("hello.txt")?.read_to_string(&mut user)?;
    Ok(user)
}

fn read_user_from_file_v3() -> Result<String, io::Error> {
    fs::read_to_string("hello.text")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}