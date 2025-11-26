use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entities::AuditLog;
use crate::domain::enums::AuditAction;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn create(&self, audit_log: &AuditLog) -> Result<(), DomainError>;

    async fn find_by_id(&self, audit_id: Uuid) -> Result<Option<AuditLog>, DomainError>;

    async fn find_by_user_id(
        &self,
        user_id: String, // Changed from &str to String
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError>;

    async fn find_by_company_id(
        &self,
        company_id: Uuid, // Changed from &Uuid to Uuid
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
        user_id: Option<String>,  // Changed from Option<&str>
        company_id: Option<Uuid>, // Changed from Option<&Uuid>
        action: Option<AuditAction>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError>;
}
