use tracing::{info, error};
use uuid::Uuid;
use crate::domain::entities::AuditLog;
use crate::domain::enums::AuditAction;
use crate::domain::repositories::AuditRepository;
use crate::domain::errors::DomainError;

pub struct Auditor<T: AuditRepository> {
    audit_repository: T,
}

impl<T: AuditRepository> Auditor<T> {
    pub fn new(audit_repository: T) -> Self {
        Self { audit_repository }
    }

    pub async fn log_event(
        &self,
        action: AuditAction,
        user_id: Option<String>,
        user_email: Option<String>,
        user_role: Option<String>,
        company_id: Option<Uuid>,
        resource_type: String,
        resource_id: Option<String>,
        old_values: Option<serde_json::Value>,
        new_values: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        request_path: String,
        request_method: String,
        status_code: u16,
        error_message: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> Result<(), DomainError> {
        let audit_log = AuditLog::new(
            action,
            user_id,
            user_email,
            user_role,
            company_id,
            resource_type,
            resource_id,
            old_values,
            new_values,
            ip_address,
            user_agent,
            request_path,
            request_method,
            status_code,
            error_message,
            metadata,
        );

        self.audit_repository.create(&audit_log).await?;
        info!("Audit event logged: {:?}", audit_log.action);
        
        Ok(())
    }

    // Convenience methods for common audit events
    pub async fn log_user_login(
        &self,
        user_id: String,
        user_email: String,
        user_role: String,
        company_id: Option<Uuid>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), DomainError> {
        self.log_event(
            AuditAction::UserLoggedIn,
            Some(user_id),
            Some(user_email),
            Some(user_role),
            company_id,
            "user".to_string(),
            None,
            None,
            None,
            ip_address,
            user_agent,
            "/api/v1/auth/login".to_string(),
            "POST".to_string(),
            200,
            None,
            None,
        ).await
    }

    pub async fn log_user_registration(
        &self,
        user_id: String,
        user_email: String,
        user_role: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), DomainError> {
        self.log_event(
            AuditAction::UserRegistered,
            Some(user_id),
            Some(user_email),
            Some(user_role),
            None,
            "user".to_string(),
            Some(user_id.clone()),
            None,
            None,
            ip_address,
            user_agent,
            "/api/v1/auth/register".to_string(),
            "POST".to_string(),
            201,
            None,
            None,
        ).await
    }

    pub async fn log_user_role_change(
        &self,
        user_id: String,
        target_user_id: String,
        old_role: String,
        new_role: String,
        changed_by: String,
        company_id: Option<Uuid>,
    ) -> Result<(), DomainError> {
        self.log_event(
            AuditAction::UserRoleChanged,
            Some(changed_by),
            None,
            None,
            company_id,
            "user".to_string(),
            Some(target_user_id),
            Some(serde_json::json!({ "role": old_role })),
            Some(serde_json::json!({ "role": new_role })),
            None,
            None,
            format!("/api/v1/users/{}/roles", user_id),
            "POST".to_string(),
            200,
            None,
            None,
        ).await
    }

    pub async fn log_unauthorized_access(
        &self,
        user_id: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        request_path: String,
        request_method: String,
    ) -> Result<(), DomainError> {
        self.log_event(
            AuditAction::UnauthorizedAccessAttempt,
            user_id,
            None,
            None,
            None,
            "system".to_string(),
            None,
            None,
            None,
            ip_address,
            user_agent,
            request_path,
            request_method,
            401,
            Some("Unauthorized access attempt".to_string()),
            None,
        ).await
    }
}