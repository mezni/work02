use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::interfaces::controllers::{AuthController, CompanyController, UserController};
use crate::interfaces::openapi::ApiDoc;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Serve Swagger UI
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );

    // API routes
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(AuthController::register))
                    .route("/login", web::post().to(AuthController::login))
                    .route("/refresh", web::post().to(AuthController::refresh_token))
                    .route("/validate", web::post().to(AuthController::validate_token))
                    .route("/logout", web::post().to(AuthController::logout)),
            )
            .service(
                web::scope("/users")
                    .route("", web::get().to(UserController::list_users))
                    .route("", web::post().to(UserController::create_user))
                    .route("/{id}", web::get().to(UserController::get_user))
                    .route("/{id}", web::put().to(UserController::update_user)),
            )
            .service(
                web::scope("/companies")
                    .route("", web::get().to(CompanyController::list_companies))
                    .route("", web::post().to(CompanyController::create_company))
                    .route("/{id}", web::get().to(CompanyController::get_company))
                    .route("/{id}", web::put().to(CompanyController::update_company))
                    .route(
                        "/{id}/users",
                        web::get().to(CompanyController::list_company_users),
                    ),
            ),
    );

    // Health check
    cfg.route(
        "/health",
        web::get().to(|| async {
            actix_web::HttpResponse::Ok().json(serde_json::json!({
                "status": "ok",
                "service": "auth-service"
            }))
        }),
    );
}
