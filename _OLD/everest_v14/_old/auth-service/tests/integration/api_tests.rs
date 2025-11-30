#[cfg(test)]
mod integration_tests {
    use actix_web::{test, web, App};
    use sqlx::PgPool;
    use std::sync::Arc;
    
    use auth_service::{
        application::handlers::{UserCommandHandler, UserQueryHandler},
        infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
        interfaces::http::{handlers::UserHandlers, routes::configure_routes},
    };

    async fn setup_test_app(pool: PgPool) -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        let user_repository = Arc::new(PostgresUserRepository::new(pool));
        
        let command_handler = Arc::new(UserCommandHandler::new(user_repository.clone()));
        let query_handler = Arc::new(UserQueryHandler::new(user_repository.clone()));

        // Mock Keycloak client
        let keycloak_config = auth_service::infrastructure::config::KeycloakConfig {
            url: "http://localhost:8080".to_string(),
            realm: "test".to_string(),
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        };

        let keycloak_client = Arc::new(
            auth_service::infrastructure::keycloak::client::KeycloakClient::new(keycloak_config)
        );

        let user_handlers = web::Data::new(UserHandlers::new(
            command_handler,
            query_handler,
            keycloak_client,
        ));

        test::init_service(
            App::new()
                .app_data(user_handlers)
                .configure(configure_routes::<PostgresUserRepository>),
        )
        .await
    }

    #[actix_web::test]
    async fn test_health_check() {
        let pool = PgPool::connect("postgresql://test:test@localhost/test_db")
            .await
            .expect("Failed to connect to test database");

        let app = setup_test_app(pool).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_user_unauthorized() {
        let pool = PgPool::connect("postgresql://test:test@localhost/test_db")
            .await
            .expect("Failed to connect to test database");

        let app = setup_test_app(pool).await;

        let req = test::TestRequest::post()
            .uri("/api/v1/users")
            .set_json(serde_json::json!({
                "email": "test@example.com",
                "username": "testuser",
                "password": "password123",
                "role": "operator",
                "organisation_name": "AcmeCorp"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Should be unauthorized without authentication
        assert_eq!(resp.status().as_u16(), 401);
    }
}
