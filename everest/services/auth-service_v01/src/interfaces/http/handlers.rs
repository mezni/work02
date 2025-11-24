use actix_web::{post, web, HttpResponse, Responder};
use crate::application::dtos::CreateUserDTO;
use crate::application::services::UserService;
use crate::infrastructure::repository_impl::UserRepositoryKeycloak;
use utoipa::ToSchema;

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserDTO,
    responses(
        (status = 200, description = "User created", body = String)
    )
)]
#[post("/users")]
pub async fn create_user(
    dto: web::Json<CreateUserDTO>,
    service: web::Data<UserService<UserRepositoryKeycloak>>,
) -> impl Responder {
    match service.create_user(dto.0).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
