fn prints_and_return_10(a: i32) -> i32 {
    println!("I got {a}");
    10
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_will_pass() {
        let value = prints_and_return_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore] // for expensive tasks
    fn this_will_fail() {
        let value = prints_and_return_10(8);
        assert_eq!(value, 5);
    }

    #[test]
    fn add_two_and_two() {
        let value = add_two(2);
        assert_eq!(value, 4);
    }
}
