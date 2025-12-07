
use sqlx::PgPool;
use chrono::NaiveDateTime;
use crate::domain::entities::UserReview;

pub struct ReviewRepository {
pool: PgPool,
}

impl ReviewRepository {
pub fn new(pool: PgPool) -> Self {
Self { pool }
}


pub async fn create_review(
    &self,
    review_id: String,
    user_id: String,
    station_id: String,
    rating: i32,
    review_text: Option<String>,
    created_by: String,
) -> Result<UserReview, sqlx::Error> {
    let review = sqlx::query_as::<_, UserReview>(
        r#"
        INSERT INTO user_reviews (
            review_id, user_id, station_id, rating, review_text, created_by
        )
        VALUES ($1,$2,$3,$4,$5,$6)
        RETURNING review_id, user_id, station_id, rating, review_text, created_at, updated_at, created_by, updated_by
        "#
    )
    .bind(review_id)
    .bind(user_id)
    .bind(station_id)
    .bind(rating)
    .bind(review_text)
    .bind(created_by)
    .fetch_one(&self.pool)
    .await?;

    Ok(review)
}

pub async fn get_reviews_by_station(
    &self,
    station_id: String,
) -> Result<Vec<UserReview>, sqlx::Error> {
    let reviews = sqlx::query_as::<_, UserReview>(
        r#"
        SELECT review_id, user_id, station_id, rating, review_text, created_at, updated_at, created_by, updated_by
        FROM user_reviews
        WHERE station_id = $1
        "#
    )
    .bind(station_id)
    .fetch_all(&self.pool)
    .await?;

    Ok(reviews)
}

pub async fn update_review(
    &self,
    review_id: String,
    rating: i32,
    review_text: Option<String>,
    updated_by: String,
) -> Result<UserReview, sqlx::Error> {
    let review = sqlx::query_as::<_, UserReview>(
        r#"
        UPDATE user_reviews
        SET rating = $1,
            review_text = $2,
            updated_at = NOW(),
            updated_by = $3
        WHERE review_id = $4
        RETURNING review_id, user_id, station_id, rating, review_text, created_at, updated_at, created_by, updated_by
        "#
    )
    .bind(rating)
    .bind(review_text)
    .bind(updated_by)
    .bind(review_id)
    .fetch_one(&self.pool)
    .await?;

    Ok(review)
}


}
