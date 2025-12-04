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


// Convert a number to a string that contains raindrop sounds
pub fn raindrops(n: u32) -> String {
    let mut out = String::new();
    
    if n % 3 == 0 {
        out.push_str("Pling");
    }
    if n % 5 == 0 {
        out.push_str("Plang");
    }
    if n % 7 == 0 {
        out.push_str("Plong");
    }
    if out.is_empty() {
        out = n.to_string();
    }
    
    out
}


// The points awarded depend on two things:
// The level (a number) that the player completed.
// The base value of each magical item collected by the player during that level.
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    
    for &f in factors {
        if f == 0 {
            continue;
        }
    
        for m in (f..limit).step_by(f as usize) {
            set.insert(m);
        }
    }
    
    set.iter().sum()
}


// Bob is a lackadaisical teenager.
// In conversation, his responses are very limited.
pub fn reply(message: &str) -> &str {
    let message = message.trim();
    
    let apn: Vec<char> = message.chars().filter(|c| c.is_alphabetic()).collect();
    let is_shouting = !apn.is_empty() && apn.iter().all(|c| c.is_uppercase());
    let is_question = message.ends_with('?');
    let is_silence = message.chars().all(|c| c.is_whitespace());
    
    match message {
        _ if is_question & is_shouting => "Calm down, I know what I'm doing!",
        
        _ if is_silence => "Fine. Be that way!",
            
        _ if is_shouting => "Whoa, chill out!",
            
        _ if is_question => "Sure.",
        
        _ => "Whatever."
    }
}


// Manage a game player's High Score list.
#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top = self.scores.to_vec();
        top.sort_unstable_by(|a, b| b.cmp(a));
        top.truncate(3);
        top
    }
}


// Determine if a string has balanced brackets.
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut s = Vec::new();
    
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => s.push(c),
            ')' => {
                if Some('(') != s.pop() { return false; }
            }
            ']' => {
                if Some('[') != s.pop() { return false; }
            }
            '}' => {
                if Some('{') != s.pop() { return false; }
            }
            _ => {}
        }
    }
    s.is_empty()
}


// Collatz Conjecture, a puzzle that has baffled thinkers for decades.
pub fn collatz(mut n: u64) -> Option<u64> {
    if n < 1 { return None; }
    let mut cnt = 0;
    
    while n > 1 {
        match n % 2 {
            0 => n /= 2,
            _ => n = (n * 3) + 1
        }
        cnt += 1;
    }
    
    Some(cnt)
}


// Given a string of digits, return all the possible consecutive number series
// of a specified length in that string.
pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let digits_len = digits.len();
    
    if len == 0 || digits_len < len { 
        return v; 
    }
    
    for i in 0..=digits_len - len {
        v.push(digits[i..i + len].to_string());
    }
    
    v
}


// In a kindergarten class, each student is assigned a set of plants.
// The plants are represented by letters.
// Each student has a specific position in the class roster.
// Given the diagram of plants and a student's name,
// return the list of plants assigned to that student.
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let idx = student.chars()
                    .next()
                    .map(|c| (c.to_ascii_uppercase() as usize - 65) * 2)
                    .unwrap_or(0);
                    
    diagram.lines()
        .flat_map(|l| { 
            l.as_bytes()[idx..idx + 2].iter()
                .map(|&c| match c {
                        b'G' => "grass",
                        b'C' => "clover",
                        b'R' => "radishes",
                        b'V' => "violets",
                        _ => panic!("unknown...")
                })
        }).collect()
}


// Calculate the number of eggs in a given display value.
pub fn egg_count(display_value: u32) -> usize {
    let mut cnt = 0;
    let mut n = display_value;
    
    while n > 0 {
        cnt += n & 1;
        n >>= 1;
    }
    
    cnt as usize
}
