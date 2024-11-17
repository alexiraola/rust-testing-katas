use std::{error::Error, sync::Arc};

use crate::domain::{
    repositories::user_repository::UserRepository,
    value_objects::{email::Email, password::Password},
};

use super::dtos::{UserLoginRequest, UserLoginResponse};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Invalid email or password")]
pub struct InvalidCredentialsError {}

pub struct UserLoginService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserLoginService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        UserLoginService { user_repository }
    }

    pub async fn login(
        &self,
        request: UserLoginRequest,
    ) -> Result<UserLoginResponse, Box<dyn Error>> {
        let password = Password::new(request.password)?;
        let optional_user = self
            .user_repository
            .find_by_email(Email::new(request.email.clone())?)
            .await
            .map_err(|_| InvalidCredentialsError {})?;

        if let Some(user) = optional_user {
            if user.is_matching_password(&password) {
                return Ok(user.to_dto().into());
            }
        }

        Err(Box::new(InvalidCredentialsError {}))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::{dtos::UserLoginRequest, user_login_service::UserLoginService},
        domain::{
            entities::user::User,
            repositories::user_repository::UserRepository,
            value_objects::{email::Email, id::Id, password::Password},
        },
        infrastructure::in_memory_user_repository::InMemoryUserRepository,
    };

    use std::{error::Error, sync::Arc};

    #[tokio::test]
    async fn register_with_valid_credentials() {
        let login_request = create_login_request();

        let repo = Arc::new(InMemoryUserRepository::new());
        let login_service = UserLoginService::new(repo.clone());

        let user = create_user().unwrap();
        repo.save(user).await;

        let response = login_service.login(login_request).await;

        assert!(response.is_ok_and(|r| r.email == "test@example.com".to_string()));
    }

    fn create_user() -> Result<User, Box<dyn Error>> {
        let id = Id::generate_unique_identifier();
        let email = Email::new("test@example.com".to_string())?;
        let password = Password::new("TestPass123_".to_string())?;

        Ok(User::new(id, email, password))
    }

    fn create_login_request() -> UserLoginRequest {
        UserLoginRequest {
            email: "test@example.com".to_string(),
            password: "TestPass123_".to_string(),
        }
    }
}
