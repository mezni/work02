use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::{UserReview, ReviewRepository};
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
    async fn create(
        &self,
        review_id: &str,
        user_id: &str,
        station_id: &str,
        rating: i32,
        review_text: Option<&str>,
        created_by: &str,
    ) -> Result<UserReview, DomainError> {
        let review = sqlx::query_as!(
            UserReview,
            r#"
            INSERT INTO user_reviews (
                review_id, user_id, station_id, rating, review_text, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING 
                review_id as "review_id!",
                user_id as "user_id!",
                station_id as "station_id!",
                rating as "rating!",
                review_text,
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by as "created_by!",
                updated_by
            "#,
            review_id,
            user_id,
            station_id,
            rating,
            review_text,
            created_by
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(review)
    }

    async fn find_by_station(&self, station_id: &str) -> Result<Vec<UserReview>, DomainError> {
        let reviews = sqlx::query_as!(
            UserReview,
            r#"
            SELECT 
                review_id as "review_id!",
                user_id as "user_id!",
                station_id as "station_id!",
                rating as "rating!",
                review_text,
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by as "created_by!",
                updated_by
            FROM user_reviews
            WHERE station_id = $1
            ORDER BY created_at DESC
            "#,
            station_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reviews)
    }

    async fn find_by_user(&self, user_id: &str) -> Result<Vec<UserReview>, DomainError> {
        let reviews = sqlx::query_as!(
            UserReview,
            r#"
            SELECT 
                review_id as "review_id!",
                user_id as "user_id!",
                station_id as "station_id!",
                rating as "rating!",
                review_text,
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by as "created_by!",
                updated_by
            FROM user_reviews
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reviews)
    }

    async fn update(
        &self,
        review_id: &str,
        rating: Option<i32>,
        review_text: Option<&str>,
        updated_by: &str,
    ) -> Result<UserReview, DomainError> {
        let review = sqlx::query_as!(
            UserReview,
            r#"
            UPDATE user_reviews
            SET 
                rating = COALESCE($2, rating),
                review_text = COALESCE($3, review_text),
                updated_by = $4,
                updated_at = NOW()
            WHERE review_id = $1
            RETURNING 
                review_id as "review_id!",
                user_id as "user_id!",
                station_id as "station_id!",
                rating as "rating!",
                review_text,
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by as "created_by!",
                updated_by
            "#,
            review_id,
            rating,
            review_text,
            updated_by
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(review)
    }

    async fn delete(&self, review_id: &str) -> Result<(), DomainError> {
        sqlx::query!(
            "DELETE FROM user_reviews WHERE review_id = $1",
            review_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}