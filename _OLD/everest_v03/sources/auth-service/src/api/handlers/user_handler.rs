// src/api/handlers/user_handler.rs
use actix_web::{web, HttpResponse};
use utoipa::ToSchema;
use serde::Deserialize;
use crate::application::{
    UserService, 
    CreateUserCommand, 
    UpdateUserCommand, 
    UserDto, 
    GetUsersQuery,
    GetUserByIdQuery
};
use crate::api::ApiError;

#[derive(Clone)]
pub struct UserHandler {
    user_service: web::Data<dyn UserService>,
}

impl UserHandler {
    pub fn new(user_service: web::Data<dyn UserService>) -> Self {
        Self { user_service }
    }

    #[utoipa::path(
        post,
        path = "/api/v1/users",
        request_body = CreateUserCommand,
        responses(
            (status = 201, description = "User created successfully", body = UserDto),
            (status = 400, description = "Invalid input data"),
            (status = 409, description = "User already exists")
        ),
        tag = "users"
    )]
    pub async fn create_user(
        user_service: web::Data<dyn UserService>,
        command: web::Json<CreateUserCommand>,
    ) -> Result<HttpResponse, ApiError> {
        let user_dto = user_service.create_user(command.into_inner()).await?;
        Ok(HttpResponse::Created().json(user_dto))
    }

    #[utoipa::path(
        get,
        path = "/api/v1/users",
        params(
            ("page" = Option<u32>, Query, description = "Page number"),
            ("size" = Option<u32>, Query, description = "Page size"),
            ("active_only" = Option<bool>, Query, description = "Filter active users only")
        ),
        responses(
            (status = 200, description = "Users retrieved successfully", body = crate::application::UserListDto),
        ),
        tag = "users"
    )]
    pub async fn get_users(
        user_service: web::Data<dyn UserService>,
        query: web::Query<GetUsersQueryParams>,
    ) -> Result<HttpResponse, ApiError> {
        let query = GetUsersQuery::new(
            query.page,
            query.size,
            query.active_only,
        );
        let user_list = user_service.get_users(query).await?;
        Ok(HttpResponse::Ok().json(user_list))
    }

    #[utoipa::path(
        get,
        path = "/api/v1/users/{id}",
        params(
            ("id" = String, Path, description = "User ID")
        ),
        responses(
            (status = 200, description = "User found", body = UserDto),
            (status = 404, description = "User not found")
        ),
        tag = "users"
    )]
    pub async fn get_user_by_id(
        user_service: web::Data<dyn UserService>,
        path: web::Path<String>,
    ) -> Result<HttpResponse, ApiError> {
        let user_id = path.into_inner();
        let query = GetUserByIdQuery::new(user_id);
        let user_dto = user_service.get_user_by_id(query).await?;
        Ok(HttpResponse::Ok().json(user_dto))
    }

    #[utoipa::path(
        put,
        path = "/api/v1/users/{id}",
        params(
            ("id" = String, Path, description = "User ID")
        ),
        request_body = UpdateUserCommand,
        responses(
            (status = 200, description = "User updated successfully", body = UserDto),
            (status = 400, description = "Invalid input data"),
            (status = 404, description = "User not found")
        ),
        tag = "users"
    )]
    pub async fn update_user(
        user_service: web::Data<dyn UserService>,
        path: web::Path<String>,
        command: web::Json<UpdateUserCommand>,
    ) -> Result<HttpResponse, ApiError> {
        let user_id = path.into_inner();
        let user_dto = user_service.update_user(user_id, command.into_inner()).await?;
        Ok(HttpResponse::Ok().json(user_dto))
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetUsersQueryParams {
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub active_only: Option<bool>,
}