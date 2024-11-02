use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
    value_objects::{email::Email, id::Id},
};

#[derive(Debug)]
pub struct InMemoryUserRepository {
    users: Vec<User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository { users: Vec::new() }
    }
}

impl UserRepository for InMemoryUserRepository {
    async fn save(&mut self, user: User) -> Result<(), String> {
        self.users.push(user);
        Ok(())
    }
    async fn find_by_id(&self, id: Id) -> Option<crate::domain::entities::user::User> {
        None
    }

    async fn find_by_email(&self, email: Email) -> Option<User> {
        None
    }

    async fn find_all(&self) -> Vec<User> {
        self.users.clone()
    }

    async fn remove(&mut self, user: User) {
        todo!()
    }

    // add code here
}

#[cfg(test)]
mod test {
    use crate::domain::{
        entities::user::User,
        repositories::user_repository::UserRepository,
        value_objects::{email::Email, id::Id, password::Password},
    };

    use super::InMemoryUserRepository;

    #[tokio::test]
    async fn find_user_by_id() {
        let id = Id::generate_unique_identifier();
        let email = Email::create("test@example.com".to_string());
        let password = Password::create_from_plaintext("SafePass123_".to_string());
        let user = User {
            id,
            email,
            password,
        };

        let mut repo = InMemoryUserRepository::new();
        repo.save(user).await;

        let users = repo.find_all().await;

        assert_eq!(users.len(), 1);
    }
}
