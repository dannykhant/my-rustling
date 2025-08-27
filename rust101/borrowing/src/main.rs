fn main() {
    passing_reference();

    // Modifying borrowed value
    let mut s = String::from("hello");
    mut_change(&mut s);

    // Dangling Ref
    let _ref_to_sth = no_dangle();
}

fn passing_reference() {
    let s1 = String::from("hello");

    let len = calc_length(&s1);

    println!("length of {s1}: {len}");
}

fn calc_length(s: &String) -> usize {
    s.len()
}

// fn change(s: &String) {
//     // This won't work
//     s.push_str(", world");
// }

fn mut_change(str: &mut String) {
    str.push_str(", world");
    println!("{str}");
}

// fn dangle() -> &String {
//     // This won't work
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}