use crate::domain::{Station, StationRepository, StationWithReviews, ReviewRepository};
use crate::infrastructure::error::DomainError;
use std::sync::Arc;

pub struct StationService {
    station_repo: Arc<dyn StationRepository>,
    review_repo: Arc<dyn ReviewRepository>,
}

impl StationService {
    pub fn new(
        station_repo: Arc<dyn StationRepository>,
        review_repo: Arc<dyn ReviewRepository>,
    ) -> Self {
        Self {
            station_repo,
            review_repo,
        }
    }

    pub async fn get_all_stations(&self) -> Result<Vec<Station>, DomainError> {
        self.station_repo.find_all().await
    }

    pub async fn get_station_by_id(&self, id: i32) -> Result<Station, DomainError> {
        self.station_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Station with id {} not found", id)))
    }

    pub async fn get_station_with_reviews(&self, id: i32) -> Result<StationWithReviews, DomainError> {
        let station = self.get_station_by_id(id).await?;
        let reviews = self.review_repo.find_by_station_id(id).await?;
        
        let average_rating = if reviews.is_empty() {
            None
        } else {
            let sum: i32 = reviews.iter().map(|r| r.rating).sum();
            Some(sum as f64 / reviews.len() as f64)
        };

        Ok(StationWithReviews {
            station,
            reviews,
            average_rating,
        })
    }

    pub async fn get_stations_by_city(&self, city: &str) -> Result<Vec<Station>, DomainError> {
        self.station_repo.find_by_city(city).await
    }
}