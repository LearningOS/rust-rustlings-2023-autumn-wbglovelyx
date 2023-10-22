pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        let even_number = 4;
        assert!(is_even(even_number));
    }

    #[test]
    fn is_false_when_odd() {
        let odd_number = 5;
        assert!(!is_even(odd_number));
    }
}
