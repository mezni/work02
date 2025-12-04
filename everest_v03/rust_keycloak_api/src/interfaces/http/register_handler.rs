use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
        (status = 201, description = "User registered", body = RegisterResponse)
    ),
    tag = "Authentication"
)]
#[post("/api/v1/register")]
pub async fn register_handler(body: web::Json<RegisterRequest>) -> impl Responder {
    HttpResponse::Created().json(RegisterResponse {
        status: "success",
        message: "User registered successfully".to_string(),
    })
}
