use crate::core::errors::AppResult;
use crate::domain::user_entity::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> AppResult<User>;
    async fn find_by_id(&self, user_id: &str) -> AppResult<Option<User>>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>>;
    async fn update(&self, user: &User) -> AppResult<User>;
    async fn delete(&self, user_id: &str) -> AppResult<()>;
    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn count(&self) -> AppResult<i64>;
    async fn find_by_network(&self, network_id: &str, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn find_by_station(&self, station_id: &str, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn find_by_role(&self, role: &str, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn search(&self, query: &str, limit: i64, offset: i64) -> AppResult<Vec<User>>;
}