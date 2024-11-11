use core::fmt;
use thiserror::Error;

use crate::domain::common::hash;
use regex::Regex;

#[derive(Error, Debug, PartialEq)]
#[error("Password {}", errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))]
pub struct PasswordError {
    errors: Vec<PasswordErrorType>,
}

#[derive(Error, Debug, PartialEq)]
pub enum PasswordErrorType {
    #[error("is too short")]
    TooShort,
    #[error("must contain a number")]
    MustContainNumber,
    #[error("must contain a lowercase")]
    MustContainLowercase,
    #[error("must contain a uppercase")]
    MustContainUppercase,
    #[error("must contain an underscore")]
    MustContainUnderscore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn new(plaintext: String) -> Result<Self, PasswordError> {
        Self::ensure_is_strong_password(&plaintext)?;
        Ok(Self(Self::hash_plaintext(&plaintext)))
    }

    fn hash_plaintext(plaintext: &str) -> String {
        hash::hash(plaintext)
    }

    fn ensure_is_strong_password(plaintext: &str) -> Result<(), PasswordError> {
        let mut accumulated_errors = vec![];

        if !Self::has_six_characters_or_more(plaintext) {
            accumulated_errors.push(PasswordErrorType::TooShort);
        }
        if !Self::contains_number(plaintext) {
            accumulated_errors.push(PasswordErrorType::MustContainNumber);
        }
        if !Self::contains_lowercase(plaintext) {
            accumulated_errors.push(PasswordErrorType::MustContainLowercase);
        }
        if !Self::contains_uppercase(plaintext) {
            accumulated_errors.push(PasswordErrorType::MustContainUppercase);
        }
        if !Self::contains_underscore(plaintext) {
            accumulated_errors.push(PasswordErrorType::MustContainUnderscore);
        }

        if !accumulated_errors.is_empty() {
            Err(PasswordError {
                errors: accumulated_errors,
            })
        } else {
            Ok(())
        }
    }

    fn has_six_characters_or_more(plaintext: &str) -> bool {
        plaintext.len() >= 6
    }

    fn contains_number(plaintext: &str) -> bool {
        let regex = Regex::new(r"\d").unwrap();
        regex.is_match(plaintext)
    }

    fn contains_lowercase(plaintext: &str) -> bool {
        let regex = Regex::new(r"[a-z]").unwrap();
        regex.is_match(plaintext)
    }

    fn contains_uppercase(plaintext: &str) -> bool {
        let regex = Regex::new(r"[A-Z]").unwrap();
        regex.is_match(plaintext)
    }

    fn contains_underscore(plaintext: &str) -> bool {
        plaintext.contains("_")
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::domain::value_objects::password::{Password, PasswordError, PasswordErrorType};
    use regex::Regex;

    #[test]
    fn creates_correct_password() {
        assert!(Password::new(String::from("SecurePass123_")).is_ok());
    }

    #[test]
    fn fails_creating_with_short_password() {
        assert_eq!(
            Password::new(String::from("1aA_")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::TooShort]
            })
        );
    }

    #[test]
    fn fails_creating_when_missing_a_number() {
        assert_eq!(
            Password::new(String::from("aaaaaA_")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::MustContainNumber]
            })
        )
    }

    #[test]
    fn fails_when_missing_lowercase() {
        assert_eq!(
            Password::new(String::from("1234A_")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::MustContainLowercase]
            })
        )
    }

    #[test]
    fn fails_when_missing_uppercase() {
        assert_eq!(
            Password::new(String::from("1234a_")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::MustContainUppercase]
            })
        )
    }

    #[test]
    fn fails_when_missing_underscore() {
        assert_eq!(
            Password::new(String::from("1234aA")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::MustContainUnderscore]
            })
        )
    }

    #[test]
    fn fails_when_missing_several_requirements() {
        assert_eq!(
            Password::new(String::from("abc")),
            Err(PasswordError {
                errors: vec![
                    PasswordErrorType::TooShort,
                    PasswordErrorType::MustContainNumber,
                    PasswordErrorType::MustContainUppercase,
                    PasswordErrorType::MustContainUnderscore
                ]
            })
        )
    }

    #[test]
    fn ensures_password_is_hashed() {
        let password = Password::new(String::from("SecurePass123_")).unwrap();
        let hashed_value = password.to_string();

        let regex = Regex::new(r"[a-f-F0-9]{64}").unwrap();

        assert_ne!(hashed_value, String::from("SecurePass123_"));
        assert_eq!(hashed_value.len(), 64);
        assert!(regex.is_match(&hashed_value));
    }

    #[test]
    fn matches_for_two_same_passwords() {
        let a_password = Password::new("SecurePass123_".to_string());
        let another_password = Password::new("SecurePass123_".to_string());

        assert_eq!(a_password, another_password);
    }

    #[test]
    fn does_not_match_for_two_different_passwords() {
        let a_password = Password::new("SecurePass123_".to_string());
        let another_password = Password::new("SecurePass12_".to_string());

        assert_ne!(a_password, another_password);
    }
}
