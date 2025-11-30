use async_trait::async_trait;
use crate::domain::entities::audit_log::{AuditLog, AuditLogQuery};
use super::RepositoryError;

#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn create(&self, audit_log: &AuditLog) -> Result<i32, RepositoryError>;
    async fn query(&self, query: &AuditLogQuery) -> Result<Vec<AuditLog>, RepositoryError>;
    async fn export(&self, query: &AuditLogQuery) -> Result<Vec<u8>, RepositoryError>; // CSV or JSON export
}