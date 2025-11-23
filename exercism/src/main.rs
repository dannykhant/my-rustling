#![allow(dead_code)]
mod exercises;

fn main() {
    let proverbs = ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"];
    println!("{}", exercises::build_proverb(&proverbs));
}
