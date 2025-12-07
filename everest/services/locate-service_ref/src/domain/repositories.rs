use async_trait::async_trait;
use crate::domain::entities::*;
use crate::infrastructure::error::DomainError;

#[async_trait]
pub trait StationRepository: Send + Sync {
    async fn find_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        radius_meters: i32,
        limit: i32,
    ) -> Result<Vec<NearbyStation>, DomainError>;
}

#[async_trait]
pub trait ReviewRepository: Send + Sync {
    async fn create(
        &self,
        review_id: &str,
        user_id: &str,
        station_id: &str,
        rating: i32,
        review_text: Option<&str>,
        created_by: &str,
    ) -> Result<UserReview, DomainError>;
    
    async fn find_by_station(&self, station_id: &str) -> Result<Vec<UserReview>, DomainError>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<UserReview>, DomainError>;
    async fn update(
        &self,
        review_id: &str,
        rating: Option<i32>,
        review_text: Option<&str>,
        updated_by: &str,
    ) -> Result<UserReview, DomainError>;
    async fn delete(&self, review_id: &str) -> Result<(), DomainError>;
}