use core::panic;

use crate::domain::value_objects::{email::Email, id::Id, password::Password};

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

    pub fn change_password(&mut self, new_password: Password) {
        self.ensure_is_different_password(&new_password);
        self.password = new_password;
    }

    fn ensure_is_different_password(&mut self, new_password: &Password) {
        if self.is_matching_password(new_password) {
            panic!("New password must be different");
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
    use crate::domain::value_objects::{email::Email, id::Id, password::Password};

    use super::User;

    #[test]
    fn changes_password_when_different_provided() {
        let mut user = create_user();

        user.change_password(Password::create_from_plaintext(
            "AnotherSafePass123_".to_string(),
        ));

        assert!(user.is_matching_password(&Password::create_from_plaintext(
            "AnotherSafePass123_".to_string()
        )))
    }

    #[test]
    #[should_panic(expected = "New password must be different")]
    fn does_not_allow_to_change_with_same_password() {
        let mut user = create_user();

        user.change_password(Password::create_from_plaintext("SafePass123_".to_string()));
    }

    fn create_user() -> User {
        let id = Id::generate_unique_identifier();
        let email = Email::create("test@example.com".to_string());
        let password = Password::create_from_plaintext("SafePass123_".to_string());

        User::new(id, email, password)
    }
}
