use crate::application::dto::audit_dto::{AuditLogDto, AuditQueryDto, AuditExportResponse};
use crate::domain::repositories::audit_log_repository::AuditLogRepository;
use crate::domain::entities::audit_log::AuditLogQuery;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    #[error("Unsupported export format: {0}")]
    UnsupportedFormat(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct AuditService {
    audit_repository: Arc<dyn AuditLogRepository>,
}

impl AuditService {
    pub fn new(audit_repository: Arc<dyn AuditLogRepository>) -> Self {
        Self { audit_repository }
    }

    pub async fn query_audit_logs(&self, query: AuditQueryDto) -> Result<Vec<AuditLogDto>, AuditError> {
        info!("Querying audit logs with filters");
        
        // Convert DTO to domain query manually
        let domain_query = AuditLogQuery {
            user_id: query.user_id,
            organisation_id: query.organisation_id,
            action: query.action,
            resource_type: query.resource_type,
            start_date: query.start_date,
            end_date: query.end_date,
            page: query.page,
            page_size: query.page_size,
        };
        
        let logs = self.audit_repository.query(&domain_query)
            .await
            .map_err(|e| AuditError::DatabaseError(e.to_string()))?;
            
        Ok(logs.into_iter().map(Self::to_dto).collect())
    }

    pub async fn export_audit_logs(&self, query: AuditQueryDto, format: &str) -> Result<AuditExportResponse, AuditError> {
        info!("Exporting audit logs in {} format", format);
        
        // Convert DTO to domain query manually
        let domain_query = AuditLogQuery {
            user_id: query.user_id,
            organisation_id: query.organisation_id,
            action: query.action,
            resource_type: query.resource_type,
            start_date: query.start_date,
            end_date: query.end_date,
            page: query.page,
            page_size: query.page_size,
        };
        
        match format.to_lowercase().as_str() {
            "csv" | "json" => {
                let data = self.audit_repository.export(&domain_query)
                    .await
                    .map_err(|e| AuditError::DatabaseError(e.to_string()))?;
                    
                Ok(AuditExportResponse {
                    data,
                    filename: format!("audit_logs.{}", format),
                    content_type: match format {
                        "csv" => "text/csv",
                        "json" => "application/json",
                        _ => "application/octet-stream",
                    }.to_string(),
                })
            }
            _ => Err(AuditError::UnsupportedFormat(format.to_string())),
        }
    }

    fn to_dto(audit_log: crate::domain::entities::audit_log::AuditLog) -> AuditLogDto {
        AuditLogDto {
            id: audit_log.id,
            user_id: audit_log.user_id,
            organisation_id: audit_log.organisation_id,
            action: audit_log.action,
            resource_type: audit_log.resource_type,
            resource_id: audit_log.resource_id,
            details: audit_log.details,
            ip_address: audit_log.ip_address,
            user_agent: audit_log.user_agent,
            timestamp: audit_log.timestamp,
        }
    }
}