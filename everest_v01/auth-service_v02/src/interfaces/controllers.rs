// src/interfaces/controllers.rs
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::interfaces::dto::RegisterRequest;
use crate::application::user_service::UserService;
use crate::infrastructure::user_repository::InMemoryUserRepository;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check OK", body = String)
    ),
    tag = "Health"
)]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("✅ Service is healthy")
}

#[utoipa::path(
    post,
    path = "/users/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = String)
    ),
    tag = "Users"
)]
#[post("/users/register")]
pub async fn register_user(
    service: web::Data<UserService<InMemoryUserRepository>>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    println!("📝 Registering user: {} <{}>", req.username, req.email);
    
    let user = service.register(req.username.clone(), req.email.clone()).await;
    
    let response = format!(
        "✅ User registered successfully!\nUsername: {}\nEmail: {}",
        user.username, user.email
    );
    
    HttpResponse::Ok().body(response)
}

// Helper functions for OpenAPI documentation
pub mod docs {
    use actix_web::HttpResponse;
    use crate::interfaces::dto::RegisterRequest;
    
    #[utoipa::path(
        get,
        path = "/health",
        responses(
            (status = 200, description = "Health check endpoint", body = String)
        ),
        tag = "Health"
    )]
    pub fn health_api() -> HttpResponse {
        HttpResponse::Ok().body("Health OK")
    }
    
    #[utoipa::path(
        post,
        path = "/users/register",
        request_body = RegisterRequest,
        responses(
            (status = 200, description = "User registered successfully", body = String)
        ),
        tag = "Users"
    )]
    pub fn register_user_api(_body: RegisterRequest) -> HttpResponse {
        HttpResponse::Ok().body("User registered")
    }
}