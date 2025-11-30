use actix_web::{web, HttpResponse};
use crate::service::user_service::UserService;
use crate::domain::user::{Role, User};
use crate::errors::AppError;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub role: Role,
    pub org_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
}

#[utoipa::path(
    post,
    path = "/user/create",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created", body = User),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user(
    svc: web::Data<UserService>,
    req: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let user = svc
        .create_user(
            req.name.clone(),
            req.email.clone(),
            req.role.clone(),
            req.org_id,
            req.station_id,
        )
        .await?;
    Ok(HttpResponse::Ok().json(user))
}
