use crate::AppState;
use crate::application::dtos::registration::{RegisterUserRequest, RegisterUserResponse};
use crate::application::registration_service::RegistrationService;
use actix_web::{HttpResponse, Responder, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/registration")
            .route("/register", web::post().to(register_user))
            .route("/verify", web::post().to(verify_registration))
            .route("/verify/resend", web::post().to(resend_verification)),
    );
}

#[utoipa::path(
    post,
    path = "/api/v1/registration/register",
    request_body = RegisterUserRequest,
    responses(
        (status = 200, description = "User registered successfully", body = RegisterUserResponse),
        (status = 400, description = "Bad request")
    ),
    tag = "Registration"
)]
async fn register_user(state: web::Data<AppState>) -> impl Responder {
    let svc = RegistrationService::new(state.into_inner());
    HttpResponse::Ok().json(RegisterUserResponse {
        message: svc.register_user().await,
    })
}

#[utoipa::path(
    post,
    path = "/api/v1/registration/verify",
    responses(
        (status = 200, description = "Account verified", body = String),
        (status = 401, description = "Invalid token")
    ),
    tag = "Registration"
)]
async fn verify_registration(state: web::Data<AppState>) -> impl Responder {
    let svc = RegistrationService::new(state.into_inner());
    HttpResponse::Ok().body(svc.verify_registration().await)
}

#[utoipa::path(
    post,
    path = "/api/v1/registration/verify/resend",
    responses(
        (status = 200, description = "Verification email resent", body = String)
    ),
    tag = "Registration"
)]
async fn resend_verification(state: web::Data<AppState>) -> impl Responder {
    let svc = RegistrationService::new(state.into_inner());
    HttpResponse::Ok().body(svc.resend_verification().await)
}
