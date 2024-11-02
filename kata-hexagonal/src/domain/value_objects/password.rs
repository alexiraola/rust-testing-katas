use core::{fmt, panic};

use crate::domain::common::hash;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn create_from_plaintext(plaintext: String) -> Self {
        Self::ensure_is_strong_password(&plaintext);
        Self {
            password: Self::hash_plaintext(&plaintext),
        }
    }

    fn hash_plaintext(plaintext: &String) -> String {
        hash::hash(plaintext)
    }

    fn ensure_is_strong_password(plaintext: &String) {
        let mut accumulated_errors = vec![];

        if !Self::has_six_characters_or_more(plaintext) {
            accumulated_errors.push("is too short");
        }
        if !Self::contains_number(plaintext) {
            accumulated_errors.push("must contain a number");
        }
        if !Self::contains_lowercase(plaintext) {
            accumulated_errors.push("must contain a lowercase");
        }
        if !Self::contains_uppercase(plaintext) {
            accumulated_errors.push("must contain an uppercase");
        }
        if !Self::contains_underscore(plaintext) {
            accumulated_errors.push("must contain an underscore");
        }

        if accumulated_errors.len() > 0 {
            panic!("Password {}", accumulated_errors.join(", "))
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
    use crate::domain::value_objects::password::Password;
    use regex::Regex;

    #[test]
    fn creates_correct_password() {
        Password::create_from_plaintext(String::from("SecurePass123_"));
    }

    #[test]
    #[should_panic(expected = "Password is too short")]
    fn fails_creating_with_short_password() {
        Password::create_from_plaintext(String::from("1aA_"));
    }

    #[test]
    #[should_panic(expected = "Password must contain a number")]
    fn fails_creating_when_missing_a_number() {
        Password::create_from_plaintext(String::from("aaaaaA_"));
    }

    #[test]
    #[should_panic(expected = "Password must contain a lowercase")]
    fn fails_when_missing_lowercase() {
        Password::create_from_plaintext(String::from("1234A_"));
    }

    #[test]
    #[should_panic(expected = "Password must contain an uppercase")]
    fn fails_when_missing_uppercase() {
        Password::create_from_plaintext(String::from("1234a_"));
    }

    #[test]
    #[should_panic(expected = "Password must contain an underscore")]
    fn fails_when_missing_underscore() {
        Password::create_from_plaintext(String::from("1234aA"));
    }

    #[test]
    #[should_panic(
        expected = "Password is too short, must contain a number, must contain an uppercase, must contain an underscore"
    )]
    fn fails_when_missing_several_requirements() {
        Password::create_from_plaintext(String::from("abc"));
    }

    #[test]
    fn ensures_password_is_hashed() {
        let password = Password::create_from_plaintext(String::from("SecurePass123_"));
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
