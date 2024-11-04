use async_trait::async_trait;

use crate::domain::entities::user::User;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::id::Id;

#[async_trait]
pub trait UserRepository {
    async fn save(&mut self, user: User) -> Result<(), String>;
    async fn find_by_id(&self, id: Id) -> Result<Option<User>, String>;
    async fn find_by_email(&self, email: Email) -> Result<Option<User>, String>;
    async fn find_all(&self) -> Result<Vec<User>, String>;
    async fn remove(&mut self, user: User) -> Result<(), String>;
}
