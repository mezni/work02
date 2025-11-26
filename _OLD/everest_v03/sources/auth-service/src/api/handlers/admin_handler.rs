// src/api/handlers/admin_handler.rs
use actix_web::{web, HttpResponse};
use crate::application::{UserService, DeactivateUserCommand, UserDto};
use crate::api::ApiError;

#[derive(Clone)]
pub struct AdminHandler {
    user_service: web::Data<dyn UserService>,
}

impl AdminHandler {
    pub fn new(user_service: web::Data<dyn UserService>) -> Self {
        Self { user_service }
    }

    #[utoipa::path(
        post,
        path = "/api/v1/admin/users/{id}/deactivate",
        params(
            ("id" = String, Path, description = "User ID to deactivate")
        ),
        request_body = DeactivateUserCommand,
        responses(
            (status = 200, description = "User deactivated successfully", body = UserDto),
            (status = 400, description = "Invalid request"),
            (status = 404, description = "User not found")
        ),
        tag = "admin",
        security(("bearer_token" = []))
    )]
    pub async fn deactivate_user(
        user_service: web::Data<dyn UserService>,
        path: web::Path<String>,
        command: web::Json<DeactivateUserCommand>,
    ) -> Result<HttpResponse, ApiError> {
        let user_id = path.into_inner();
        let user_dto = user_service.deactivate_user(user_id, command.into_inner()).await?;
        Ok(HttpResponse::Ok().json(user_dto))
    }

    #[utoipa::path(
        get,
        path = "/api/v1/admin/users",
        params(
            ("page" = Option<u32>, Query, description = "Page number"),
            ("size" = Option<u32>, Query, description = "Page size"),
            ("active_only" = Option<bool>, Query, description = "Filter active users only")
        ),
        responses(
            (status = 200, description = "Users list retrieved", body = crate::application::UserListDto),
        ),
        tag = "admin",
        security(("bearer_token" = []))
    )]
    pub async fn get_users_admin(
        user_service: web::Data<dyn UserService>,
        query: web::Query<crate::api::handlers::user_handler::GetUsersQueryParams>,
    ) -> Result<HttpResponse, ApiError> {
        let query = crate::application::GetUsersQuery::new(
            query.page,
            query.size,
            query.active_only,
        );
        let user_list = user_service.get_users(query).await?;
        Ok(HttpResponse::Ok().json(user_list))
    }
}