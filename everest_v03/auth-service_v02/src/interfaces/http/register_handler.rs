use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::application::register_service::RegisterService;
use crate::application::user_dto::RegisterRequestDto;
use crate::application::errors::ApplicationError;
use crate::domain::user_repository::UserRepository;
use std::sync::Arc;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "john_doe")]
    pub username: String,

    #[schema(example = "john@example.com")]
    pub email: String,

    #[schema(example = "secret123")]
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    #[schema(example = "success")]
    pub status: &'static str,

    #[schema(example = "User registered successfully")]
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered", body = RegisterResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists")
    ),
    tag = "Authentication"
)]
#[post("/api/v1/register")]
pub async fn register_handler<R: UserRepository + 'static>(
    body: web::Json<RegisterRequest>,
    repo: web::Data<Arc<R>>,
) -> impl Responder {
    let service = RegisterService::new(repo.get_ref().clone());

    let dto = RegisterRequestDto {
        username: body.username.clone(),
        email: body.email.clone(),
        first_name: None,
        last_name: None,
    };

    match service.register_user(dto) {
        Ok(_) => HttpResponse::Created().json(RegisterResponse {
            status: "success",
            message: "User registered successfully".to_string(),
        }),
        Err(ApplicationError::UserAlreadyExists(msg)) => HttpResponse::Conflict().json(RegisterResponse {
            status: "error",
            message: format!("User already exists: {}", msg),
        }),
        Err(ApplicationError::ValidationError(msg)) => HttpResponse::BadRequest().json(RegisterResponse {
            status: "error",
            message: msg,
        }),
        Err(e) => HttpResponse::InternalServerError().json(RegisterResponse {
            status: "error",
            message: format!("Unexpected error: {:?}", e),
        }),
    }
}
