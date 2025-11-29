use crate::application::dto::audit_dto::{AuditQueryDto, AuditExportResponse, ErrorResponse};
use crate::application::services::audit_service::AuditError;
use crate::interfaces::AppState;
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};

/// Get audit logs with filtering
#[utoipa::path(
    get,
    path = "/api/v1/audit/logs",
    params(
        ("user_id" = Option<String>, Query, description = "Filter by user ID"),
        ("organisation_id" = Option<i32>, Query, description = "Filter by organisation ID"),
        ("action" = Option<String>, Query, description = "Filter by action type"),
        ("resource_type" = Option<String>, Query, description = "Filter by resource type"),
        ("start_date" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("end_date" = Option<String>, Query, description = "End date (ISO 8601)"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "List of audit logs"),
        (status = 400, description = "Invalid query parameters", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Audit"
)]
pub async fn get_audit_logs(
    state: web::Data<AppState>,
    query: web::Query<AuditQueryDto>,
) -> impl Responder {
    info!("Querying audit logs");

    match state.audit_service.query_audit_logs(query.into_inner()).await {
        Ok(logs) => {
            info!("Retrieved {} audit logs", logs.len());
            HttpResponse::Ok().json(logs)
        }
        Err(AuditError::InvalidQuery(msg)) => {
            error!("Invalid audit query: {}", msg);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid Query".to_string(),
                message: msg,
            })
        }
        Err(e) => {
            error!("Failed to query audit logs: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Export audit logs
#[utoipa::path(
    get,
    path = "/api/v1/audit/logs/export",
    params(
        ("user_id" = Option<String>, Query, description = "Filter by user ID"),
        ("organisation_id" = Option<i32>, Query, description = "Filter by organisation ID"),
        ("action" = Option<String>, Query, description = "Filter by action type"),
        ("resource_type" = Option<String>, Query, description = "Filter by resource type"),
        ("start_date" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("end_date" = Option<String>, Query, description = "End date (ISO 8601)"),
        ("format" = Option<String>, Query, description = "Export format (csv, json)")
    ),
    responses(
        (status = 200, description = "Audit logs exported successfully"),
        (status = 400, description = "Invalid query parameters", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Audit"
)]
pub async fn export_audit_logs(
    state: web::Data<AppState>,
    query: web::Query<AuditQueryDto>,
) -> impl Responder {
    info!("Exporting audit logs");

    let format = query.format.clone().unwrap_or_else(|| "csv".to_string());

    match state.audit_service.export_audit_logs(query.into_inner(), &format).await {
        Ok(export_data) => {
            info!("Audit logs exported successfully in {} format", format);
            
            let content_type = match format.as_str() {
                "csv" => "text/csv",
                "json" => "application/json",
                _ => "application/octet-stream",
            };

            let filename = match format.as_str() {
                "csv" => "audit_logs.csv",
                "json" => "audit_logs.json",
                _ => "audit_logs.export",
            };

            HttpResponse::Ok()
                .content_type(content_type)
                .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
                .body(export_data.data)
        }
        Err(AuditError::InvalidQuery(msg)) => {
            error!("Invalid audit query for export: {}", msg);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid Query".to_string(),
                message: msg,
            })
        }
        Err(AuditError::UnsupportedFormat(format)) => {
            error!("Unsupported export format: {}", format);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Unsupported Format".to_string(),
                message: format!("Export format '{}' is not supported", format),
            })
        }
        Err(e) => {
            error!("Failed to export audit logs: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}