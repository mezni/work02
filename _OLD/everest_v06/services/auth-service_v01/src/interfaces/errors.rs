use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use tracing::error;
use crate::application::errors::ApplicationError;
use crate::domain::errors::DomainError;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub error_code: String,
    pub message: String,
    pub timestamp: String,
    pub path: String,
}

impl ErrorResponse {
    pub fn new(error: &str, error_code: &str, message: &str, path: &str) -> Self {
        Self {
            success: false,
            error: error.to_string(),
            error_code: error_code.to_string(),
            message: message.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            path: path.to_string(),
        }
    }
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        let (status, error, message) = match self {
            ApplicationError::Domain(domain_error) => match domain_error {
                DomainError::Validation(_) => (actix_web::http::StatusCode::BAD_REQUEST, "validation_error", domain_error.to_string()),
                DomainError::UserNotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "user_not_found", domain_error.to_string()),
                DomainError::CompanyNotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "company_not_found", domain_error.to_string()),
                DomainError::EmailAlreadyExists(_) => (actix_web::http::StatusCode::CONFLICT, "email_already_exists", domain_error.to_string()),
                DomainError::UsernameAlreadyExists(_) => (actix_web::http::StatusCode::CONFLICT, "username_already_exists", domain_error.to_string()),
                DomainError::Unauthorized(_) => (actix_web::http::StatusCode::FORBIDDEN, "unauthorized", domain_error.to_string()),
                _ => (actix_web::http::StatusCode::BAD_REQUEST, "domain_error", domain_error.to_string()),
            },
            ApplicationError::Infrastructure(infra_error) => match infra_error {
                InfrastructureError::Authentication(_) => (actix_web::http::StatusCode::UNAUTHORIZED, "authentication_error", infra_error.to_string()),
                InfrastructureError::Authorization(_) => (actix_web::http::StatusCode::FORBIDDEN, "authorization_error", infra_error.to_string()),
                InfrastructureError::Keycloak(_) => (actix_web::http::StatusCode::BAD_GATEWAY, "external_service_error", "Authentication service unavailable".to_string()),
                _ => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "internal_error", "Internal server error".to_string()),
            },
            ApplicationError::Validation(_) => (actix_web::http::StatusCode::BAD_REQUEST, "validation_error", self.to_string()),
            ApplicationError::Authentication(_) => (actix_web::http::StatusCode::UNAUTHORIZED, "authentication_error", self.to_string()),
            ApplicationError::Authorization(_) => (actix_web::http::StatusCode::FORBIDDEN, "authorization_error", self.to_string()),
            ApplicationError::UserNotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "user_not_found", self.to_string()),
            ApplicationError::CompanyNotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "company_not_found", self.to_string()),
            ApplicationError::InvalidInput(_) => (actix_web::http::StatusCode::BAD_REQUEST, "invalid_input", self.to_string()),
            ApplicationError::BusinessRuleViolation(_) => (actix_web::http::StatusCode::CONFLICT, "business_rule_violation", self.to_string()),
        };

        error!("Application error: {} - {}", self.code(), self);

        let error_response = ErrorResponse::new(
            error,
            self.code(),
            &message,
            "", // Path would be set by middleware
        );

        HttpResponse::build(status).json(error_response)
    }
}

// Implement From traits for easy conversion
impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        ApplicationError::Domain(error)
    }
}

impl From<InfrastructureError> for ApplicationError {
    fn from(error: InfrastructureError) -> Self {
        ApplicationError::Infrastructure(error)
    }
}