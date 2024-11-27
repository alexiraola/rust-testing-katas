use crate::domain::value_objects::{
    email::{self, Email},
    id::Id,
    password::Password,
};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("New password must be different")]
pub struct EqualPasswordError {}

#[derive(Debug, Clone)]
pub struct User {
    id: Id,
    email: Email,
    password: Password,
}

pub struct UserDto {
    pub id: String,
    pub email: String,
}

impl User {
    pub fn new(id: Id, email: Email, password: Password) -> Self {
        User {
            id,
            email,
            password,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn email(&self) -> String {
        self.email.to_string()
    }

    pub fn password(&self) -> String {
        self.password.to_string()
    }

    pub fn change_password(&mut self, new_password: Password) -> Result<(), EqualPasswordError> {
        self.ensure_is_different_password(&new_password)?;
        self.password = new_password;
        Ok(())
    }

    fn ensure_is_different_password(
        &mut self,
        new_password: &Password,
    ) -> Result<(), EqualPasswordError> {
        if self.is_matching_password(new_password) {
            Err(EqualPasswordError {})
        } else {
            Ok(())
        }
    }

    pub fn is_matching_password(&self, password: &Password) -> bool {
        self.password == *password
    }

    pub fn is_matching_id(&self, id: &Id) -> bool {
        self.id == *id
    }

    pub fn is_matching_email(&self, email: &Email) -> bool {
        self.email == *email
    }

    pub fn to_dto(&self) -> UserDto {
        UserDto {
            id: self.id.to_string(),
            email: self.email.to_string(),
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for User {}

#[cfg(test)]
mod test {
    use crate::domain::{
        entities::user::EqualPasswordError,
        value_objects::{email::Email, id::Id, password::Password},
    };

    use super::User;

    #[test]
    fn changes_password_when_different_provided() {
        let mut user = create_user();

        let _ = user.change_password(Password::new("AnotherSafePass123_".to_string()).unwrap());

        assert!(
            user.is_matching_password(&Password::new("AnotherSafePass123_".to_string()).unwrap())
        )
    }

    #[test]
    fn does_not_allow_to_change_with_same_password() {
        let mut user = create_user();

        assert_eq!(
            user.change_password(Password::new("SafePass123_".to_string()).unwrap()),
            Err(EqualPasswordError {})
        );
    }

    fn create_user() -> User {
        let id = Id::generate_unique_identifier();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password = Password::new("SafePass123_".to_string()).unwrap();

        User::new(id, email, password)
    }
}
