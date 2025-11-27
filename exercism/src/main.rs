#![allow(dead_code)]
mod exercises;

fn main() {
    let hs = exercises::HighScores::new(&[30, 50, 20, 70, 40]);
    println!("{:?}", hs.personal_top_three());
}
