use crate::core::errors::AppResult;
use crate::domain::audit_entity::AuditLog;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn create(&self, audit: &AuditLog) -> AppResult<AuditLog>;
    async fn find_by_id(&self, audit_id: &str) -> AppResult<Option<AuditLog>>;
    async fn find_by_user(&self, user_id: &str, limit: i64, offset: i64) -> AppResult<Vec<AuditLog>>;
    async fn find_by_action(&self, action: &str, limit: i64, offset: i64) -> AppResult<Vec<AuditLog>>;
    async fn find_by_resource(&self, resource_type: &str, resource_id: &str, limit: i64, offset: i64) -> AppResult<Vec<AuditLog>>;
    async fn find_by_date_range(&self, start: DateTime<Utc>, end: DateTime<Utc>, limit: i64, offset: i64) -> AppResult<Vec<AuditLog>>;
    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<AuditLog>>;
    async fn count(&self) -> AppResult<i64>;
    async fn count_by_user(&self, user_id: &str) -> AppResult<i64>;
}