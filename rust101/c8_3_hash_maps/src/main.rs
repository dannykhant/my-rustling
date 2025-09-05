use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Overwriting
    scores.insert(String::from("Blue"), 30);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Entry
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(100);

    println!("{scores:?}");

    let text = "hello world cold world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
