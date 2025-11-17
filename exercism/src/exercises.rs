// Your task is to reverse a given string.
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
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
