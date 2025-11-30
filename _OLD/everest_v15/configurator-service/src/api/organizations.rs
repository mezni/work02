// configurator-service/src/api/organizations.rs
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    application::{
        ApplicationError,
        commands::CreateOrganizationCommand,
        dtos::{OrganizationDto, PaginatedResponse},
    },
    infrastructure::repositories::RepositoryFactory,
};

// ... your existing DTOs ...

/// Create a new organization
#[utoipa::path(
    post,
    path = "/api/v1/organizations",
    request_body = CreateOrganizationRequest,
    responses(
        (status = 201, description = "Organization created successfully", body = OrganizationResponse),
        (status = 400, description = "Invalid input data", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
pub async fn create_organization(
    pool: web::Data<sqlx::PgPool>,
    request: web::Json<CreateOrganizationRequest>,
) -> HttpResponse {
    let repository_factory = RepositoryFactory::new(pool.get_ref().clone());
    let organization_service = repository_factory.organization_service();

    // Use system user ID for now (will be replaced with actual auth later)
    let system_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    let command = CreateOrganizationCommand {
        name: request.name.clone(),
        created_by: system_user_id,
    };

    match organization_service.create_organization(command).await {
        Ok(organization_dto) => HttpResponse::Created().json(OrganizationResponse {
            id: organization_dto.id.to_string(),
            name: organization_dto.name,
            status: organization_dto.status.to_string(),
            created_at: organization_dto.created_at.to_rfc3339(),
        }),
        Err(ApplicationError::DomainValidation(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: msg,
            })
        }
        Err(ApplicationError::OrganizationNameAlreadyExists(name)) => HttpResponse::BadRequest()
            .json(ErrorResponse {
                code: "DUPLICATE_ORGANIZATION".to_string(),
                message: format!("An organization with name '{}' already exists", name),
            }),
        Err(e) => {
            eprintln!("Failed to create organization: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                code: "INTERNAL_ERROR".to_string(),
                message: "Failed to create organization".to_string(),
            })
        }
    }
}

/// Get all organizations
///
/// Returns a list of all organizations in the system with pagination support.
#[utoipa::path(
    get,
    path = "/api/v1/organizations",
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u32>, Query, description = "Items per page (default: 20, max: 100)")
    ),
    responses(
        (status = 200, description = "List of organizations retrieved successfully", body = OrganizationListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
pub async fn get_organizations(
    pool: web::Data<PgPool>,
    query: web::Query<PaginatedQuery>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    match sqlx::query(
        r#"
        SELECT id, name, status, created_at
        FROM organizations 
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => {
            let organizations: Vec<OrganizationResponse> = rows
                .into_iter()
                .map(|row| {
                    let id: Uuid = row.get("id");
                    let name: String = row.get("name");
                    let status: String = row.get("status");
                    let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

                    OrganizationResponse {
                        id: id.to_string(),
                        name,
                        status,
                        created_at: created_at.to_rfc3339(),
                    }
                })
                .collect();

            // Get total count (simplified - in production you might want a more efficient count)
            let total = organizations.len();

            HttpResponse::Ok().json(OrganizationListResponse {
                organizations,
                total,
            })
        }
        Err(e) => {
            eprintln!("Failed to fetch organizations: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                code: "INTERNAL_ERROR".to_string(),
                message: "Failed to fetch organizations".to_string(),
            })
        }
    }
}

/// Get organization by ID
///
/// Returns detailed information about a specific organization.
#[utoipa::path(
    get,
    path = "/api/v1/organizations/{id}",
    params(
        ("id" = String, Path, description = "Organization UUID")
    ),
    responses(
        (status = 200, description = "Organization retrieved successfully", body = OrganizationResponse),
        (status = 404, description = "Organization not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
pub async fn get_organization(pool: web::Data<PgPool>, path: web::Path<String>) -> HttpResponse {
    let organization_id = path.into_inner();

    let id = match Uuid::parse_str(&organization_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                code: "INVALID_UUID".to_string(),
                message: "Invalid organization ID format".to_string(),
            });
        }
    };

    match sqlx::query(
        r#"
        SELECT id, name, status, created_at
        FROM organizations 
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(row)) => {
            let id: Uuid = row.get("id");
            let name: String = row.get("name");
            let status: String = row.get("status");
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

            HttpResponse::Ok().json(OrganizationResponse {
                id: id.to_string(),
                name,
                status,
                created_at: created_at.to_rfc3339(),
            })
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            code: "ORGANIZATION_NOT_FOUND".to_string(),
            message: "Organization not found".to_string(),
        }),
        Err(e) => {
            eprintln!("Failed to fetch organization: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                code: "INTERNAL_ERROR".to_string(),
                message: "Failed to fetch organization".to_string(),
            })
        }
    }
}

// Query parameters for pagination
#[derive(Debug, Deserialize)]
pub struct PaginatedQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/organizations")
            .service(web::resource("").route(web::get().to(get_organizations)))
            .service(web::resource("").route(web::post().to(create_organization)))
            .service(web::resource("/{id}").route(web::get().to(get_organization))),
    );
}
