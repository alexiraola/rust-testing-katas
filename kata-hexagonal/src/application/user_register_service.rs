use std::{error::Error, fmt::Display, sync::Arc};

use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
    value_objects::{email::Email, id::Id, password::Password},
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

pub struct UserRegisterService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserRegisterService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        UserRegisterService { user_repository }
    }

    pub async fn register(
        &self,
        request: UserRegisterRequest,
    ) -> Result<UserRegisterResponse, Box<dyn Error>> {
        self.ensure_user_does_not_exist(&request).await?;
        let user = self.create_user(request)?;
        let dto = user.to_dto();

        self.user_repository.save(user).await?;

        Ok(UserRegisterResponse::from(dto))
    }

    async fn ensure_user_does_not_exist(
        &self,
        request: &UserRegisterRequest,
    ) -> Result<(), Box<dyn Error>> {
        let user_found = self
            .user_repository
            .find_by_email(Email::new(request.email.clone())?)
            .await;

        if let Ok(Some(_)) = user_found {
            Err(Box::new(ExistingUserError {}))
        } else {
            Ok(())
        }
    }

    fn create_user(&self, request: UserRegisterRequest) -> Result<User, Box<dyn Error>> {
        let id = Id::generate_unique_identifier();
        let email = Email::new(request.email)?;
        let password = Password::create_from_plaintext(request.password)?;
        Ok(User::new(id, email, password))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::{repositories::user_repository::UserRepository, value_objects::email::Email},
        infrastructure::in_memory_user_repository::InMemoryUserRepository,
    };

    use std::sync::Arc;

    use super::{UserRegisterRequest, UserRegisterService};

    #[tokio::test]
    async fn register_with_valid_credentials() {
        let register_request = create_register_request();

        let repo = Arc::new(InMemoryUserRepository::new());
        let mut register_service = UserRegisterService::new(repo.clone());

        let _ = register_service.register(register_request).await;

        let user = repo
            .find_by_email(Email::new("test@example.com".to_string()).unwrap())
            .await;

        assert!(user
            .unwrap()
            .unwrap()
            .is_matching_email(&Email::new("test@example.com".to_string()).unwrap()))
    }

    #[tokio::test]
    async fn does_not_allow_to_register_existing_email() {
        let register_request = create_register_request();

        let repo = Arc::new(InMemoryUserRepository::new());
        let mut register_service = UserRegisterService::new(repo.clone());

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
