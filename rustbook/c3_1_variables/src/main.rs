use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Input array idx");

    let mut idx = String::new();

    io::stdin()
        .read_line(&mut idx)
        .expect("error");

    let idx: usize = idx
        .trim()
        .parse()
        .expect("Expects number");

    let element = a[idx];

    println!("Value of element at idx {idx}: {element}");
}