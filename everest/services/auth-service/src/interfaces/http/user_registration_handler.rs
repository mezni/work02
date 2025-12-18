use crate::AppState;
use crate::application::user_registration_service::UserRegistrationService;
use crate::core::errors::AppError;
use actix_web::{HttpResponse, web};
// Point inward to the application layer
use crate::application::user_registration_dto::{RegisterUserRequest, RegisterUserResponse};

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "Registration created successfully", body = RegisterUserResponse),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal error")
    ),
    tag = "Registration"
)]
pub async fn register_user(
    state: web::Data<AppState>,
    req: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse, AppError> {
    // Pass the DTO to the service
    let response = UserRegistrationService::execute(&state.db, req.into_inner()).await?;

    Ok(HttpResponse::Created().json(response))
}
