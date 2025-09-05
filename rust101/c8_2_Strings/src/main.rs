fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{s}");

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    let s4 = String::from("hello");
    let s5 = String::from("world");
    let s0 = format!("{s4}-{s5}-{s2}");

    for c in "กา".chars() {
        println!("{c}");
    }

    for b in "กา".bytes() {
        println!("{b}");
    }
}
