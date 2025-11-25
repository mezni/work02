#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::services::{AuthService, AuthServiceImpl};
    use crate::domain::entities::User;
    use crate::domain::enums::UserRole;
    use async_trait::async_trait;
    use uuid::Uuid;

    // Mock repositories for testing
    struct MockUserRepository;
    
    #[async_trait]
    impl crate::domain::repositories::UserRepository for MockUserRepository {
        async fn create(&self, _user: &User) -> Result<User, crate::domain::errors::DomainError> {
            Ok(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap())
        }
        
        async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>, crate::domain::errors::DomainError> {
            Ok(Some(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap()))
        }
        
        async fn find_by_keycloak_id(&self, _keycloak_id: &str) -> Result<Option<User>, crate::domain::errors::DomainError> {
            Ok(Some(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap()))
        }
        
        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, crate::domain::errors::DomainError> {
            Ok(Some(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap()))
        }
        
        async fn find_by_username(&self, _username: &str) -> Result<Option<User>, crate::domain::errors::DomainError> {
            Ok(Some(User::new(
                "keycloak-123".to_string(),
                "testuser".to_string(),
                "test@example.com".to_string(),
                UserRole::User,
                None,
            ).unwrap()))
        }
        
        async fn update(&self, user: &User) -> Result<User, crate::domain::errors::DomainError> {
            Ok(user.clone())
        }
        
        async fn delete(&self, _id: Uuid) -> Result<(), crate::domain::errors::DomainError> {
            Ok(())
        }
        
        async fn list_by_company(&self, _company_id: Uuid) -> Result<Vec<User>, crate::domain::errors::DomainError> {
            Ok(vec![])
        }
        
        async fn list_all(&self) -> Result<Vec<User>, crate::domain::errors::DomainError> {
            Ok(vec![])
        }
    }

    // Mock Keycloak client for testing
    struct MockKeycloakClient;
    
    impl crate::infrastructure::auth::KeycloakClient for MockKeycloakClient {
        async fn create_user(&self, _username: &str, _email: &str, _password: &str) -> Result<String, crate::infrastructure::errors::InfrastructureError> {
            Ok("keycloak-123".to_string())
        }
        
        async fn login(&self, _username: &str, _password: &str) -> Result<crate::infrastructure::auth::KeycloakTokenResponse, crate::infrastructure::errors::InfrastructureError> {
            Ok(crate::infrastructure::auth::KeycloakTokenResponse {
                access_token: "access-token".to_string(),
                refresh_token: "refresh-token".to_string(),
                expires_in: 3600,
                token_type: "Bearer".to_string(),
            })
        }
        
        async fn refresh_token(&self, _refresh_token: &str) -> Result<crate::infrastructure::auth::KeycloakTokenResponse, crate::infrastructure::errors::InfrastructureError> {
            Ok(crate::infrastructure::auth::KeycloakTokenResponse {
                access_token: "new-access-token".to_string(),
                refresh_token: "new-refresh-token".to_string(),
                expires_in: 3600,
                token_type: "Bearer".to_string(),
            })
        }
        
        async fn user_info(&self, _access_token: &str) -> Result<crate::infrastructure::auth::KeycloakUserInfo, crate::infrastructure::errors::InfrastructureError> {
            Ok(crate::infrastructure::auth::KeycloakUserInfo {
                sub: "keycloak-123".to_string(),
                email: "test@example.com".to_string(),
                preferred_username: "testuser".to_string(),
                email_verified: true,
                exp: 1234567890,
                iat: 1234567890,
            })
        }
        
        async fn update_user(&self, _user_id: &str, _attributes: std::collections::HashMap<String, String>) -> Result<(), crate::infrastructure::errors::InfrastructureError> {
            Ok(())
        }
        
        async fn reset_password(&self, _user_id: &str, _new_password: &str) -> Result<(), crate::infrastructure::errors::InfrastructureError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_auth_service_permissions_calculation() {
        let auth_service = AuthServiceImpl::new(
            MockKeycloakClient,
            Box::new(MockUserRepository),
        );
        
        let admin_user = User::new(
            "keycloak-admin".to_string(),
            "admin".to_string(),
            "admin@example.com".to_string(),
            UserRole::Admin,
            None,
        ).unwrap();
        
        let permissions = auth_service.calculate_permissions(&admin_user);
        assert!(permissions.contains(&"users:read".to_string()));
        assert!(permissions.contains(&"users:write".to_string()));
        assert!(permissions.contains(&"companies:read".to_string()));
    }
}
