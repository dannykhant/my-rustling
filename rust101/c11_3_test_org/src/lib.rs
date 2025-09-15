fn internal_add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    internal_add(a, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = internal_add(2, 2);
        assert_eq!(result, 4);
    }
}
