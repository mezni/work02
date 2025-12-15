use crate::core::errors::AppResult;
use crate::domain::events::OutboxEvent;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[async_trait]
pub trait OutboxRepository: Send + Sync {
    async fn create(&self, event: &OutboxEvent) -> AppResult<OutboxEvent>;
    async fn find_by_id(&self, event_id: &str) -> AppResult<Option<OutboxEvent>>;
    async fn get_unpublished(&self, limit: i64) -> AppResult<Vec<OutboxEvent>>;
    async fn get_unpublished_by_aggregate(&self, aggregate_id: &str, limit: i64) -> AppResult<Vec<OutboxEvent>>;
    async fn mark_published(&self, event_id: &str) -> AppResult<()>;
    async fn mark_multiple_published(&self, event_ids: &[String]) -> AppResult<u64>;
    async fn delete_old(&self, before: DateTime<Utc>) -> AppResult<u64>;
    async fn count_unpublished(&self) -> AppResult<i64>;
    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<OutboxEvent>>;
}


pub struct PostgresOutboxRepository {
    pub(crate) pool: PgPool,  // Changed from private
}

impl PostgresOutboxRepository {
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait]
impl OutboxRepository for PostgresOutboxRepository {
    async fn create(&self, event: &OutboxEvent) -> AppResult<OutboxEvent> {
        let result = sqlx::query_as::<_, OutboxEvent>(
            r#"
            INSERT INTO outbox_events (
                event_id, event_type, aggregate_id, payload, published
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(&event.event_id)
        .bind(&event.event_type)
        .bind(&event.aggregate_id)
        .bind(&event.payload)
        .bind(event.published)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, event_id: &str) -> AppResult<Option<OutboxEvent>> {
        let result = sqlx::query_as::<_, OutboxEvent>(
            "SELECT * FROM outbox_events WHERE event_id = $1"
        )
        .bind(event_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_unpublished(&self, limit: i64) -> AppResult<Vec<OutboxEvent>> {
        let results = sqlx::query_as::<_, OutboxEvent>(
            r#"
            SELECT * FROM outbox_events 
            WHERE published = FALSE 
            ORDER BY created_at ASC 
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn get_unpublished_by_aggregate(&self, aggregate_id: &str, limit: i64) -> AppResult<Vec<OutboxEvent>> {
        let results = sqlx::query_as::<_, OutboxEvent>(
            r#"
            SELECT * FROM outbox_events 
            WHERE aggregate_id = $1 AND published = FALSE 
            ORDER BY created_at ASC 
            LIMIT $2
            "#
        )
        .bind(aggregate_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn mark_published(&self, event_id: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE outbox_events 
            SET published = TRUE, published_at = NOW() 
            WHERE event_id = $1
            "#
        )
        .bind(event_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn mark_multiple_published(&self, event_ids: &[String]) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            UPDATE outbox_events 
            SET published = TRUE, published_at = NOW() 
            WHERE event_id = ANY($1)
            "#
        )
        .bind(event_ids)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn delete_old(&self, before: DateTime<Utc>) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM outbox_events 
            WHERE published = TRUE AND published_at < $1
            "#
        )
        .bind(before)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn count_unpublished(&self) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM outbox_events WHERE published = FALSE"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<OutboxEvent>> {
        let results = sqlx::query_as::<_, OutboxEvent>(
            "SELECT * FROM outbox_events ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }
}