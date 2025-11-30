use crate::application::dto::organisation_dto::{
    CreateOrganisationDto, UpdateOrganisationDto, AssignUserToOrganisationDto, ErrorResponse,
    SuccessResponse,
};
use crate::application::services::organisation_service::OrganisationError;
use crate::interfaces::AppState;
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};
use validator::Validate;

/// Create a new organisation
#[utoipa::path(
    post,
    path = "/api/v1/organisations",
    request_body = CreateOrganisationDto,
    responses(
        (status = 201, description = "Organisation created successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 409, description = "Organisation name already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organisations"
)]
pub async fn create_organisation(
    state: web::Data<AppState>,
    payload: web::Json<CreateOrganisationDto>,
) -> impl Responder {
    info!("Creating organisation: {}", payload.name);

    // Validate input
    if let Err(e) = payload.validate() {
        error!("Validation error: {}", e);
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation Error".to_string(),
            message: e.to_string(),
        });
    }

    match state
        .organisation_service
        .create_organisation(payload.into_inner())
        .await
    {
        Ok(organisation) => {
            info!("Organisation created successfully: {}", organisation.id);
            HttpResponse::Created().json(organisation)
        }
        Err(OrganisationError::NameExists) => {
            error!("Organisation name already exists");
            HttpResponse::Conflict().json(ErrorResponse {
                error: "Conflict".to_string(),
                message: "Organisation name already exists".to_string(),
            })
        }
        Err(OrganisationError::InvalidData(msg)) => {
            error!("Invalid organisation data: {}", msg);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid Data".to_string(),
                message: msg,
            })
        }
        Err(e) => {
            error!("Failed to create organisation: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Get all organisations
#[utoipa::path(
    get,
    path = "/api/v1/organisations",
    responses(
        (status = 200, description = "List of organisations"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organisations"
)]
pub async fn list_organisations(state: web::Data<AppState>) -> impl Responder {
    info!("Listing all organisations");

    match state.organisation_service.list_organisations().await {
        Ok(organisations) => HttpResponse::Ok().json(organisations),
        Err(e) => {
            error!("Failed to list organisations: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Get a specific organisation
#[utoipa::path(
    get,
    path = "/api/v1/organisations/{id}",
    params(
        ("id" = i32, Path, description = "Organisation ID")
    ),
    responses(
        (status = 200, description = "Organisation details"),
        (status = 404, description = "Organisation not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organisations"
)]
pub async fn get_organisation(
    state: web::Data<AppState>,
    org_id: web::Path<i32>,
) -> impl Responder {
    info!("Getting organisation: {}", org_id);

    match state.organisation_service.get_organisation(*org_id).await {
        Ok(organisation) => HttpResponse::Ok().json(organisation),
        Err(OrganisationError::NotFound) => {
            error!("Organisation not found: {}", org_id);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                message: format!("Organisation with ID {} not found", org_id),
            })
        }
        Err(e) => {
            error!("Failed to get organisation: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Update an organisation
#[utoipa::path(
    put,
    path = "/api/v1/organisations/{id}",
    params(
        ("id" = i32, Path, description = "Organisation ID")
    ),
    request_body = UpdateOrganisationDto,
    responses(
        (status = 200, description = "Organisation updated successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "Organisation not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organisations"
)]
pub async fn update_organisation(
    state: web::Data<AppState>,
    org_id: web::Path<i32>,
    payload: web::Json<UpdateOrganisationDto>,
) -> impl Responder {
    info!("Updating organisation: {}", org_id);

    if let Err(e) = payload.validate() {
        error!("Validation error: {}", e);
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation Error".to_string(),
            message: e.to_string(),
        });
    }

    match state
        .organisation_service
        .update_organisation(*org_id, payload.into_inner())
        .await
    {
        Ok(organisation) => {
            info!("Organisation updated successfully: {}", org_id);
            HttpResponse::Ok().json(organisation)
        }
        Err(OrganisationError::NotFound) => {
            error!("Organisation not found: {}", org_id);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                message: format!("Organisation with ID {} not found", org_id),
            })
        }
        Err(OrganisationError::NameExists) => {
            error!("Organisation name already exists");
            HttpResponse::Conflict().json(ErrorResponse {
                error: "Conflict".to_string(),
                message: "Organisation name already exists".to_string(),
            })
        }
        Err(e) => {
            error!("Failed to update organisation: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Delete an organisation (soft delete)
#[utoipa::path(
    delete,
    path = "/api/v1/organisations/{id}",
    params(
        ("id" = i32, Path, description = "Organisation ID")
    ),
    responses(
        (status = 200, description = "Organisation deleted successfully", body = SuccessResponse),
        (status = 404, description = "Organisation not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organisations"
)]
pub async fn delete_organisation(
    state: web::Data<AppState>,
    org_id: web::Path<i32>,
) -> impl Responder {
    info!("Deleting organisation: {}", org_id);

    match state.organisation_service.delete_organisation(*org_id).await {
        Ok(_) => {
            info!("Organisation deleted successfully: {}", org_id);
            HttpResponse::Ok().json(SuccessResponse {
                message: "Organisation deleted successfully".to_string(),
            })
        }
        Err(OrganisationError::NotFound) => {
            error!("Organisation not found: {}", org_id);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                message: format!("Organisation with ID {} not found", org_id),
            })
        }
        Err(e) => {
            error!("Failed to delete organisation: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Assign a user to an organisation
#[utoipa::path(
    post,
    path = "/api/v1/organisations/assign-user",
    request_body = AssignUserToOrganisationDto,
    responses(
        (status = 200, description = "User assigned to organisation successfully", body = SuccessResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "Organisation or user not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organisations"
)]
pub async fn assign_user_to_organisation(
    state: web::Data<AppState>,
    payload: web::Json<AssignUserToOrganisationDto>,
) -> impl Responder {
    info!(
        "Assigning user {} to organisation {}",
        payload.user_id, payload.organisation_id
    );

    match state
        .organisation_service
        .assign_user_to_organisation(payload.into_inner())
        .await
    {
        Ok(_) => {
            info!("User assigned to organisation successfully");
            HttpResponse::Ok().json(SuccessResponse {
                message: "User assigned to organisation successfully".to_string(),
            })
        }
        Err(OrganisationError::NotFound) => {
            error!("Organisation or user not found");
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Not Found".to_string(),
                message: "Organisation or user not found".to_string(),
            })
        }
        Err(e) => {
            error!("Failed to assign user to organisation: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Server Error".to_string(),
                message: e.to_string(),
            })
        }
    }
}