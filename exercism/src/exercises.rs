// Your task is to reverse a given string.
pub fn reverse(input: &str) -> String {
    use unicode_segmentation::UnicodeSegmentation;
    input.graphemes(true).rev().collect()
}


// Returns a DateTime one billion seconds after start.
use time::PrimitiveDateTime as DateTime;

pub fn after(start: DateTime) -> DateTime {
    
    use time::Duration;

    let end_odt = start.assume_utc() + Duration::seconds(1_000_000_000);
    
    DateTime::new(end_odt.date(), end_odt.time())
}


// An Armstrong number is a number 
// that is the sum of its own digits 
// each raised to the power of the number of digits.
pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    let len = digits.len() as u32;
    let output: u32 = digits.iter().map(|d| d.pow(len)).sum();
    num == output
}


// Recite the lyrics to that popular children's repetitive song: 
// Ten Green Bottles.
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    use std::fmt::Write;
    
    let dict = ["No", "One", "Two", "Three", "Four", "Five", 
                "Six", "Seven", "Eight", "Nine", "Ten"];
                    
    let mut bt = start_bottles;
    let mut msg = String::new();
    
    for _ in 1..=take_down {
        let current = dict[bt as usize];
        let curr_particle = if bt == 1 { "bottle" } else { "bottles" };
        
        let left = dict[(bt - 1) as usize].to_lowercase();
        let left_particle = if (bt - 1) == 1 { "bottle" } else { "bottles" };
        
        let _ = writeln!(&mut msg, 
            "{current} green {curr_particle} hanging on the wall,\n\
            {current} green {curr_particle} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {left} green {left_particle} hanging on the wall.\n");
            
        bt -= 1;
    }
    
    msg
}


// Find the difference between the square of the sum 
// and the sum of the squares of the first N natural numbers.
pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i * i;
    }
    total
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}


// Calculate the number of grains of wheat on a chessboard.
pub fn square(s: u32) -> u64 {
     2u64.pow(s - 1)
}

pub fn total() -> u64 {
    let mut total = 0u64;
    for i in 1..=64 {
        total += square(i);
    }
    total
}


// It is divisible by 4.
// If it is divisible by 100, it must also be divisible by 400
pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}


// Given a number n, determine what the nth prime is.
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, 
// we can see that the 6th prime is 13.
pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x))
            .nth(n as usize)
            .unwrap()
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    for i in (3..=n/2).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}


// Compute the prime factors of a given natural number.
pub fn factors(mut n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![];
    while n > 1 {
        for i in 2..=n {
            if n % i == 0 {
                primes.push(i);
                n /= i;
                break;
            }
        }
    }
    primes
}


// For want of a horseshoe nail, a kingdom was lost, or so the saying goes.
pub fn build_proverb(list: &[&str]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    
    for (i, &v) in list.iter().enumerate() {
        if i == list.len() - 1 {
            let _ = write!(&mut out, "And all for the want of a {}.", 
                list.first().copied().unwrap_or(""));
            break;
        }
        let _ = writeln!(&mut out, "For want of a {v} the {} was lost.", 
            list.get(i+1).copied().unwrap_or(""));
    }
    out
}