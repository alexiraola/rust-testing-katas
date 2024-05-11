pub fn fizzbuzz(number: i32) -> String {
    if number % 3 == 0 && number % 5 == 0 {
        return "fizzbuzz".to_string();
    }
    if number % 3 == 0 {
        return "fizz".to_string();
    }
    if number % 5 == 0 {
        return "buzz".to_string();
    }
    number.to_string()
}
