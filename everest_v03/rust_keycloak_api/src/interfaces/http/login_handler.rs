use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = "secret123")]
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "success")]
    pub status: &'static str,
    #[schema(example = "Logged in successfully")]
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authenticated user", body = LoginResponse)
    ),
    tag = "Authentication"
)]
#[post("/api/v1/auth")]
pub async fn login_handler(body: web::Json<LoginRequest>) -> impl Responder {
    HttpResponse::Ok().json(LoginResponse {
        status: "success",
        message: "Logged in successfully".to_string(),
    })
}
