// src/interfaces/http/user_handler.rs
use crate::application::register_service::RegisterService;
use crate::application::authenticate_service::AuthenticateService;
use crate::application::user_dto::UserDTO;
use actix_web::{web, HttpResponse, HttpRequest};

pub async fn register(
    register_service: web::Data<RegisterService>,
    user_data: web::Json<RegisterUser>,
) -> HttpResponse {
    let user = register_service.register(user_data.username.clone(), user_data.email.clone(), user_data.password.clone()).await;
    match user {
        Ok(user) => HttpResponse::Created().json(UserDTO::from(user)),
        Err(_) => HttpResponse::InternalServerError().json("Error registering user"),
    }
}

pub async fn authenticate(
    authenticate_service: web::Data<AuthenticateService>,
    user_data: web::Json<AuthenticateUser>,
) -> HttpResponse {
    let user = authenticate_service.authenticate(user_data.username.clone(), user_data.password.clone()).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(UserDTO::from(user)),
        Err(_) => HttpResponse::Unauthorized().json("Invalid credentials"),
    }
}

#[derive(serde::Deserialize)]
struct RegisterUser {
    username: String,
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
struct AuthenticateUser {
    username: String,
    password: String,
}