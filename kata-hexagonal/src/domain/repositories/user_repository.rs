use crate::domain::entities::user::User;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::id::Id;

pub trait UserRepository {
    async fn save(&mut self, user: User) -> Result<(), String>;
    async fn find_by_id(&self, id: Id) -> Option<User>;
    async fn find_by_email(&self, email: Email) -> Option<User>;
    async fn find_all(&self) -> Vec<User>;
    async fn remove(&mut self, user: User);
}
