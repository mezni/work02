use async_trait::async_trait;
use uuid::Uuid;
use crate::application::error::ApplicationResult;
use crate::domain::{User, Token};

#[async_trait]
pub trait AuthServiceTrait: Send + Sync {
    async fn login(&self, email: &str, password: &str) -> ApplicationResult<Token>;
    async fn logout(&self, refresh_token: &str) -> ApplicationResult<()>;
    async fn change_password(&self, user_id: Uuid, current_password: &str, new_password: &str) -> ApplicationResult<()>;
    async fn reset_password(&self, email: &str) -> ApplicationResult<()>;
    async fn confirm_password_reset(&self, token: &str, new_password: &str) -> ApplicationResult<()>;
}

#[async_trait]
pub trait RegistrationServiceTrait: Send + Sync {
    async fn register(&self, email: &str, password: &str) -> ApplicationResult<Uuid>;
    async fn verify_email(&self, token: &str) -> ApplicationResult<()>;
    async fn resend_verification(&self, email: &str) -> ApplicationResult<()>;
}

#[async_trait]
pub trait TokenServiceTrait: Send + Sync {
    async fn refresh_token(&self, refresh_token: &str) -> ApplicationResult<Token>;
    async fn validate_token(&self, token: &str) -> ApplicationResult<User>;
    async fn revoke_token(&self, token: &str) -> ApplicationResult<()>;
    async fn revoke_all_tokens(&self, user_id: Uuid) -> ApplicationResult<()>;
}

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn get_user(&self, user_id: Uuid) -> ApplicationResult<User>;
    async fn update_user_profile(&self, user_id: Uuid, company_name: Option<String>, station_name: Option<String>) -> ApplicationResult<User>;
    async fn update_user_role(&self, user_id: Uuid, role: crate::domain::value_objects::UserRole) -> ApplicationResult<User>;
}