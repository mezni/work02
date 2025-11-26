use actix_web::{HttpResponse, web};
use tracing::{info, error};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::application::dto::{AuditLogDto, AuditLogListResponse, AuditSearchRequest};
use crate::application::handlers::query_handlers::AuditQueryHandler;
use crate::common::response::ApiResponse;
use crate::domain::repositories::{AuditRepository, UserRepository};
use crate::domain::enums::AuditAction;

pub struct AuditController<T: AuditRepository, U: UserRepository> {
    audit_query_handler: AuditQueryHandler<T, U>,
}

impl<T: AuditRepository, U: UserRepository> AuditController<T, U> {
    pub fn new(audit_query_handler: AuditQueryHandler<T, U>) -> Self {
        Self {
            audit_query_handler,
        }
    }

    pub async fn get_audit_logs(
        &self,
        request: AuditSearchRequest,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Getting audit logs");

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let page = request.page.unwrap_or(1);
        let page_size = request.page_size.unwrap_or(20);

        let query = crate::application::queries::GetAuditLogsQuery {
            user_id: request.user_id,
            company_id: request.company_id,
            action: request.action,
            start_date: request.start_date,
            end_date: request.end_date,
            page,
            page_size,
            requested_by: requester_uuid,
        };

        let logs = self.audit_query_handler.handle_get_audit_logs(query).await?;
        let log_dtos: Vec<AuditLogDto> = logs.into_iter().map(AuditLogDto::from).collect();

        let response = AuditLogListResponse {
            logs: log_dtos,
            total: logs.len() as u64,
            page,
            page_size,
            total_pages: (logs.len() as f64 / page_size as f64).ceil() as u32,
        };

        let api_response = ApiResponse::success(response, "Audit logs retrieved successfully");
        Ok(HttpResponse::Ok().json(api_response))
    }

    pub async fn get_user_audit_logs(
        &self,
        user_id: String,
        query: crate::interfaces::routes::audit_routes::UserAuditQuery,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Getting audit logs for user: {}", user_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(20);

        let audit_query = crate::application::queries::GetUserAuditLogsQuery {
            user_id,
            page,
            page_size,
            requested_by: requester_uuid,
        };

        let logs = self.audit_query_handler.handle_get_user_audit_logs(audit_query).await?;
        let log_dtos: Vec<AuditLogDto> = logs.into_iter().map(AuditLogDto::from).collect();

        let response = AuditLogListResponse {
            logs: log_dtos,
            total: logs.len() as u64,
            page,
            page_size,
            total_pages: (logs.len() as f64 / page_size as f64).ceil() as u32,
        };

        let api_response = ApiResponse::success(response, "User audit logs retrieved successfully");
        Ok(HttpResponse::Ok().json(api_response))
    }

    pub async fn get_company_audit_logs(
        &self,
        company_id: Uuid,
        query: crate::interfaces::routes::audit_routes::CompanyAuditQuery,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Getting audit logs for company: {}", company_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(20);

        let audit_query = crate::application::queries::GetCompanyAuditLogsQuery {
            company_id,
            page,
            page_size,
            requested_by: requester_uuid,
        };

        let logs = self.audit_query_handler.handle_get_company_audit_logs(audit_query).await?;
        let log_dtos: Vec<AuditLogDto> = logs.into_iter().map(AuditLogDto::from).collect();

        let response = AuditLogListResponse {
            logs: log_dtos,
            total: logs.len() as u64,
            page,
            page_size,
            total_pages: (logs.len() as f64 / page_size as f64).ceil() as u32,
        };

        let api_response = ApiResponse::success(response, "Company audit logs retrieved successfully");
        Ok(HttpResponse::Ok().json(api_response))
    }
}