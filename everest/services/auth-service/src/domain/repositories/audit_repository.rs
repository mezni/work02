use crate::domain::entities::AuditLog;
use crate::domain::enums::AuditAction;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn create(&self, audit_log: &AuditLog) -> Result<(), DomainError>;

    async fn find_by_id(&self, audit_id: &Uuid) -> Result<Option<AuditLog>, DomainError>;

    async fn find_by_user_id(
        &self,
        user_id: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError>;

    async fn find_by_company_id(
        &self,
        company_id: &Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError>;

    async fn find_by_action(
        &self,
        action: AuditAction,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError>;

    async fn search(
        &self,
        user_id: Option<&str>,
        company_id: Option<&Uuid>,
        action: Option<AuditAction>,
        start_date: Option<chrono::DateTime<chrono::Utc>>,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError>;
}
