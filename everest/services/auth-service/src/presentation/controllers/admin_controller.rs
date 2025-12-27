use crate::AppState;
use crate::application::admin_service::AdminService;
use crate::application::dtos::admin::*;
use crate::core::errors::AppError;
use crate::domain::services::{AdminService as AdminServiceTrait, CreateUserData, UpdateUserData};
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

#[derive(Deserialize, IntoParams)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/users")
            .route("", web::get().to(list_users))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/users",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Users retrieved", body = UserListResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
pub async fn list_users(
    state: web::Data<AppState>,
    query: web::Query<PaginationQuery>,
) -> Result<impl Responder, AppError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let svc = AdminService::new(state.into_inner());
    let users = svc.list_users(limit, offset).await?;

    Ok(HttpResponse::Ok().json(UserListResponse {
        total: users.len(),
        users: users.into_iter().map(UserResponse::from).collect(),
        limit,
        offset,
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/users/{id}",
    params(("id" = String, Path, description = "User UUID")),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
pub async fn get_user(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let svc = AdminService::new(state.into_inner());
    let user = svc.get_user(&path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 409, description = "Conflict - User exists")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
pub async fn create_user(
    state: web::Data<AppState>,
    body: web::Json<CreateUserRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let svc = AdminService::new(state.into_inner());
    let data = CreateUserData {
        email: body.email.clone(),
        username: body.username.clone(),
        password: body.password.clone(),
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        phone: body.phone.clone(),
        role: body.role.clone(),
    };

    let user = svc.create_user(data).await?;
    Ok(HttpResponse::Created().json(UserResponse::from(user)))
}

#[utoipa::path(
    put,
    path = "/api/v1/admin/users/{id}",
    params(("id" = String, Path, description = "User UUID")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
pub async fn update_user(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateUserRequest>,
) -> Result<impl Responder, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let svc = AdminService::new(state.into_inner());
    let data = UpdateUserData {
        email: body.email.clone(),
        username: body.username.clone(),
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        phone: body.phone.clone(),
        role: body.role.clone(),
        is_active: body.is_active,
    };

    let user = svc.update_user(&path.into_inner(), data).await?;
    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/admin/users/{id}",
    params(("id" = String, Path, description = "User UUID")),
    responses(
        (status = 200, description = "User deleted"),
        (status = 404, description = "User not found")
    ),
    tag = "Admin",
    security(("bearer_auth" = []))
)]
pub async fn delete_user(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let svc = AdminService::new(state.into_inner());
    svc.delete_user(&path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User deleted successfully"
    })))
}
