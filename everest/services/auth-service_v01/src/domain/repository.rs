use crate::domain::models::User;
use anyhow::Result;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User, password: &str) -> Result<String>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn assign_role(&self, user_id: &str, role: &str) -> Result<()>;
}
