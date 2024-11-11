use regex::Regex;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Email(String);

#[derive(Error, Debug, Clone, PartialEq)]
pub enum EmailError {
    #[error("Invalid email format")]
    InvalidFormat,
}

impl Email {
    pub fn new(address: String) -> Result<Self, EmailError> {
        Self::ensure_is_valid_email(&address)?;
        Ok(Self(address))
    }

    fn ensure_is_valid_email(address: &str) -> Result<(), EmailError> {
        let email_regex = Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(address) {
            return Err(EmailError::InvalidFormat);
        }
        Ok(())
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::domain::value_objects::email::{Email, EmailError};

    #[test]
    fn create_email_with_correct_format() {
        let email = Email::new(String::from("example@example.com"));
        assert_eq!(
            email.unwrap().to_string(),
            String::from("example@example.com")
        );
    }

    #[test]
    fn fails_creating_with_invalid_format() {
        let email = Email::new("invalid".to_string());
        assert_eq!(email.unwrap_err(), EmailError::InvalidFormat);
    }

    #[test]
    fn two_emails_with_same_address_should_be_equal() {
        assert_eq!(
            Email::new("test@example.com".to_string()).unwrap(),
            Email::new("test@example.com".to_string()).unwrap()
        );
    }

    #[test]
    fn two_emails_with_different_address_should_not_be_equal() {
        assert_ne!(
            Email::new("tes@example.com".to_string()).unwrap(),
            Email::new("test@example.com".to_string()).unwrap()
        );
    }
}
