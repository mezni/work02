// src/domain/user_repository.rs
use crate::domain::user::User;

pub trait UserRepository {
    async fn find_by_id(&self, id: &str) -> Option<User>;
    async fn save(&self, user: User) -> Result<(), Error>;
}