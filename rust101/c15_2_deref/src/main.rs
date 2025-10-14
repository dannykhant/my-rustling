fn hello(name: &str) {
    println!("hi {name}");
}

fn main() {
    let x: i32 = 3;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = Box::new(String::from("Rust"));
    hello(&m);
}
