use std::{error::Error, fmt::Display};

use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
    value_objects::{
        email::{Email, EmailError},
        id::Id,
        password::Password,
    },
};

use super::dtos::{UserRegisterRequest, UserRegisterResponse};

#[derive(Debug, PartialEq, Eq)]
pub struct ExistingUserError {}

impl Display for ExistingUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User already exists with this email")
    }
}

impl Error for ExistingUserError {}

pub struct UserRegisterService<'a> {
    user_repository: &'a mut dyn UserRepository,
}

impl<'a> UserRegisterService<'a> {
    pub fn new(user_repository: &'a mut dyn UserRepository) -> Self {
        UserRegisterService { user_repository }
    }

    pub async fn register(
        &mut self,
        request: UserRegisterRequest,
    ) -> Result<UserRegisterResponse, Box<dyn Error>> {
        self.ensure_user_does_not_exist(&request).await?;
        let user = self.create_user(request)?;
        let dto = user.to_dto();

        self.user_repository.save(user).await?;

        Ok(UserRegisterResponse::from(dto))
    }

    async fn ensure_user_does_not_exist(
        &mut self,
        request: &UserRegisterRequest,
    ) -> Result<(), Box<dyn Error>> {
        let user_found = self
            .user_repository
            .find_by_email(Email::create(request.email.clone())?)
            .await;

        if let Ok(Some(_)) = user_found {
            Err(Box::new(EmailError::InvalidFormat))
        } else {
            Ok(())
        }
    }

    fn create_user(&self, request: UserRegisterRequest) -> Result<User, Box<dyn Error>> {
        let id = Id::generate_unique_identifier();
        let email = Email::create(request.email)?;
        let password = Password::create_from_plaintext(request.password)?;
        Ok(User::new(id, email, password))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::user_register_service::ExistingUserError,
        domain::{repositories::user_repository::UserRepository, value_objects::email::Email},
        infrastructure::in_memory_user_repository::InMemoryUserRepository,
    };

    use super::{UserRegisterRequest, UserRegisterService};

    #[tokio::test]
    async fn register_with_valid_credentials() {
        let register_request = create_register_request();

        let mut repo = InMemoryUserRepository::new();
        let mut register_service = UserRegisterService::new(&mut repo);

        let _ = register_service.register(register_request).await;

        let user = repo
            .find_by_email(Email::create("test@example.com".to_string()).unwrap())
            .await;

        assert!(user
            .unwrap()
            .unwrap()
            .is_matching_email(&Email::create("test@example.com".to_string()).unwrap()))
    }

    #[tokio::test]
    async fn does_not_allow_to_register_existing_email() {
        let register_request = create_register_request();

        let mut repo = InMemoryUserRepository::new();
        let mut register_service = UserRegisterService::new(&mut repo);

        let _ = register_service.register(register_request.clone()).await;
        let res = register_service.register(register_request.clone()).await;

        assert!(res.is_err());
    }

    fn create_register_request() -> UserRegisterRequest {
        UserRegisterRequest {
            email: "test@example.com".to_string(),
            password: "TestPass123_".to_string(),
        }
    }
}
