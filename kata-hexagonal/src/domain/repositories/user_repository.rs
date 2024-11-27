use async_trait::async_trait;

use crate::domain::entities::user::User;
use crate::domain::value_objects::email::Email;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> Result<(), String>;
    async fn find_by_email(&self, email: Email) -> Result<Option<User>, String>;
    async fn find_all(&self) -> Result<Vec<User>, String>;
    async fn remove(&self, user: User) -> Result<(), String>;
}
