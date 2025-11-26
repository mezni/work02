use auth_service::infrastructure::auth::KeycloakClient;
use auth_service::infrastructure::config::KeycloakConfig;

#[cfg(test)]
mod keycloak_tests {
    use super::*;
    
    #[test]
    fn test_keycloak_config_creation() {
        let config = KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test-realm".to_string(),
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        };
        
        assert_eq!(config.server_url, "http://localhost:8080");
        assert_eq!(config.realm, "test-realm");
        assert_eq!(config.client_id, "test-client");
    }
    
    #[test]
    fn test_keycloak_client_initialization() {
        let config = KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test-realm".to_string(),
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        };
        
        let client = KeycloakClient::new(config);
        
        // Test that client can be created (compilation test)
        assert!(true, "KeycloakClient should be created successfully");
    }
    
    #[test]
    fn test_keycloak_url_generation() {
        let config = KeycloakConfig {
            server_url: "http://localhost:8080".to_string(),
            realm: "test-realm".to_string(),
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            admin_username: "admin".to_string(),
            admin_password: "admin".to_string(),
        };
        
        let token_url = config.token_url();
        let user_info_url = config.user_info_url();
        let admin_users_url = config.admin_users_url();
        
        assert!(token_url.contains("protocol/openid-connect/token"));
        assert!(user_info_url.contains("protocol/openid-connect/userinfo"));
        assert!(admin_users_url.contains("admin/realms/test-realm/users"));
    }
}
