use actix_web::{web, HttpResponse};
use utoipa::{ToSchema, IntoParams};
use crate::application::services::user_app_service::UserAppService;
use crate::application::dtos::requests::CreateUserRequest;
use crate::domain::models::user::User;
use crate::infrastructure::keycloak::client::KeycloakAuthClient;

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "User already exists")
    )
)]
pub async fn create_user(
    service: web::Data<UserAppService<KeycloakAuthClient>>,
    request: web::Json<CreateUserRequest>,
) -> HttpResponse {
    match service.create_user(request.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            if e.to_string().to_lowercase().contains("exists") || e.to_string().contains("409") {
                HttpResponse::Conflict().body(e.to_string())
            } else {
                HttpResponse::BadRequest().body(e.to_string())
            }
        }
    }
}