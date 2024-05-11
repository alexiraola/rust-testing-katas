use kata_fizzbuzz::fizzbuzz;

#[test]
fn three_returns_fizz() {
    let result = fizzbuzz(3);
    assert_eq!(result, "fizz");
}

#[test]
fn five_returns_buzz() {
    let result = fizzbuzz(5);
    assert_eq!(result, "buzz");
}

#[test]
fn fifteen_returns_fizzbuzz() {
    let result = fizzbuzz(15);
    assert_eq!(result, "fizzbuzz");
}

#[test]
fn divisible_by_three_returns_fizz() {
    let result = fizzbuzz(9);
    assert_eq!(result, "fizz");
}

#[test]
fn divisible_by_five_returns_buzz() {
    let result = fizzbuzz(10);
    assert_eq!(result, "buzz");
}

#[test]
fn divisible_by_three_and_five_returns_fizzbuzz() {
    let result = fizzbuzz(30);
    assert_eq!(result, "fizzbuzz");
}

#[test]
fn not_divisible_by_three_or_five_returns_number() {
    let result = fizzbuzz(23);
    assert_eq!(result, "23");
}
