use std::{error::Error, fmt};

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Email {
    email: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailError {
    InvalidFormat,
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::InvalidFormat => write!(f, "Invalid email format"),
        }
    }
}

impl Error for EmailError {}

impl Email {
    pub fn create(address: String) -> Result<Self, EmailError> {
        Self::ensure_is_valid_email(&address)?;
        Ok(Self { email: address })
    }

    fn ensure_is_valid_email(address: &str) -> Result<(), EmailError> {
        let email_regex = Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(address) {
            return Err(EmailError::InvalidFormat);
        }
        Ok(())
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
    use crate::domain::value_objects::email::{Email, EmailError};

    #[test]
    fn create_email_with_correct_format() {
        let email = Email::create(String::from("example@example.com"));
        assert_eq!(
            email.unwrap().to_string(),
            String::from("example@example.com")
        );
    }

    #[test]
    fn fails_creating_with_invalid_format() {
        let email = Email::create("invalid".to_string());
        assert_eq!(email.unwrap_err(), EmailError::InvalidFormat);
    }

    #[test]
    fn two_emails_with_same_address_should_be_equal() {
        assert_eq!(
            Email::create("test@example.com".to_string()).unwrap(),
            Email::create("test@example.com".to_string()).unwrap()
        );
    }

    #[test]
    fn two_emails_with_different_address_should_not_be_equal() {
        assert_ne!(
            Email::create("tes@example.com".to_string()).unwrap(),
            Email::create("test@example.com".to_string()).unwrap()
        );
    }
}
