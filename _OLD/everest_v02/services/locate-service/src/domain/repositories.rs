use async_trait::async_trait;
use crate::domain::entities::{Station, StationReview};
use crate::infrastructure::error::DomainError;

#[async_trait]
pub trait StationRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Station>, DomainError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Station>, DomainError>;
    async fn find_by_city(&self, city: &str) -> Result<Vec<Station>, DomainError>;
}

#[async_trait]
pub trait ReviewRepository: Send + Sync {
    async fn find_by_station_id(&self, station_id: i32) -> Result<Vec<StationReview>, DomainError>;
    async fn create(&self, station_id: i32, reviewer_name: &str, rating: i32, comment: Option<&str>) -> Result<StationReview, DomainError>;
}