use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::interfaces::controllers::{AuthController, UserController, CompanyController};
use crate::interfaces::openapi::ApiDoc;

// Handler functions that work with Actix Web's routing system
async fn register_handler(
    auth_controller: web::Data<AuthController>,
    register_request: web::Json<crate::application::dto::RegisterRequest>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.register(register_request).await
}

async fn login_handler(
    auth_controller: web::Data<AuthController>,
    login_request: web::Json<crate::application::dto::LoginRequest>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.login(login_request).await
}

async fn refresh_token_handler(
    auth_controller: web::Data<AuthController>,
    refresh_request: web::Json<serde_json::Value>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.refresh_token(refresh_request).await
}

async fn validate_token_handler(
    auth_controller: web::Data<AuthController>,
    token_request: web::Json<serde_json::Value>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.validate_token(token_request).await
}

async fn logout_handler(
    auth_controller: web::Data<AuthController>,
    request: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    auth_controller.logout(request).await
}

async fn list_users_handler(
    user_controller: web::Data<UserController>,
    request: actix_web::HttpRequest,
    query: web::Query<crate::interfaces::controllers::ListUsersQuery>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    user_controller.list_users(request, query).await
}

async fn get_user_handler(
    user_controller: web::Data<UserController>,
    request: actix_web::HttpRequest,
    user_id: web::Path<uuid::Uuid>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    user_controller.get_user(request, user_id).await
}

async fn create_user_handler(
    user_controller: web::Data<UserController>,
    request: actix_web::HttpRequest,
    create_user_dto: web::Json<crate::application::dto::CreateUserDto>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    user_controller.create_user(request, create_user_dto).await
}

async fn update_user_handler(
    user_controller: web::Data<UserController>,
    request: actix_web::HttpRequest,
    user_id: web::Path<uuid::Uuid>,
    update_user_dto: web::Json<crate::application::dto::UpdateUserDto>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    user_controller.update_user(request, user_id, update_user_dto).await
}

async fn list_companies_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    query: web::Query<crate::interfaces::controllers::ListCompaniesQuery>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.list_companies(request, query).await
}

async fn get_company_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    company_id: web::Path<uuid::Uuid>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.get_company(request, company_id).await
}

async fn create_company_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    create_company_dto: web::Json<crate::application::dto::CreateCompanyDto>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.create_company(request, create_company_dto).await
}

async fn update_company_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    company_id: web::Path<uuid::Uuid>,
    update_company_dto: web::Json<crate::application::dto::UpdateCompanyDto>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.update_company(request, company_id, update_company_dto).await
}

async fn list_company_users_handler(
    company_controller: web::Data<CompanyController>,
    request: actix_web::HttpRequest,
    company_id: web::Path<uuid::Uuid>,
    query: web::Query<crate::interfaces::controllers::ListCompanyUsersQuery>,
) -> Result<actix_web::HttpResponse, crate::interfaces::errors::InterfaceError> {
    company_controller.list_company_users(request, company_id, query).await
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Serve Swagger UI
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
    
    // API routes
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(register_handler))
                    .route("/login", web::post().to(login_handler))
                    .route("/refresh", web::post().to(refresh_token_handler))
                    .route("/validate", web::post().to(validate_token_handler))
                    .route("/logout", web::post().to(logout_handler))
            )
            .service(
                web::scope("/users")
                    .route("", web::get().to(list_users_handler))
                    .route("", web::post().to(create_user_handler))
                    .route("/{id}", web::get().to(get_user_handler))
                    .route("/{id}", web::put().to(update_user_handler))
            )
            .service(
                web::scope("/companies")
                    .route("", web::get().to(list_companies_handler))
                    .route("", web::post().to(create_company_handler))
                    .route("/{id}", web::get().to(get_company_handler))
                    .route("/{id}", web::put().to(update_company_handler))
                    .route("/{id}/users", web::get().to(list_company_users_handler))
            )
    );
    
    // Health check
    cfg.route("/health", web::get().to(|| async {
        actix_web::HttpResponse::Ok().json(serde_json::json!({
            "status": "ok",
            "service": "auth-service"
        }))
    }));
}
