use core::panic;

use crate::domain::{
    entities::user::{User, UserDto},
    repositories::user_repository::UserRepository,
    value_objects::{email::Email, id::Id, password::Password},
};

use super::dtos::{UserRegisterRequest, UserRegisterResponse};

pub struct UserRegisterService<'a> {
    user_repository: &'a mut dyn UserRepository,
}

impl<'a> UserRegisterService<'a> {
    fn new(user_repository: &'a mut dyn UserRepository) -> Self {
        UserRegisterService { user_repository }
    }

    pub async fn register(
        &mut self,
        request: UserRegisterRequest,
    ) -> Result<UserRegisterResponse, String> {
        self.ensure_user_does_not_exist(&request).await;
        let user = self.create_user(request);
        let dto = user.to_dto();

        self.user_repository.save(user).await?;

        Ok(UserRegisterResponse::from(dto))
    }

    async fn ensure_user_does_not_exist(&mut self, request: &UserRegisterRequest) {
        let user_found = self
            .user_repository
            .find_by_email(Email::create(request.email.clone()))
            .await;

        if let Ok(Some(_)) = user_found {
            panic!("User already exists with this email");
        }
    }

    fn create_user(&self, request: UserRegisterRequest) -> User {
        let id = Id::generate_unique_identifier();
        let email = Email::create(request.email);
        let password = Password::create_from_plaintext(request.password);
        User::new(id, email, password)
    }
}

#[cfg(test)]
mod test {
    use crate::{
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
            .find_by_email(Email::create("test@example.com".to_string()))
            .await;

        assert!(user
            .unwrap()
            .unwrap()
            .is_matching_email(&Email::create("test@example.com".to_string())))
    }

    #[tokio::test]
    #[should_panic(expected = "User already exists with this email")]
    async fn does_not_allow_to_register_existing_email() {
        let register_request = create_register_request();

        let mut repo = InMemoryUserRepository::new();
        let mut register_service = UserRegisterService::new(&mut repo);

        let _ = register_service.register(register_request.clone()).await;
        let _ = register_service.register(register_request.clone()).await;
    }

    fn create_register_request() -> UserRegisterRequest {
        UserRegisterRequest {
            email: "test@example.com".to_string(),
            password: "TestPass123_".to_string(),
        }
    }
}
