use crate::core::errors::AppResult;
use crate::domain::audit::Audit;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn create(&self, audit: &Audit) -> AppResult<Audit>;
    async fn find_by_id(&self, audit_id: &str) -> AppResult<Option<Audit>>;
    async fn find_by_user(&self, user_id: &str, limit: i64, offset: i64) -> AppResult<Vec<Audit>>;
    async fn find_by_action(&self, action: &str, limit: i64, offset: i64) -> AppResult<Vec<Audit>>;
    async fn find_by_resource(&self, resource_type: &str, resource_id: &str, limit: i64, offset: i64) -> AppResult<Vec<Audit>>;
    async fn find_by_date_range(&self, start: DateTime<Utc>, end: DateTime<Utc>, limit: i64, offset: i64) -> AppResult<Vec<Audit>>;
    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<Audit>>;
    async fn count(&self) -> AppResult<i64>;
    async fn count_by_user(&self, user_id: &str) -> AppResult<i64>;
    async fn count_by_action(&self, action: &str) -> AppResult<i64>;
    async fn delete_older_than(&self, date: DateTime<Utc>) -> AppResult<u64>;
}

pub struct PostgresAuditRepository {
    pool: PgPool,
}

impl PostgresAuditRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for PostgresAuditRepository {
    async fn create(&self, audit: &Audit) -> AppResult<Audit> {
        let result = sqlx::query_as::<_, Audit>(
            r#"
            INSERT INTO audit_logs (
                audit_id, user_id, action, resource_type, resource_id,
                ip_address, country, city, latitude, longitude,
                user_agent, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
        )
        .bind(&audit.audit_id)
        .bind(&audit.user_id)
        .bind(&audit.action)
        .bind(&audit.resource_type)
        .bind(&audit.resource_id)
        .bind(&audit.ip_address)
        .bind(&audit.country)
        .bind(&audit.city)
        .bind(audit.latitude)
        .bind(audit.longitude)
        .bind(&audit.user_agent)
        .bind(&audit.metadata)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, audit_id: &str) -> AppResult<Option<Audit>> {
        let result = sqlx::query_as::<_, Audit>("SELECT * FROM audit_logs WHERE audit_id = $1")
            .bind(audit_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_user(&self, user_id: &str, limit: i64, offset: i64) -> AppResult<Vec<Audit>> {
        let results = sqlx::query_as::<_, Audit>(
            "SELECT * FROM audit_logs WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn find_by_action(&self, action: &str, limit: i64, offset: i64) -> AppResult<Vec<Audit>> {
        let results = sqlx::query_as::<_, Audit>(
            "SELECT * FROM audit_logs WHERE action = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(action)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn find_by_resource(&self, resource_type: &str, resource_id: &str, limit: i64, offset: i64) -> AppResult<Vec<Audit>> {
        let results = sqlx::query_as::<_, Audit>(
            "SELECT * FROM audit_logs WHERE resource_type = $1 AND resource_id = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4"
        )
        .bind(resource_type)
        .bind(resource_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn find_by_date_range(&self, start: DateTime<Utc>, end: DateTime<Utc>, limit: i64, offset: i64) -> AppResult<Vec<Audit>> {
        let results = sqlx::query_as::<_, Audit>(
            "SELECT * FROM audit_logs WHERE created_at >= $1 AND created_at <= $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4"
        )
        .bind(start)
        .bind(end)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<Audit>> {
        let results = sqlx::query_as::<_, Audit>(
            "SELECT * FROM audit_logs ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn count(&self) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_logs")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    async fn count_by_user(&self, user_id: &str) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    async fn count_by_action(&self, action: &str) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE action = $1")
            .bind(action)
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    async fn delete_older_than(&self, date: DateTime<Utc>) -> AppResult<u64> {
        let result = sqlx::query("DELETE FROM audit_logs WHERE created_at < $1")
            .bind(date)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}