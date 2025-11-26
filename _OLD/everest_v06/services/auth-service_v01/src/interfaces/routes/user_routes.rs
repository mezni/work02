use actix_web::{web, HttpResponse};
use uuid::Uuid;
use utoipa::OpenApi;
use crate::interfaces::controllers::user_controller::*;
use crate::application::dto::{CreateUserRequest, UpdateUserRequest, ChangeUserRoleRequest, AssignUserToCompanyRequest, UpdateProfileRequest, ChangePasswordRequest};

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserDto),
        (status = 400, description = "Invalid input"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_user(
    controller: web::Data<UserController>,
    request: web::Json<CreateUserRequest>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.create_user(request.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserDto),
        (status = 404, description = "User not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user(
    controller: web::Data<UserController>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.get_user(path.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/users",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Page size"),
        ("company_id" = Option<Uuid>, Query, description = "Filter by company"),
        ("role" = Option<String>, Query, description = "Filter by role")
    ),
    responses(
        (status = 200, description = "Users retrieved", body = UserListResponse),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_users(
    controller: web::Data<UserController>,
    query: web::Query<ListUsersQuery>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.list_users(query.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserDto),
        (status = 404, description = "User not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_user(
    controller: web::Data<UserController>,
    path: web::Path<Uuid>,
    request: web::Json<UpdateUserRequest>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.update_user(path.into_inner(), request.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_user(
    controller: web::Data<UserController>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.delete_user(path.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    post,
    path = "/users/{id}/roles",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = ChangeUserRoleRequest,
    responses(
        (status = 200, description = "User role changed successfully"),
        (status = 404, description = "User not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn change_user_role(
    controller: web::Data<UserController>,
    path: web::Path<Uuid>,
    request: web::Json<ChangeUserRoleRequest>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.change_user_role(path.into_inner(), request.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    post,
    path = "/users/{id}/company",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = AssignUserToCompanyRequest,
    responses(
        (status = 200, description = "User assigned to company successfully"),
        (status = 404, description = "User or company not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn assign_user_to_company(
    controller: web::Data<UserController>,
    path: web::Path<Uuid>,
    request: web::Json<AssignUserToCompanyRequest>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.assign_user_to_company(path.into_inner(), request.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    delete,
    path = "/users/{id}/company",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User removed from company successfully"),
        (status = 404, description = "User not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn remove_user_from_company(
    controller: web::Data<UserController>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.remove_user_from_company(path.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    put,
    path = "/users/me/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UserProfileDto),
        (status = 400, description = "Invalid input")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_profile(
    controller: web::Data<UserController>,
    request: web::Json<UpdateProfileRequest>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.update_profile(request.into_inner(), user_id.into_inner()).await
}

#[utoipa::path(
    put,
    path = "/users/me/password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Invalid input")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn change_password(
    controller: web::Data<UserController>,
    request: web::Json<ChangePasswordRequest>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.change_password(request.into_inner(), user_id.into_inner()).await
}

#[derive(serde::Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub company_id: Option<Uuid>,
    pub role: Option<String>,
}

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("")
                    .route(web::post().to(create_user))
                    .route(web::get().to(list_users))
            )
            .service(
                web::resource("/me/profile")
                    .route(web::put().to(update_profile))
            )
            .service(
                web::resource("/me/password")
                    .route(web::put().to(change_password))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_user))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user))
            )
            .service(
                web::resource("/{id}/roles")
                    .route(web::post().to(change_user_role))
            )
            .service(
                web::resource("/{id}/company")
                    .route(web::post().to(assign_user_to_company))
                    .route(web::delete().to(remove_user_from_company))
            )
    );
}