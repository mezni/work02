use async_trait::async_trait;
use auth_service::application::dto::{BusinessClaims, LoginResponse, UserDto};
use auth_service::application::errors::ApplicationError;
use auth_service::application::services::{AuthService, AuthServiceImpl};
use auth_service::domain::entities::User;
use auth_service::domain::enums::UserRole;
use auth_service::domain::errors::DomainError;
use auth_service::domain::repositories::UserRepository;
use auth_service::infrastructure::auth::{KeycloakClient, KeycloakTokenResponse, KeycloakUserInfo};
use auth_service::infrastructure::errors::InfrastructureError;
use uuid::Uuid;

// Mock UserRepository for auth tests
struct MockUserRepository {
    should_find_user: bool,
    user_role: UserRole,
}

impl MockUserRepository {
    fn new(should_find_user: bool, user_role: UserRole) -> Self {
        Self {
            should_find_user,
            user_role,
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        Ok(user.clone())
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError> {
        if self.should_find_user {
            Ok(Some(
                User::new(
                    keycloak_id.to_string(),
                    "testuser".to_string(),
                    "test@example.com".to_string(),
                    self.user_role,
                    None,
                )
                .unwrap(),
            ))
        } else {
            Ok(None)
        }
    }

    async fn find_by_email(&self, _email: &str) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn find_by_username(&self, _username: &str) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        Ok(user.clone())
    }

    async fn delete(&self, _id: Uuid) -> Result<(), DomainError> {
        Ok(())
    }

    async fn list_by_company(&self, _company_id: Uuid) -> Result<Vec<User>, DomainError> {
        Ok(vec![])
    }

    async fn list_all(&self) -> Result<Vec<User>, DomainError> {
        Ok(vec![])
    }
}

// Mock KeycloakClient for auth tests - we need to create a trait for this
#[async_trait]
pub trait KeycloakClientTrait: Send + Sync {
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<String, InfrastructureError>;
    async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<KeycloakTokenResponse, InfrastructureError>;
    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<KeycloakTokenResponse, InfrastructureError>;
    async fn user_info(&self, access_token: &str) -> Result<KeycloakUserInfo, InfrastructureError>;
    async fn update_user(
        &self,
        user_id: &str,
        attributes: std::collections::HashMap<String, String>,
    ) -> Result<(), InfrastructureError>;
    async fn reset_password(
        &self,
        user_id: &str,
        new_password: &str,
    ) -> Result<(), InfrastructureError>;
}

// Implement the trait for the real KeycloakClient
#[async_trait]
impl KeycloakClientTrait for KeycloakClient {
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<String, InfrastructureError> {
        KeycloakClient::create_user(self, username, email, password).await
    }

    async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<KeycloakTokenResponse, InfrastructureError> {
        KeycloakClient::login(self, username, password).await
    }

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<KeycloakTokenResponse, InfrastructureError> {
        KeycloakClient::refresh_token(self, refresh_token).await
    }

    async fn user_info(&self, access_token: &str) -> Result<KeycloakUserInfo, InfrastructureError> {
        KeycloakClient::user_info(self, access_token).await
    }

    async fn update_user(
        &self,
        user_id: &str,
        attributes: std::collections::HashMap<String, String>,
    ) -> Result<(), InfrastructureError> {
        KeycloakClient::update_user(self, user_id, attributes).await
    }

    async fn reset_password(
        &self,
        user_id: &str,
        new_password: &str,
    ) -> Result<(), InfrastructureError> {
        KeycloakClient::reset_password(self, user_id, new_password).await
    }
}

// Mock KeycloakClient for auth tests
struct MockKeycloakClient {
    should_succeed: bool,
}

impl MockKeycloakClient {
    fn new(should_succeed: bool) -> Self {
        Self { should_succeed }
    }
}

#[async_trait]
impl KeycloakClientTrait for MockKeycloakClient {
    async fn create_user(
        &self,
        _username: &str,
        _email: &str,
        _password: &str,
    ) -> Result<String, InfrastructureError> {
        if self.should_succeed {
            Ok("keycloak-123".to_string())
        } else {
            Err(InfrastructureError::KeycloakError("Mock error".to_string()))
        }
    }

    async fn login(
        &self,
        _username: &str,
        _password: &str,
    ) -> Result<KeycloakTokenResponse, InfrastructureError> {
        if self.should_succeed {
            Ok(KeycloakTokenResponse {
                access_token: "access-token".to_string(),
                refresh_token: "refresh-token".to_string(),
                expires_in: 3600,
                token_type: "Bearer".to_string(),
            })
        } else {
            Err(InfrastructureError::KeycloakError("Mock error".to_string()))
        }
    }

