#![allow(dead_code)]
#![allow(unused_variables)]

mod formatted_print;
mod debug;

fn main() {
    println!("Hello, world!");

    formatted_print::formatted_print();
    debug::debug();
}
