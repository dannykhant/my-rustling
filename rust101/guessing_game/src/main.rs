use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number...");

    let secret_num = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_num}");

    loop {
        println!("Input your guess...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        }

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