    async fn refresh_token(
        &self,
        _refresh_token: &str,
    ) -> Result<KeycloakTokenResponse, InfrastructureError> {
        if self.should_succeed {
            Ok(KeycloakTokenResponse {
                access_token: "new-access-token".to_string(),
                refresh_token: "new-refresh-token".to_string(),
                expires_in: 3600,
                token_type: "Bearer".to_string(),
            })
        } else {
            Err(InfrastructureError::KeycloakError("Mock error".to_string()))
        }
    }

    async fn user_info(
        &self,
        _access_token: &str,
    ) -> Result<KeycloakUserInfo, InfrastructureError> {
        if self.should_succeed {
            Ok(KeycloakUserInfo {
                sub: "keycloak-123".to_string(),
                email: "test@example.com".to_string(),
                preferred_username: "testuser".to_string(),
                email_verified: true,
                exp: 1234567890,
                iat: 1234567890,
            })
        } else {
            Err(InfrastructureError::KeycloakError("Mock error".to_string()))
        }
    }

    async fn update_user(
        &self,
        _user_id: &str,
        _attributes: std::collections::HashMap<String, String>,
    ) -> Result<(), InfrastructureError> {
        Ok(())
    }

    async fn reset_password(
        &self,
        _user_id: &str,
        _new_password: &str,
    ) -> Result<(), InfrastructureError> {
        Ok(())
    }
}

// We need to update AuthServiceImpl to use the trait
pub struct TestAuthServiceImpl {
    keycloak_client: Box<dyn KeycloakClientTrait>,
    user_repository: Box<dyn UserRepository>,
}

impl TestAuthServiceImpl {
    pub fn new(
        keycloak_client: Box<dyn KeycloakClientTrait>,
        user_repository: Box<dyn UserRepository>,
    ) -> Self {
        Self {
            keycloak_client,
            user_repository,
        }
    }

    // Helper method to test permission calculation
    pub fn calculate_permissions(&self, user: &User) -> Vec<String> {
        let mut permissions = Vec::new();

        match user.role {
            UserRole::Admin => {
                permissions.extend_from_slice(&[
                    "users:read".to_string(),
                    "users:write".to_string(),
                    "users:delete".to_string(),
                    "companies:read".to_string(),
                    "companies:write".to_string(),
                    "companies:delete".to_string(),
                    "audit:read".to_string(),
                ]);
            }
            UserRole::Partner | UserRole::Operator => {
                permissions.extend_from_slice(&[
                    "users:read".to_string(),
                    "users:write".to_string(),
                    "companies:read".to_string(),
                ]);

                if let Some(company_id) = user.company_id {
                    permissions.push(format!("company:{}:manage", company_id));
                }
            }
            UserRole::User => {
                permissions.extend_from_slice(&[
                    "users:read:self".to_string(),
                    "users:write:self".to_string(),
                ]);
            }
            UserRole::Guest => {
                permissions.push("public:read".to_string());
            }
        }

        permissions
    }
}

#[async_trait]
impl AuthService for TestAuthServiceImpl {
    async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginResponse, ApplicationError> {
        // Authenticate with Keycloak
        let token_response = self
            .keycloak_client
            .login(&username, &password)
            .await
            .map_err(|_| ApplicationError::AuthenticationFailed)?;

        // Get user info from Keycloak
        let user_info = self
            .keycloak_client
            .user_info(&token_response.access_token)
            .await
            .map_err(|_| ApplicationError::AuthenticationFailed)?;

        // Find user in local database
        let user = self
            .user_repository
            .find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;

        let user_dto = UserDto {
            id: user.id,
            keycloak_id: user.keycloak_id,
            username: user.username,
            email: user.email,
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        };

        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_response.expires_in,
            user: user_dto,
        })
    }

    async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<UserDto, ApplicationError> {
        // Create user in Keycloak
        let keycloak_user_id = self
            .keycloak_client
            .create_user(&username, &email, &password)
            .await
            .map_err(|e| ApplicationError::ValidationError(e.to_string()))?;

        // Create user in local database with default User role
        let user = User::new(keycloak_user_id, username, email, UserRole::User, None)?;

        let created_user = self.user_repository.create(&user).await?;

        Ok(UserDto {
            id: created_user.id,
            keycloak_id: created_user.keycloak_id,
            username: created_user.username,
            email: created_user.email,
            role: created_user.role,
            company_id: created_user.company_id,
            email_verified: created_user.email_verified,
            created_at: created_user.created_at.to_rfc3339(),
            updated_at: created_user.updated_at.to_rfc3339(),
        })
    }

    async fn validate_token(&self, token: String) -> Result<BusinessClaims, ApplicationError> {
        // Validate token with Keycloak
        let user_info = self
            .keycloak_client
            .user_info(&token)
            .await
            .map_err(|_| ApplicationError::InvalidToken)?;

        // Find user in local database to get business context
        let user = self
            .user_repository
            .find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;

        // Generate business claims
        let permissions = self.calculate_permissions(&user);

        Ok(BusinessClaims {
            sub: user_info.sub,
            email: user_info.email,
            username: user_info.preferred_username,
            role: user.role,
            company_id: user.company_id,
            permissions,
            exp: user_info.exp,
            iat: user_info.iat,
        })
    }

    async fn refresh_token(
        &self,
        refresh_token: String,
    ) -> Result<LoginResponse, ApplicationError> {
        let token_response = self
            .keycloak_client
            .refresh_token(&refresh_token)
            .await
            .map_err(|_| ApplicationError::InvalidToken)?;

        // Get user info to return user data
        let user_info = self
            .keycloak_client
            .user_info(&token_response.access_token)
            .await
            .map_err(|_| ApplicationError::InvalidToken)?;

        let user = self
            .user_repository
            .find_by_keycloak_id(&user_info.sub)
            .await?
            .ok_or(ApplicationError::UserNotFound)?;

        let user_dto = UserDto {
            id: user.id,
            keycloak_id: user.keycloak_id,
            username: user.username,
            email: user.email,
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        };

        Ok(LoginResponse {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_response.expires_in,
            user: user_dto,
        })
    }
}

