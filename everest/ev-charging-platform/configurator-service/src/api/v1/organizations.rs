use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{PgPool, FromRow}; // Add FromRow import
use utoipa::ToSchema;
use uuid::Uuid;

// Struct for the query result
#[derive(FromRow)]
struct OrganizationRecord {
    id: Uuid,
    name: String,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrganizationRequest {
    pub name: String,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct OrganizationResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
}

/// Create a new organization
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

pub async fn create_organization(
    pool: web::Data<PgPool>,
    request: web::Json<CreateOrganizationRequest>,
) -> HttpResponse {
    let system_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    
    match sqlx::query_as::<_, OrganizationRecord>(
        r#"
        INSERT INTO organizations (name, created_by, updated_by)
        VALUES ($1, $2, $3)
        RETURNING id, name, status, created_at
        "#,
    )
    .bind(&request.name)
    .bind(system_user_id)
    .bind(system_user_id)
    .fetch_one(pool.get_ref())
    .await {
        Ok(record) => {
            HttpResponse::Created().json(OrganizationResponse {
                id: record.id.to_string(),
                name: record.name,
                status: record.status,
                created_at: record.created_at.to_rfc3339(),
            })
        }
        Err(e) => {
            eprintln!("Failed to create organization: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create organization"
            }))
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/organizations")
            .route(web::post().to(create_organization))
    );
}





