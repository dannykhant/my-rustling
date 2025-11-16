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