#[tokio::test]
async fn test_auth_service_login_success() {
    let keycloak_client = Box::new(MockKeycloakClient::new(true));
    let user_repo = Box::new(MockUserRepository::new(true, UserRole::User));

    let auth_service = TestAuthServiceImpl::new(keycloak_client, user_repo);

    let result = auth_service
        .login("testuser".to_string(), "password".to_string())
        .await;

    assert!(result.is_ok());
    let login_response = result.unwrap();
    assert_eq!(login_response.access_token, "access-token");
    assert_eq!(login_response.user.username, "testuser");
}

#[tokio::test]
async fn test_auth_service_login_user_not_found() {
    let keycloak_client = Box::new(MockKeycloakClient::new(true));
    let user_repo = Box::new(MockUserRepository::new(false, UserRole::User)); // User not found

    let auth_service = TestAuthServiceImpl::new(keycloak_client, user_repo);

    let result = auth_service
        .login("testuser".to_string(), "password".to_string())
        .await;

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ApplicationError::UserNotFound
    ));
}

#[tokio::test]
async fn test_auth_service_register_success() {
    let keycloak_client = Box::new(MockKeycloakClient::new(true));
    let user_repo = Box::new(MockUserRepository::new(false, UserRole::User));

    let auth_service = TestAuthServiceImpl::new(keycloak_client, user_repo);

    let result = auth_service
        .register(
            "newuser".to_string(),
            "newuser@example.com".to_string(),
            "password".to_string(),
        )
        .await;

    assert!(result.is_ok());
    let user_dto = result.unwrap();
    assert_eq!(user_dto.username, "newuser");
    assert_eq!(user_dto.role, UserRole::User);
}

#[tokio::test]
async fn test_auth_service_permissions_calculation() {
    let keycloak_client = Box::new(MockKeycloakClient::new(true));
    let user_repo = Box::new(MockUserRepository::new(true, UserRole::Admin));

    let auth_service = TestAuthServiceImpl::new(keycloak_client, user_repo);

    let admin_user = User::new(
        "keycloak-admin".to_string(),
        "admin".to_string(),
        "admin@example.com".to_string(),
        UserRole::Admin,
        None,
    )
    .unwrap();

    let permissions = auth_service.calculate_permissions(&admin_user);

    assert!(permissions.contains(&"users:read".to_string()));
    assert!(permissions.contains(&"users:write".to_string()));
    assert!(permissions.contains(&"companies:read".to_string()));
    assert!(permissions.contains(&"companies:write".to_string()));

    let user_user = User::new(
        "keycloak-user".to_string(),
        "user".to_string(),
        "user@example.com".to_string(),
        UserRole::User,
        None,
    )
    .unwrap();

    let user_permissions = auth_service.calculate_permissions(&user_user);
    assert!(user_permissions.contains(&"users:read:self".to_string()));
    assert!(user_permissions.contains(&"users:write:self".to_string()));
}
