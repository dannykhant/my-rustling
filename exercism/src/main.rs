#![allow(dead_code)]
mod exercises;

fn main() {
    use time::macros::datetime;
    println!("{}", exercises::after(datetime!(2025-11-14 19:19:00)));
}
