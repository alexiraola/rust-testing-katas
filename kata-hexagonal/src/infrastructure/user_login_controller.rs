use std::error::Error;

use crate::application::{
    dtos::{UserLoginRequest, UserLoginResponse},
    user_login_service::UserLoginService,
};

use super::http::{HttpRequest, HttpResponse};

pub struct UserLoginController {
    service: UserLoginService,
}

impl UserLoginController {
    pub fn new(service: UserLoginService) -> Self {
        UserLoginController { service }
    }

    pub async fn login<T: HttpResponse<Result<UserLoginResponse, Box<dyn Error>>>>(
        &self,
        request: HttpRequest<UserLoginRequest>,
        response: &mut T,
    ) {
        match self.service.login(request.body).await {
            Ok(login_response) => response.status(200).json(Ok(login_response)),
            Err(error) => response.status(400).json(Err(error)),
        };
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::sync::Arc;

    use crate::{
        application::{
            dtos::{UserLoginRequest, UserLoginResponse},
            user_login_service::UserLoginService,
        },
        domain::{
            entities::user::User,
            repositories::user_repository::UserRepository,
            value_objects::{email::Email, id::Id, password::Password},
        },
        infrastructure::{
            http::{HttpRequest, HttpResponse},
            in_memory_user_repository::InMemoryUserRepository,
        },
    };

    use super::UserLoginController;

    struct MockResponse {
        status: u16,
        data: Option<Result<UserLoginResponse, Box<dyn Error>>>,
    }

    impl HttpResponse<Result<UserLoginResponse, Box<dyn Error>>> for MockResponse {
        fn status(&mut self, code: u16) -> &mut Self {
            self.status = code;
            self
        }

        fn json(&mut self, data: Result<UserLoginResponse, Box<dyn Error>>) -> &mut Self {
            self.data = Some(data);
            self
        }
    }

    #[tokio::test]
    async fn login_a_user() {
        let email = "test@example.com".to_string();
        let password = "TestPass123_".to_string();

        let repo = Arc::new(InMemoryUserRepository::new());
        let login_service = UserLoginService::new(repo.clone());
        let controller = UserLoginController::new(login_service);

        repo.as_ref().save(create_user().unwrap()).await;

        let mut response = MockResponse {
            status: 200,
            data: None,
        };

        controller
            .login(
                HttpRequest {
                    body: UserLoginRequest { email, password },
                },
                &mut response,
            )
            .await;

        assert_eq!(response.status, 200);
        assert_eq!(response.data.unwrap().unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn rejects_when_invalid_email_provided() {
        let email = "test@examplecom".to_string();
        let password = "SecurePass123_".to_string();

        let repo = Arc::new(InMemoryUserRepository::new());
        let login_service = UserLoginService::new(repo.clone());
        let controller = UserLoginController::new(login_service);

        let mut response = MockResponse {
            status: 200,
            data: None,
        };

        controller
            .login(
                HttpRequest {
                    body: UserLoginRequest { email, password },
                },
                &mut response,
            )
            .await;

        assert_eq!(response.status, 400);
        assert!(response.data.unwrap().is_err());
    }

    fn create_user() -> Result<User, Box<dyn Error>> {
        let id = Id::generate_unique_identifier();
        let email = Email::new("test@example.com".to_string())?;
        let password = Password::new("TestPass123_".to_string())?;

        Ok(User::new(id, email, password))
    }
}
