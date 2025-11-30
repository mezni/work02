// configurator-service/src/api/v1/organizations.rs
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrganizationRequest {
    pub name: String,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct OrganizationResponse {
    pub id: String,
    pub name: String,
    pub status: String,
}

/// Create a new organization
///
/// This endpoint allows super admins to create new organizations in the system.
/// Each organization can own multiple charging stations and have partners assigned to it.
#[utoipa::path(
    post,
    path = "/organizations",
    request_body = CreateOrganizationRequest,
    responses(
        (status = 201, description = "Organization created successfully", body = OrganizationResponse),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires super admin role")
    ),
    tag = "Organizations"
)]
pub async fn create_organization(request: web::Json<CreateOrganizationRequest>) -> HttpResponse {
    // TODO: Implement actual organization creation logic
    // For now, return a mock response

    HttpResponse::Created().json(OrganizationResponse {
        id: uuid::Uuid::new_v4().to_string(),
        name: request.name.clone(),
        status: "active".to_string(),
    })
}

// Configure function for organizations module
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/organizations").route(web::post().to(create_organization)));
}
