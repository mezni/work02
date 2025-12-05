use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::{StationReview, ReviewRepository};
use crate::infrastructure::error::DomainError;

pub struct PostgresReviewRepository {
    pool: PgPool,
}

impl PostgresReviewRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReviewRepository for PostgresReviewRepository {
    async fn find_by_station_id(&self, station_id: i32) -> Result<Vec<StationReview>, DomainError> {
        let reviews = sqlx::query_as!(
            StationReview,
            r#"
            SELECT 
                review_id as "review_id!",
                station_id as "station_id!",
                reviewer_name as "reviewer_name!",
                rating as "rating!",
                comment,
                created_at as "created_at!"
            FROM station_reviews
            WHERE station_id = $1
            ORDER BY created_at DESC
            "#,
            station_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reviews)
    }

    async fn create(
        &self,
        station_id: i32,
        reviewer_name: &str,
        rating: i32,
        comment: Option<&str>,
    ) -> Result<StationReview, DomainError> {
        if !(1..=5).contains(&rating) {
            return Err(DomainError::ValidationError(
                "Rating must be between 1 and 5".to_string()
            ));
        }

        let review = sqlx::query_as!(
            StationReview,
            r#"
            INSERT INTO station_reviews (station_id, reviewer_name, rating, comment)
            VALUES ($1, $2, $3, $4)
            RETURNING 
                review_id as "review_id!",
                station_id as "station_id!",
                reviewer_name as "reviewer_name!",
                rating as "rating!",
                comment,
                created_at as "created_at!"
            "#,
            station_id,
            reviewer_name,
            rating,
            comment
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(review)
    }
}