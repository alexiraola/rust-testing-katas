use std::sync::Mutex;

use async_trait::async_trait;

use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
    value_objects::{email::Email, id::Id},
};

#[derive(Debug)]
pub struct InMemoryUserRepository {
    users: Mutex<Vec<User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            users: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: User) -> Result<(), String> {
        if let Ok(mut lock) = self.users.lock() {
            if let Some(pos) = lock.iter().position(|u| *u == user) {
                lock[pos] = user;
            } else {
                lock.push(user);
            }
        }
        Ok(())
    }
    async fn find_by_id(&self, id: Id) -> Result<Option<User>, String> {
        let users = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err("Could not unlock".to_string()),
        };

        let user = users.iter().find(|user| user.is_matching_id(&id));

        Ok(user.cloned())
    }

    async fn find_by_email(&self, email: Email) -> Result<Option<User>, String> {
        let users = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err("Could not unlock".to_string()),
        };

        let user = users.iter().find(|u| u.is_matching_email(&email));

        Ok(user.cloned())
    }

    async fn find_all(&self) -> Result<Vec<User>, String> {
        let users = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err("Could not unlock".to_string()),
        };
        Ok(users.clone())
    }

    async fn remove(&self, user: User) -> Result<(), String> {
        let mut users = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err("Could not unlock".to_string()),
        };
        users.retain(|u| *u != user);
        Ok(())
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
        let user = create_user_by_id(id.clone());

        let mut repo = InMemoryUserRepository::new();
        let _res = repo.save(user.clone()).await;

        let found_user = repo.find_by_id(id.clone()).await;

        assert_eq!(found_user, Ok(Some(user)));
    }

    #[tokio::test]
    async fn find_user_by_email() {
        let email = Email::create("test@example.com".to_string()).unwrap();
        let user = create_user_by_email(email.clone());

        let mut repo = InMemoryUserRepository::new();
        let _res = repo.save(user.clone()).await;

        let found_user = repo.find_by_email(email.clone()).await;

        assert_eq!(found_user, Ok(Some(user)));
    }

    #[tokio::test]
    async fn does_not_find_non_existing_user_by_id() {
        let id = Id::generate_unique_identifier();

        let repo = InMemoryUserRepository::new();

        let found_user = repo.find_by_id(id.clone()).await;

        assert_eq!(found_user, Ok(None));
    }

    #[tokio::test]
    async fn does_not_find_non_existing_user_by_email() {
        let email = Email::create("test@example.com".to_string()).unwrap();

        let repo = InMemoryUserRepository::new();

        let found_user = repo.find_by_email(email.clone()).await;

        assert_eq!(found_user, Ok(None));
    }

    #[tokio::test]
    async fn finds_all_users() {
        let a_user = create_user_by_email(Email::create("test1@example.com".to_string()).unwrap());
        let another_user =
            create_user_by_email(Email::create("test2@example.com".to_string()).unwrap());
        let mut repo = InMemoryUserRepository::new();

        let _ = repo.save(a_user.clone()).await;
        let _ = repo.save(another_user.clone()).await;

        let users = repo.find_all().await;

        assert_eq!(users.as_ref().unwrap().len(), 2);
        assert_eq!(users, Ok(vec![a_user.clone(), another_user.clone()]));
    }

    #[tokio::test]
    async fn finds_no_users_when_empty() {
        let repo = InMemoryUserRepository::new();
        let users = repo.find_all().await;

        assert_eq!(users.as_ref().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn removes_a_user() {
        let email = Email::create("test@example.com".to_string()).unwrap();
        let user = create_user_by_email(email.clone());
        let mut repo = InMemoryUserRepository::new();

        let _ = repo.save(user.clone()).await;
        let _ = repo.remove(user.clone()).await;

        let found_user = repo.find_by_email(email.clone()).await;

        assert_eq!(found_user, Ok(None));
    }

    #[tokio::test]
    async fn update_user_when_exists() {
        let a_user = create_user_by_email(Email::create("test1@example.com".to_string()).unwrap());
        let mut repo = InMemoryUserRepository::new();

        let _ = repo.save(a_user.clone()).await;
        let _ = repo.save(a_user.clone()).await;

        let users = repo.find_all().await;

        assert_eq!(users.as_ref().unwrap().len(), 1);
        assert_eq!(users, Ok(vec![a_user.clone()]));
    }

    fn create_user_by_id(id: Id) -> User {
        let email = Email::create("test@example.com".to_string()).unwrap();
        let password = Password::create_from_plaintext("SafePass123_".to_string()).unwrap();
        User::new(id, email, password)
    }

    fn create_user_by_email(email: Email) -> User {
        let id = Id::generate_unique_identifier();
        let password = Password::create_from_plaintext("SafePass123_".to_string()).unwrap();
        User::new(id, email, password)
    }
}
