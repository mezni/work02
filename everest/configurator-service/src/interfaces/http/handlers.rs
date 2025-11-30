use crate::application::services::AppService;
use crate::error::AppError;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use tracing::info;

pub async fn hello() -> Result<HttpResponse, AppError> {
    let service = AppService;
    let app_info = service.get_app_info()?;

    info!(
        "Hello endpoint called - App: {} v{}",
        app_info.name, app_info.version
    );
    Ok(HttpResponse::Ok().body(format!(
        "Hello from {} v{}!",
        app_info.name, app_info.version
    )))
}

pub async fn health_check() -> Result<HttpResponse, AppError> {
    let service = AppService;
    let app_info = service.get_app_info()?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "app": app_info.name,
        "version": app_info.version
    })))
}

pub async fn get_user(user_id: web::Path<u64>) -> Result<HttpResponse, AppError> {
    let service = AppService;
    let user = service.get_user(*user_id)?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn create_user(
    user_data: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let service = AppService;
    let user = service.create_user(user_data.username.clone(), user_data.email.clone())?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn update_app_name(new_name: web::Json<String>) -> Result<HttpResponse, AppError> {
    let service = AppService;
    let result = service.update_app_name(new_name.into_inner())?;

    Ok(HttpResponse::Ok().body(result))
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}
