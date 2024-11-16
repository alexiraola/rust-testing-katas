use std::error::Error;

use crate::application::{
    dtos::{UserRegisterRequest, UserRegisterResponse},
    user_register_service::UserRegisterService,
};

use super::http::{HttpRequest, HttpResponse};

pub struct UserRegisterController {
    service: UserRegisterService,
}

impl UserRegisterController {
    pub fn new(service: UserRegisterService) -> Self {
        UserRegisterController { service }
    }

    pub async fn register<T: HttpResponse<Result<UserRegisterResponse, Box<dyn Error>>>>(
        &mut self,
        request: HttpRequest<UserRegisterRequest>,
        response: &mut T,
    ) {
        match self.service.register(request.body).await {
            Ok(register_response) => response.status(201).json(Ok(register_response)),
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
            dtos::{UserRegisterRequest, UserRegisterResponse},
            user_register_service::UserRegisterService,
        },
        infrastructure::{
            http::{HttpRequest, HttpResponse},
            in_memory_user_repository::InMemoryUserRepository,
        },
    };

    use super::UserRegisterController;

    struct MockResponse {
        status: u16,
        data: Option<Result<UserRegisterResponse, Box<dyn Error>>>,
    }

    impl HttpResponse<Result<UserRegisterResponse, Box<dyn Error>>> for MockResponse {
        fn status(&mut self, code: u16) -> &mut Self {
            self.status = code;
            self
        }

        fn json(&mut self, data: Result<UserRegisterResponse, Box<dyn Error>>) -> &mut Self {
            self.data = Some(data);
            self
        }
    }

    #[tokio::test]
    async fn register_a_valid_user() {
        let email = "test@example.com".to_string();
        let password = "SecurePass123_".to_string();

        let repo = Arc::new(InMemoryUserRepository::new());
        let register_service = UserRegisterService::new(repo.clone());
        let mut controller = UserRegisterController::new(register_service);

        let mut response = MockResponse {
            status: 200,
            data: None,
        };

        controller
            .register(
                HttpRequest {
                    body: UserRegisterRequest { email, password },
                },
                &mut response,
            )
            .await;

        assert_eq!(response.status, 201);
        assert_eq!(response.data.unwrap().unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn rejects_when_invalid_email_provided() {
        let email = "test@examplecom".to_string();
        let password = "SecurePass123_".to_string();

        let repo = Arc::new(InMemoryUserRepository::new());
        let register_service = UserRegisterService::new(repo.clone());
        let mut controller = UserRegisterController::new(register_service);

        let mut response = MockResponse {
            status: 200,
            data: None,
        };

        controller
            .register(
                HttpRequest {
                    body: UserRegisterRequest { email, password },
                },
                &mut response,
            )
            .await;

        assert_eq!(response.status, 400);
        assert!(response.data.unwrap().is_err());
    }
}
