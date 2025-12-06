use crate::application::dto::CreateReviewRequest;
use crate::domain::{StationReview, ReviewRepository, StationRepository};
use crate::infrastructure::error::DomainError;
use std::sync::Arc;

pub struct ReviewService {
    review_repo: Arc<dyn ReviewRepository>,
    station_repo: Arc<dyn StationRepository>,
}

impl ReviewService {
    pub fn new(
        review_repo: Arc<dyn ReviewRepository>,
        station_repo: Arc<dyn StationRepository>,
    ) -> Self {
        Self {
            review_repo,
            station_repo,
        }
    }

    pub async fn create_review(
        &self,
        station_id: i32,
        request: CreateReviewRequest,
    ) -> Result<StationReview, DomainError> {
        // Verify station exists
        self.station_repo
            .find_by_id(station_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Station with id {} not found", station_id)))?;

        self.review_repo
            .create(
                station_id,
                &request.reviewer_name,
                request.rating,
                request.comment.as_deref(),
            )
            .await
    }

    pub async fn get_reviews_for_station(&self, station_id: i32) -> Result<Vec<StationReview>, DomainError> {
        self.review_repo.find_by_station_id(station_id).await
    }
}