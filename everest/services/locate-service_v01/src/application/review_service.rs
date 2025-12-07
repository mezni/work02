use std::sync::Arc;
use crate::application::dto::{CreateReviewRequest, UpdateReviewRequest};
use crate::domain::{UserReview, ReviewRepository};
use crate::infrastructure::error::DomainError;
use crate::middleware::Claims;
use crate::utils::generate_review_id;

pub struct ReviewService {
    review_repo: Arc<dyn ReviewRepository>,
}

impl ReviewService {
    pub fn new(review_repo: Arc<dyn ReviewRepository>) -> Self {
        Self { review_repo }
    }

    pub async fn create_review(
        &self,
        request: CreateReviewRequest,
        claims: &Claims,
    ) -> Result<UserReview, DomainError> {
        let review_id = generate_review_id();
        let user_id = claims.get_user_id();

        self.review_repo
            .create(
                &review_id,
                user_id,
                &request.station_id,
                request.rating,
                request.review_text.as_deref(),
                user_id,
            )
            .await
    }

    pub async fn get_station_reviews(
        &self,
        station_id: &str,
    ) -> Result<Vec<UserReview>, DomainError> {
        self.review_repo.find_by_station(station_id).await
    }

    pub async fn get_user_reviews(
        &self,
        user_id: &str,
    ) -> Result<Vec<UserReview>, DomainError> {
        self.review_repo.find_by_user(user_id).await
    }

    pub async fn update_review(
        &self,
        review_id: &str,
        request: UpdateReviewRequest,
        claims: &Claims,
    ) -> Result<UserReview, DomainError> {
        self.review_repo
            .update(
                review_id,
                request.rating,
                request.review_text.as_deref(),
                claims.get_user_id(),
            )
            .await
    }

    pub async fn delete_review(
        &self,
        review_id: &str,
        _claims: &Claims,
    ) -> Result<(), DomainError> {
        self.review_repo.delete(review_id).await
    }
}