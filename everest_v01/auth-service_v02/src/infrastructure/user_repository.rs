// src/infrastructure/user_repository.rs
use async_trait::async_trait;
use crate::domain::user::User;

#[async_trait]
pub trait UserRepository: Clone + Send + Sync + 'static {
    async fn save(&self, user: &User);
}

#[derive(Clone)]
pub struct InMemoryUserRepository;

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) {
        println!("💾 Saving user to in-memory repository:");
        println!("   Username: {}", user.username);
        println!("   Email: {}", user.email);
    }
}