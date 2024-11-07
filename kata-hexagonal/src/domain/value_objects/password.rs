use core::fmt;
use std::error::Error;

use crate::domain::common::hash;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct PasswordError {
    errors: Vec<PasswordErrorType>,
}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Password {}",
            self.errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Error for PasswordError {}

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordErrorType {
    TooShort,
    MustContainNumber,
    MustContainLowercase,
    MustContainUppercase,
    MustContainUnderscore,
}

impl fmt::Display for PasswordErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordErrorType::TooShort => write!(f, "is too short"),
            PasswordErrorType::MustContainNumber => write!(f, "must contain a number"),
            PasswordErrorType::MustContainLowercase => write!(f, "must contain a lowercase"),
            PasswordErrorType::MustContainUppercase => write!(f, "must contain a uppercase"),
            PasswordErrorType::MustContainUnderscore => write!(f, "must contain an underscore"),
        }
    }
}

impl Error for PasswordErrorType {}

#[derive(Debug, Clone)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn create_from_plaintext(plaintext: String) -> Result<Self, PasswordError> {
        Self::ensure_is_strong_password(&plaintext)?;
        Ok(Self {
            password: Self::hash_plaintext(&plaintext),
        })
    }

    fn hash_plaintext(plaintext: &String) -> String {
        hash::hash(plaintext)
    }

    fn ensure_is_strong_password(plaintext: &String) -> Result<(), PasswordError> {
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

    fn has_six_characters_or_more(plaintext: &String) -> bool {
        plaintext.len() >= 6
    }

    fn contains_number(plaintext: &String) -> bool {
        let regex = Regex::new(r"\d").unwrap();
        regex.is_match(plaintext)
    }

    fn contains_lowercase(plaintext: &String) -> bool {
        let regex = Regex::new(r"[a-z]").unwrap();
        regex.is_match(plaintext)
    }

    fn contains_uppercase(plaintext: &String) -> bool {
        let regex = Regex::new(r"[A-Z]").unwrap();
        regex.is_match(plaintext)
    }

    fn contains_underscore(plaintext: &String) -> bool {
        plaintext.contains("_")
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.password)
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.password == other.password
    }
}
impl Eq for Password {}

#[cfg(test)]
mod test {
    use crate::domain::value_objects::password::{Password, PasswordError, PasswordErrorType};
    use regex::Regex;

    #[test]
    fn creates_correct_password() {
        assert!(Password::create_from_plaintext(String::from("SecurePass123_")).is_ok());
    }

    #[test]
    fn fails_creating_with_short_password() {
        assert_eq!(
            Password::create_from_plaintext(String::from("1aA_")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::TooShort]
            })
        );
    }

    #[test]
    fn fails_creating_when_missing_a_number() {
        assert_eq!(
            Password::create_from_plaintext(String::from("aaaaaA_")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::MustContainNumber]
            })
        )
    }

    #[test]
    fn fails_when_missing_lowercase() {
        assert_eq!(
            Password::create_from_plaintext(String::from("1234A_")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::MustContainLowercase]
            })
        )
    }

    #[test]
    fn fails_when_missing_uppercase() {
        assert_eq!(
            Password::create_from_plaintext(String::from("1234a_")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::MustContainUppercase]
            })
        )
    }

    #[test]
    fn fails_when_missing_underscore() {
        assert_eq!(
            Password::create_from_plaintext(String::from("1234aA")),
            Err(PasswordError {
                errors: vec![PasswordErrorType::MustContainUnderscore]
            })
        )
    }

    #[test]
    fn fails_when_missing_several_requirements() {
        assert_eq!(
            Password::create_from_plaintext(String::from("abc")),
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
        let password = Password::create_from_plaintext(String::from("SecurePass123_")).unwrap();
        let hashed_value = password.to_string();

        let regex = Regex::new(r"[a-f-F0-9]{64}").unwrap();

        assert_ne!(hashed_value, String::from("SecurePass123_"));
        assert_eq!(hashed_value.len(), 64);
        assert!(regex.is_match(&hashed_value));
    }

    #[test]
    fn matches_for_two_same_passwords() {
        let a_password = Password::create_from_plaintext("SecurePass123_".to_string());
        let another_password = Password::create_from_plaintext("SecurePass123_".to_string());

        assert_eq!(a_password, another_password);
    }

    #[test]
    fn does_not_match_for_two_different_passwords() {
        let a_password = Password::create_from_plaintext("SecurePass123_".to_string());
        let another_password = Password::create_from_plaintext("SecurePass12_".to_string());

        assert_ne!(a_password, another_password);
    }
}
