fn fizzbuzz(number: i32) -> String {
    if number == 3 {
        return "fizz".to_string();
    }
    number.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_returns_string_number() {
        let result = fizzbuzz(1);
        assert_eq!(result, "1");
    }

    #[test]
    fn three_returns_fizz() {
        let result = fizzbuzz(3);
        assert_eq!(result, "fizz");
    }
}
