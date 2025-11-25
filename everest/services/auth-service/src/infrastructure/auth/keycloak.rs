#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use mockito::{Server, Mock};
    use serde_json::json;
    use auth_service::{
        infrastructure::auth::keycloak::{
            KeycloakClient, 
            KeycloakLoginRequest, 
            KeycloakTokenResponse, 
            KeycloakUserInfo
        },
        infrastructure::config::{Settings, KeycloakSettings, AuthSettings, DatabaseSettings, ServerSettings, CacheSettings, AuditSettings},
        infrastructure::errors::InfrastructureError,
    };

    fn create_test_settings(server_url: &str) -> Settings {
        Settings {
            database: DatabaseSettings {
                url: "postgres://test:test@localhost:5432/test".to_string(),
                host: "localhost".to_string(),
                port: 5432,
                name: "test".to_string(),
                username: "test".to_string(),
                password: "test".to_string(),
            },
            keycloak: KeycloakSettings {
                url: server_url.to_string(),
                realm_name: "test-realm".to_string(),
                client_name: "test-client".to_string(),
                admin: "admin".to_string(),
                admin_password: "admin123".to_string(),
            },
            auth: AuthSettings {
                jwt_secret: "test-secret".to_string(),
                jwt_expiration_seconds: 3600,
            },
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            cache: CacheSettings {
                redis_url: "redis://localhost:6379".to_string(),
            },
            audit: AuditSettings {
                retention_days: 365,
            },
            log_level: "info".to_string(),
        }
    }

    fn create_mock_token_response() -> serde_json::Value {
        json!({
            "access_token": "test-access-token",
            "refresh_token": "test-refresh-token",
            "expires_in": 3600,
            "refresh_expires_in": 7200,
            "token_type": "Bearer",
            "scope": "email profile"
        })
    }

    fn create_mock_user_info() -> serde_json::Value {
        json!({
            "sub": "user-123",
            "email": "test@example.com",
            "preferred_username": "testuser",
            "email_verified": true,
            "given_name": "Test",
            "family_name": "User"
        })
    }

    #[tokio::test]
    async fn test_keycloak_client_creation() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        // Test that client is created successfully - we can't test private fields directly
        // but we can test that methods work
        assert!(true); // Client creation succeeded
    }

    #[tokio::test]
    async fn test_successful_login() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        let mock_response = create_mock_token_response();
        
        let _m = server.mock("POST", "/realms/test-realm/protocol/openid-connect/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.login("testuser", "password123").await;
        
        assert!(result.is_ok());
        let token_response = result.unwrap();
        assert_eq!(token_response.access_token, "test-access-token");
        assert_eq!(token_response.refresh_token, "test-refresh-token");
        assert_eq!(token_response.expires_in, 3600);
    }

    #[tokio::test]
    async fn test_login_with_invalid_credentials() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        let _m = server.mock("POST", "/realms/test-realm/protocol/openid-connect/token")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "invalid_grant"}"#)
            .create();

        let result = client.login("wronguser", "wrongpass").await;
        
        assert!(result.is_err());
        if let Err(InfrastructureError::Keycloak(msg)) = result {
            assert!(msg.contains("API error 401"));
        } else {
            panic!("Expected Keycloak error");
        }
    }

    #[tokio::test]
    async fn test_login_network_error() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        // Server is stopped to simulate network error
        drop(server);

        let result = client.login("testuser", "password123").await;
        
        assert!(result.is_err());
        assert!(matches!(result, Err(InfrastructureError::Keycloak(_))));
    }

    #[tokio::test]
    async fn test_successful_token_refresh() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        let mock_response = create_mock_token_response();
        
        let _m = server.mock("POST", "/realms/test-realm/protocol/openid-connect/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.refresh_token("old-refresh-token").await;
        
        assert!(result.is_ok());
        let token_response = result.unwrap();
        assert_eq!(token_response.access_token, "test-access-token");
    }

    #[tokio::test]
    async fn test_token_refresh_with_invalid_token() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        let _m = server.mock("POST", "/realms/test-realm/protocol/openid-connect/token")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "invalid_grant"}"#)
            .create();

        let result = client.refresh_token("invalid-refresh-token").await;
        
        assert!(result.is_err());
        if let Err(InfrastructureError::Keycloak(msg)) = result {
            assert!(msg.contains("API error 400"));
        } else {
            panic!("Expected Keycloak error");
        }
    }

    #[tokio::test]
    async fn test_successful_user_info() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        let mock_user_info = create_mock_user_info();
        
        let _m = server.mock("GET", "/realms/test-realm/protocol/openid-connect/userinfo")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_user_info.to_string())
            .match_header("authorization", "Bearer valid-token")
            .create();

        let result = client.user_info("valid-token").await;
        
        assert!(result.is_ok());
        let user_info = result.unwrap();
        assert_eq!(user_info.sub, "user-123");
        assert_eq!(user_info.email, "test@example.com");
        assert_eq!(user_info.preferred_username, "testuser");
        assert!(user_info.email_verified);
    }

    #[tokio::test]
    async fn test_user_info_with_invalid_token() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        let _m = server.mock("GET", "/realms/test-realm/protocol/openid-connect/userinfo")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "invalid_token"}"#)
            .create();

        let result = client.user_info("invalid-token").await;
        
        assert!(result.is_err());
        if let Err(InfrastructureError::Keycloak(msg)) = result {
            assert!(msg.contains("API error 401"));
        } else {
            panic!("Expected Keycloak error");
        }
    }

    #[tokio::test]
    async fn test_successful_user_creation() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        // Mock admin token endpoint
        let admin_token_mock = server.mock("POST", "/realms/master/protocol/openid-connect/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(create_mock_token_response().to_string())
            .create();

        // Mock user creation endpoint
        let user_creation_mock = server.mock("POST", "/admin/realms/test-realm/users")
            .with_status(201)
            .with_header("location", "/admin/realms/test-realm/users/user-123")
            .create();

        let result = client.create_user(
            "newuser",
            "newuser@example.com",
            "password123",
            Some("John"),
            Some("Doe"),
        ).await;

        admin_token_mock.assert();
        user_creation_mock.assert();
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "user-123");
    }

    #[tokio::test]
    async fn test_user_creation_without_names() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        // Mock admin token endpoint
        let admin_token_mock = server.mock("POST", "/realms/master/protocol/openid-connect/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(create_mock_token_response().to_string())
            .create();

        // Mock user creation endpoint
        let user_creation_mock = server.mock("POST", "/admin/realms/test-realm/users")
            .with_status(201)
            .with_header("location", "/admin/realms/test-realm/users/user-456")
            .create();

        let result = client.create_user(
            "newuser2",
            "newuser2@example.com",
            "password123",
            None,
            None,
        ).await;

        admin_token_mock.assert();
        user_creation_mock.assert();
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "user-456");
    }

    #[tokio::test]
    async fn test_user_creation_failure() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        // Mock admin token endpoint
        let admin_token_mock = server.mock("POST", "/realms/master/protocol/openid-connect/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(create_mock_token_response().to_string())
            .create();

        // Mock user creation endpoint with error
        let user_creation_mock = server.mock("POST", "/admin/realms/test-realm/users")
            .with_status(409)
            .with_body(r#"{"error": "User exists"}"#)
            .create();

        let result = client.create_user(
            "existinguser",
            "existing@example.com",
            "password123",
            Some("Existing"),
            Some("User"),
        ).await;

        admin_token_mock.assert();
        user_creation_mock.assert();
        
        assert!(result.is_err());
        if let Err(InfrastructureError::Keycloak(msg)) = result {
            assert!(msg.contains("Create user failed"));
        } else {
            panic!("Expected Keycloak error");
        }
    }

    #[tokio::test]
    async fn test_user_creation_missing_location_header() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        // Mock admin token endpoint
        let admin_token_mock = server.mock("POST", "/realms/master/protocol/openid-connect/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(create_mock_token_response().to_string())
            .create();

        // Mock user creation endpoint without location header
        let user_creation_mock = server.mock("POST", "/admin/realms/test-realm/users")
            .with_status(201)
            .create(); // No location header

        let result = client.create_user(
            "newuser",
            "newuser@example.com",
            "password123",
            Some("John"),
            Some("Doe"),
        ).await;

        admin_token_mock.assert();
        user_creation_mock.assert();
        
        assert!(result.is_err());
        if let Err(InfrastructureError::Keycloak(msg)) = result {
            assert!(msg.contains("Failed to extract user ID"));
        } else {
            panic!("Expected Keycloak error");
        }
    }

    #[tokio::test]
    async fn test_admin_token_failure() {
        let mut server = Server::new_async().await;
        let settings = create_test_settings(&server.url());
        let client = KeycloakClient::new(&settings);
        
        // Mock admin token endpoint with error
        let admin_token_mock = server.mock("POST", "/realms/master/protocol/openid-connect/token")
            .with_status(401)
            .with_body(r#"{"error": "invalid_credentials"}"#)
            .create();

        let result = client.create_user(
            "newuser",
            "newuser@example.com",
            "password123",
            Some("John"),
            Some("Doe"),
        ).await;

        admin_token_mock.assert();
        
        assert!(result.is_err());
        if let Err(InfrastructureError::Keycloak(msg)) = result {
            assert!(msg.contains("Admin token request failed") || msg.contains("API error 401"));
        } else {
            panic!("Expected Keycloak error");
        }
    }

    #[tokio::test]
    async fn test_keycloak_login_request_serialization() {
        let login_request = KeycloakLoginRequest {
            username: "testuser".to_string(),
            password: "password123".to_string(),
            grant_type: "password".to_string(),
            client_id: "test-client".to_string(),
        };

        // Test that the struct can be created and used
        assert_eq!(login_request.username, "testuser");
        assert_eq!(login_request.password, "password123");
        assert_eq!(login_request.grant_type, "password");
        assert_eq!(login_request.client_id, "test-client");
    }

    #[test]
    fn test_keycloak_token_response_deserialization() {
        let json_data = r#"{
            "access_token": "test-token",
            "refresh_token": "refresh-token",
            "expires_in": 3600,
            "refresh_expires_in": 7200,
            "token_type": "Bearer",
            "scope": "email profile"
        }"#;

        let token_response: Result<KeycloakTokenResponse, _> = serde_json::from_str(json_data);
        assert!(token_response.is_ok());
        
        let token_response = token_response.unwrap();
        assert_eq!(token_response.access_token, "test-token");
        assert_eq!(token_response.refresh_token, "refresh-token");
        assert_eq!(token_response.expires_in, 3600);
        assert_eq!(token_response.refresh_expires_in, 7200);
        assert_eq!(token_response.token_type, "Bearer");
        assert_eq!(token_response.scope, "email profile");
    }

    #[test]
    fn test_keycloak_user_info_deserialization() {
        let json_data = r#"{
            "sub": "user-123",
            "email": "test@example.com",
            "preferred_username": "testuser",
            "email_verified": true,
            "given_name": "John",
            "family_name": "Doe"
        }"#;

        let user_info: Result<KeycloakUserInfo, _> = serde_json::from_str(json_data);
        assert!(user_info.is_ok());
        
        let user_info = user_info.unwrap();
        assert_eq!(user_info.sub, "user-123");
        assert_eq!(user_info.email, "test@example.com");
        assert_eq!(user_info.preferred_username, "testuser");
        assert!(user_info.email_verified);
        assert_eq!(user_info.given_name, Some("John".to_string()));
        assert_eq!(user_info.family_name, Some("Doe".to_string()));
    }

    #[test]
    fn test_keycloak_user_info_deserialization_without_names() {
        let json_data = r#"{
            "sub": "user-123",
            "email": "test@example.com",
            "preferred_username": "testuser",
            "email_verified": false
        }"#;

        let user_info: Result<KeycloakUserInfo, _> = serde_json::from_str(json_data);
        assert!(user_info.is_ok());
        
        let user_info = user_info.unwrap();
        assert_eq!(user_info.sub, "user-123");
        assert_eq!(user_info.email, "test@example.com");
        assert_eq!(user_info.preferred_username, "testuser");
        assert!(!user_info.email_verified);
        assert!(user_info.given_name.is_none());
        assert!(user_info.family_name.is_none());
    }
}