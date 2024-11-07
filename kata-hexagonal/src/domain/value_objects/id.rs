use crate::domain::common::uuid::generate_uuid;
use core::fmt;
use regex::Regex;
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidIdError {}

impl fmt::Display for InvalidIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Id format")
    }
}

impl Error for InvalidIdError {}

#[derive(Debug, Clone)]
pub struct Id {
    id: String,
}

impl Id {
    pub fn generate_unique_identifier() -> Self {
        Id {
            id: generate_uuid(),
        }
    }

    pub fn from(id: String) -> Result<Self, InvalidIdError> {
        Self::ensure_is_valid_id(&id)?;
        Ok(Id { id })
    }

    fn ensure_is_valid_id(id: &String) -> Result<(), InvalidIdError> {
        let uuid_regex = Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        )
        .unwrap();

        if !uuid_regex.is_match(id) {
            Err(InvalidIdError {})
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Id {}

#[cfg(test)]
mod test {
    use crate::domain::value_objects::id::{Id, InvalidIdError};
    use regex::Regex;

    #[test]
    fn generates_valid_identifier() {
        let id = Id::generate_unique_identifier();
        let uuid_regex = Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        )
        .unwrap();
        assert!(uuid_regex.is_match(&id.to_string()));
    }

    #[test]
    fn create_id_from_valid_identifier() {
        let uuid = "3e1f1e36-ecb3-42bd-9f6b-a4d6d0835495".to_string();
        let id = Id::from(uuid.clone());

        assert_eq!(id.unwrap().to_string(), uuid);
    }

    #[test]
    fn does_not_allow_to_create_from_invalid_identifier() {
        assert_eq!(Id::from("invalid-id".to_string()), Err(InvalidIdError {}));
    }

    #[test]
    fn two_ids_with_same_identifier_should_be_equal() {
        let id1 = Id::from("d7ee4068-42c3-4787-8701-707e4ce145cf".to_string());
        let id2 = Id::from("d7ee4068-42c3-4787-8701-707e4ce145cf".to_string());

        assert_eq!(id1, id2);
    }

    #[test]
    fn two_ids_with_different_identifier_should_not_be_equal() {
        let id1 = Id::from("c7d51220-93d0-4b85-be13-7a02b598aa0e".to_string());
        let id2 = Id::from("d7ee4068-42c3-4787-8701-707e4ce145cf".to_string());

        assert_ne!(id1, id2);
    }
}
