use async_trait::async_trait;
use crate::domain::models::user::User;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn create_user(&self, username: &str, email: Option<&str>, first_name: Option<&str>, last_name: Option<&str>, password: &str) -> anyhow::Result<User>;
}