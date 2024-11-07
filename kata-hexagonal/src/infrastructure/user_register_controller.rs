use crate::application::{
    dtos::{UserRegisterRequest, UserRegisterResponse},
    user_register_service::UserRegisterService,
};

use super::http::{HttpRequest, HttpResponse};

pub struct UserRegisterController<'a> {
    service: &'a mut UserRegisterService<'a>,
}

impl<'a> UserRegisterController<'a> {
    pub fn new(service: &'a mut UserRegisterService<'a>) -> Self {
        UserRegisterController { service }
    }

    pub async fn register(
        &mut self,
        request: HttpRequest<UserRegisterRequest>,
        response: &mut HttpResponse<UserRegisterResponse>,
    ) {
        // let result = try {
        //     self.service.register(request.body).await;
        // }
        match self.service.register(request.body).await {
            Ok(register_response) => response.status(201).json(Some(Ok(register_response))),
            Err(error) => response.status(400).json(Some(Err(error.to_string()))),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::{dtos::UserRegisterRequest, user_register_service::UserRegisterService},
        infrastructure::{
            http::{HttpRequest, HttpResponse},
            in_memory_user_repository::InMemoryUserRepository,
        },
    };

    use super::UserRegisterController;

    #[tokio::test]
    async fn register_a_valid_user() {
        let email = "test@example.com".to_string();
        let password = "SecurePass123_".to_string();

        let mut repo = InMemoryUserRepository::new();
        let mut register_service = UserRegisterService::new(&mut repo);
        let mut controller = UserRegisterController::new(&mut register_service);

        let mut response = HttpResponse::new();

        controller
            .register(
                HttpRequest {
                    body: UserRegisterRequest { email, password },
                },
                &mut response,
            )
            .await;

        assert_eq!(response.status_code, 201);
        assert_eq!(response.data.unwrap().unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn rejects_when_email_not_provided() {
        let email = "test@examplecom".to_string();
        let password = "SecurePass123_".to_string();

        let mut repo = InMemoryUserRepository::new();
        let mut register_service = UserRegisterService::new(&mut repo);
        let mut controller = UserRegisterController::new(&mut register_service);

        let mut response = HttpResponse::new();

        controller
            .register(
                HttpRequest {
                    body: UserRegisterRequest { email, password },
                },
                &mut response,
            )
            .await;

        assert_eq!(response.status_code, 400);
        assert_eq!(
            response.data.unwrap().unwrap_err(),
            "Invalid email format".to_string()
        );
    }
}
