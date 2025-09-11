#![allow(unused)]

use std::fmt::Display;

fn main() {
    let s1 = String::from("hello");
    let result;
    {
        let s2 = String::from("xyz");

        result = longest(s1.as_str(), s2.as_str());
    }
    println!("Longest: {}", result);

    let novel = String::from("Awww. Some years ago...");
    let first_sentence = novel.split(".").next().unwrap();
    let i = ImportantExcerp {
        part: first_sentence
    };
    
    let s: &'static str = "I have static lifetime";
}

fn longest<'a>(x: &'a str,y: &str) -> &'a str {
    // if x.len() > y.len() { x } else { y }
    x
}

struct ImportantExcerp<'a> {
    part: &'a str
}

impl<'a> ImportantExcerp<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("{announcement}");
        self.part
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where 
    T: Display
{
    println!("{ann}");
    if x.len() > y.len() { x } else { y }
}