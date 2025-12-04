use crate::user_dto::{AuthDto, CreateUserDto, ErrorResponse};
use crate::user_service::UserService;
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;

pub struct UserHandler {
    service: UserService,
}

impl UserHandler {
    pub fn new(service: UserService) -> Self {
        Self { service }
    }

pub async fn create_user(
    &self,
    user_dto: web::Json<CreateUserDto>,
    client: web::Data<Client>,
) -> HttpResponse {
    match self.service.register_user(user_dto.into_inner(), &client).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => {
            eprintln!("✗ Failed to create user: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Failed to create user: {}", e),
            })
        }
    }
}

pub async fn authenticate_user(
    &self,
    auth_dto: web::Json<AuthDto>,
    client: web::Data<Client>,
) -> HttpResponse {
    match self.service.authenticate_user(auth_dto.into_inner(), &client).await {
        Ok(token_response) => HttpResponse::Ok().json(token_response),
        Err(e) => {
            eprintln!("✗ Authentication failed: {}", e);
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid credentials".to_string(),
            })
        }
    }
}
}

// Route handlers
pub async fn create_user_handler(
    user_dto: web::Json<CreateUserDto>,
    handler: web::Data<UserHandler>,
    client: web::Data<Client>,
) -> impl Responder {
    handler.create_user(user_dto, client).await
}

pub async fn authenticate_handler(
    auth_dto: web::Json<AuthDto>,
    handler: web::Data<UserHandler>,
    client: web::Data<Client>,
) -> impl Responder {
    handler.authenticate_user(auth_dto, client).await
}