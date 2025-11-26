use crate::domain::entities::AuditLog;
use crate::domain::repositories::AuditLogRepository;
use crate::domain::errors::DomainError;

pub struct AuditService {
    audit_repository: Box<dyn AuditLogRepository>,
}

impl AuditService {
    pub fn new(audit_repository: Box<dyn AuditLogRepository>) -> Self {
        Self { audit_repository }
    }
    
    pub async fn log_event(
        &self,
        user_id: Option<uuid::Uuid>,
        action: crate::domain::enums::AuditAction,
        resource_type: String,
        resource_id: Option<String>,
        details: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), DomainError> {
        let audit_log = AuditLog::new(
            user_id,
            action,
            resource_type,
            resource_id,
            details,
            ip_address,
            user_agent,
        );
        
        self.audit_repository.create(&audit_log).await
    }
}
