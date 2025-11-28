#![allow(dead_code)]
mod exercises;

fn main() {
    println!("{:?}", exercises::brackets_are_balanced("{[()}"));
}
