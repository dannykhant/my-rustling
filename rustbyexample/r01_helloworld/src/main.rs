#![allow(dead_code)]
#![allow(unused_variables)]

mod formatted_print;
mod debug;
mod display;
mod testcase_list;
mod formatting;

fn main() {
    println!("Hello, world!");

    formatted_print::formatted_print();
    debug::debug();
    display::display();

    let v = testcase_list::List(vec![1, 2, 3]);
    println!("{}", v);

    formatting::formatting();
}
