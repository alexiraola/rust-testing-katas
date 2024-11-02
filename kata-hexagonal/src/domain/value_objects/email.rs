use core::panic;
use std::fmt;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Email {
    email: String,
}

impl Email {
    pub fn create(address: String) -> Self {
        Self::ensure_is_valid_email(&address);
        Self { email: address }
    }

    fn ensure_is_valid_email(address: &str) {
        let email_regex = Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(address) {
            panic!("Invalid email format");
        }
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email
    }
}
impl Eq for Email {}

#[cfg(test)]
mod test {
    use crate::domain::value_objects::email::Email;

    #[test]
    fn create_email_with_correct_format() {
        let address = String::from("example@example.com");
        let email = Email::create(address);
        assert_eq!(email.to_string(), String::from("example@example.com"));
    }

    #[test]
    #[should_panic(expected = "Invalid email format")]
    fn fails_creating_with_invalid_format() {
        Email::create("invalid".to_string());
    }

    #[test]
    fn two_emails_with_same_address_should_be_equal() {
        assert_eq!(
            Email::create("test@example.com".to_string()),
            Email::create("test@example.com".to_string())
        );
    }

    #[test]
    fn two_emails_with_different_address_should_not_be_equal() {
        assert_ne!(
            Email::create("tes@example.com".to_string()),
            Email::create("test@example.com".to_string())
        );
    }
}
