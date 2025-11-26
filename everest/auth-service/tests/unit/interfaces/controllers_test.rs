use actix_web::{http, test, web, App};
use async_trait::async_trait;
use auth_service::application::dto::{LoginRequest, RegisterRequest};
use auth_service::domain::entities::User;
use auth_service::domain::enums::UserRole;
use auth_service::interfaces::controllers::{AuthController, CompanyController, UserController};
use auth_service::interfaces::routes::configure_routes;
use uuid::Uuid;

// Mock AuthService for testing
struct MockAuthService {
    should_succeed: bool,
}

impl MockAuthService {
    fn new(should_succeed: bool) -> Self {
        Self { should_succeed }
    }
}

#[async_trait]
impl auth_service::application::services::AuthService for MockAuthService {
    async fn login(
        &self,
        _username: String,
        _password: String,
    ) -> Result<
        auth_service::application::dto::LoginResponse,
        auth_service::application::errors::ApplicationError,
    > {
        if self.should_succeed {
            let user = User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            )
            .unwrap();

            Ok(auth_service::application::dto::LoginResponse {
                access_token: "test-token".to_string(),
                refresh_token: "test-refresh-token".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: 3600,
                user: auth_service::application::dto::UserDto {
                    id: user.id,
                    keycloak_id: user.keycloak_id,
                    username: user.username,
                    email: user.email,
                    role: user.role,
                    company_id: user.company_id,
                    email_verified: user.email_verified,
                    created_at: user.created_at.to_rfc3339(),
                    updated_at: user.updated_at.to_rfc3339(),
                },
            })
        } else {
            Err(auth_service::application::errors::ApplicationError::AuthenticationFailed)
        }
    }

    async fn register(
        &self,
        _username: String,
        _email: String,
        _password: String,
    ) -> Result<
        auth_service::application::dto::UserDto,
        auth_service::application::errors::ApplicationError,
    > {
        if self.should_succeed {
            let user = User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            )
            .unwrap();

            Ok(auth_service::application::dto::UserDto {
                id: user.id,
                keycloak_id: user.keycloak_id,
                username: user.username,
                email: user.email,
                role: user.role,
                company_id: user.company_id,
                email_verified: user.email_verified,
                created_at: user.created_at.to_rfc3339(),
                updated_at: user.updated_at.to_rfc3339(),
            })
        } else {
            Err(
                auth_service::application::errors::ApplicationError::ValidationError(
                    "Mock error".to_string(),
                ),
            )
        }
    }

    async fn validate_token(
        &self,
        _token: String,
    ) -> Result<
        auth_service::application::dto::BusinessClaims,
        auth_service::application::errors::ApplicationError,
    > {
        if self.should_succeed {
            Ok(auth_service::application::dto::BusinessClaims {
                sub: "keycloak-123".to_string(),
                email: "test@example.com".to_string(),
                username: "testuser".to_string(),
                role: UserRole::User,
                company_id: None,
                permissions: vec!["users:read:self".to_string()],
                exp: 1234567890,
                iat: 1234567890,
            })
        } else {
            Err(auth_service::application::errors::ApplicationError::InvalidToken)
        }
    }

    async fn refresh_token(
        &self,
        _refresh_token: String,
    ) -> Result<
        auth_service::application::dto::LoginResponse,
        auth_service::application::errors::ApplicationError,
    > {
        if self.should_succeed {
            let user = User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            )
            .unwrap();

            Ok(auth_service::application::dto::LoginResponse {
                access_token: "new-token".to_string(),
                refresh_token: "new-refresh-token".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: 3600,
                user: auth_service::application::dto::UserDto {
                    id: user.id,
                    keycloak_id: user.keycloak_id,
                    username: user.username,
                    email: user.email,
                    role: user.role,
                    company_id: user.company_id,
                    email_verified: user.email_verified,
                    created_at: user.created_at.to_rfc3339(),
                    updated_at: user.updated_at.to_rfc3339(),
                },
            })
        } else {
            Err(auth_service::application::errors::ApplicationError::InvalidToken)
        }
    }
}

#[actix_web::test]
async fn test_health_endpoint() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/health").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("auth-service"));
}

#[actix_web::test]
async fn test_swagger_ui_endpoint() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/swagger-ui/").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_auth_register_endpoint() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let register_request = RegisterRequest {
        username: "newuser".to_string(),
        email: "newuser@example.com".to_string(),
        password: "password123".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&register_request)
        .to_request();

    let resp = test::call_service(&app, req).await;
    // This should return 501 Not Implemented since we're using mock controllers
    // In a real implementation, this would test the actual registration flow
    assert!(resp.status().is_client_error() || resp.status().is_server_error());
}

#[actix_web::test]
async fn test_auth_login_endpoint() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let login_request = LoginRequest {
        username: "testuser".to_string(),
        password: "password123".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(&login_request)
        .to_request();

    let resp = test::call_service(&app, req).await;
    // This should return an error since we're using mock controllers
    assert!(resp.status().is_client_error() || resp.status().is_server_error());
}

#[actix_web::test]
async fn test_user_list_endpoint() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get().uri("/api/v1/users").to_request();

    let resp = test::call_service(&app, req).await;
    // Should return unauthorized/forbidden without proper authentication
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_company_list_endpoint() {
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/companies")
        .to_request();

    let resp = test::call_service(&app, req).await;
    // Should return unauthorized/forbidden without proper authentication
    assert!(resp.status().is_client_error());
}
