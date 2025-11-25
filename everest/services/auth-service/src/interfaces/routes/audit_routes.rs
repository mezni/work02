use actix_web::{web, HttpResponse};
use uuid::Uuid;
use utoipa::OpenApi;
use crate::interfaces::controllers::audit_controller::*;
use crate::application::dto::AuditSearchRequest;

#[utoipa::path(
    get,
    path = "/audit/logs",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Page size"),
        ("user_id" = Option<String>, Query, description = "Filter by user ID"),
        ("company_id" = Option<Uuid>, Query, description = "Filter by company ID"),
        ("action" = Option<String>, Query, description = "Filter by action"),
        ("start_date" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("end_date" = Option<String>, Query, description = "End date (ISO 8601)")
    ),
    responses(
        (status = 200, description = "Audit logs retrieved", body = AuditLogListResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_audit_logs(
    controller: web::Data<AuditController>,
    query: web::Query<AuditSearchRequest>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.get_audit_logs(query.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/audit/users/{user_id}",
    params(
        ("user_id" = String, Path, description = "User ID"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "User audit logs retrieved", body = AuditLogListResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user_audit_logs(
    controller: web::Data<AuditController>,
    path: web::Path<String>,
    query: web::Query<UserAuditQuery>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.get_user_audit_logs(path.into_inner(), query.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/audit/companies/{company_id}",
    params(
        ("company_id" = Uuid, Path, description = "Company ID"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Company audit logs retrieved", body = AuditLogListResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_company_audit_logs(
    controller: web::Data<AuditController>,
    path: web::Path<Uuid>,
    query: web::Query<CompanyAuditQuery>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.get_company_audit_logs(path.into_inner(), query.into_inner(), user_id.into_inner()).await
}

#[derive(serde::Deserialize)]
pub struct UserAuditQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(serde::Deserialize)]
pub struct CompanyAuditQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

pub fn configure_audit_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/audit")
            .service(
                web::resource("/logs")
                    .route(web::get().to(get_audit_logs))
            )
            .service(
                web::resource("/users/{user_id}")
                    .route(web::get().to(get_user_audit_logs))
            )
            .service(
                web::resource("/companies/{company_id}")
                    .route(web::get().to(get_company_audit_logs))
            )
    );
}