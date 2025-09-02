fn main() {
    // Ownership with passing values
    let s = String::from("hello");
    
    take_ownership(s);

    let x = 5;

    makes_copy(x);

    // Ownership with returning value
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{s1} {s3}");

    // Pass back to reuse the original value
    let s1 = String::from("hello");

    let (s2, len) = calc_length(s1);

    println!("length of {s2}: {len}");
}

fn take_ownership(a_string: String) {
    println!("{a_string}");
}

fn makes_copy(an_integer: i32) {
    println!("{an_integer}");
}

fn gives_ownership() -> String {
    let a_string = String::from("yours");
    a_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}