use async_trait::async_trait;
use uuid::Uuid;
use super::user::User;
use super::token::{Token, TokenClaims, RefreshToken};
use super::registration::Registration;
use super::error::DomainResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<User>;
    async fn find_by_email(&self, email: &str) -> DomainResult<User>;
    async fn create(&self, user: &User, password: &str) -> DomainResult<Uuid>;
    async fn update(&self, user: &User) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
    async fn exists_by_email(&self, email: &str) -> DomainResult<bool>;
}

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn generate_access_token(&self, claims: &TokenClaims) -> DomainResult<Token>;
    async fn validate_access_token(&self, token: &str) -> DomainResult<TokenClaims>;
    async fn generate_refresh_token(&self, user_id: Uuid) -> DomainResult<RefreshToken>;
    async fn validate_refresh_token(&self, token: &str) -> DomainResult<RefreshToken>;
    async fn revoke_refresh_token(&self, token: &str) -> DomainResult<()>;
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> DomainResult<()>;
}

#[async_trait]
pub trait RegistrationRepository: Send + Sync {
    async fn register_user(&self, registration: &Registration, password: &str) -> DomainResult<Uuid>;
    async fn verify_email(&self, token: &str) -> DomainResult<()>;
    async fn resend_verification(&self, email: &str) -> DomainResult<()>;
}

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn authenticate(&self, email: &str, password: &str) -> DomainResult<User>;
    async fn change_password(&self, user_id: Uuid, old_password: &str, new_password: &str) -> DomainResult<()>;
    async fn reset_password(&self, email: &str) -> DomainResult<()>;
    async fn confirm_password_reset(&self, token: &str, new_password: &str) -> DomainResult<()>;
}

pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: Box<dyn super::events::DomainEvent>);
}