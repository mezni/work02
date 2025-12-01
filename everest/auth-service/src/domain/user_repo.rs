// domain/user_repo.rs
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::User;

#[async_trait]
pub trait UserRepository {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, crate::error::Error>;
    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<User>, crate::error::Error>;
    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, crate::error::Error>;
    async fn save_admin_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, crate::error::Error>;
}
