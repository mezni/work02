use actix_web::{web, HttpResponse};
use crate::service::organization_service::OrganizationService;
use crate::domain::organization::Organization;
use crate::errors::AppError;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateOrgRequest {
    pub name: String,
    pub address: Option<String>,
}

#[utoipa::path(
    post,
    path = "/org/create",
    request_body = CreateOrgRequest,
    responses(
        (status = 200, description = "Organization created", body = Organization),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_org(
    svc: web::Data<OrganizationService>,
    payload: web::Json<CreateOrgRequest>,
) -> Result<HttpResponse, AppError> {
    let org = svc
        .create_organization(payload.name.clone(), payload.address.clone())
        .await?;
    Ok(HttpResponse::Ok().json(org))
}
