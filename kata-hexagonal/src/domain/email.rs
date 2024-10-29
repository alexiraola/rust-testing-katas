use core::panic;

use regex::Regex;

pub struct Email {
    email: String,
}

impl Email {
    pub fn new(email: &str) -> Self {
        let email_regex = Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(email) {
            panic!("Invalid email format");
        }
        Self {
            email: String::from(email),
        }
    }

    pub fn to_string(&self) -> String {
        self.email.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::domain::email::Email;

    #[test]
    fn create_email_with_correct_format() {
        let address = "example@example.com";
        let email = Email::new(address);
        assert_eq!(email.to_string(), address);
    }

    #[test]
    #[should_panic(expected = "Invalid email format")]
    fn fails_creating_with_invalid_format() {
        Email::new("invalid");
    }
}
