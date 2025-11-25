use crate::domain::models::{NewUser, User, UserCredentials};
use crate::domain::enums::UserRole;
use crate::application::repositories::UserRepository;
use crate::infrastructure::keycloak::KeycloakClient;
use crate::domain::errors::DomainError;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    user_repository: R,
    keycloak_client: KeycloakClient,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(user_repository: R, keycloak_client: KeycloakClient) -> Self {
        Self {
            user_repository,
            keycloak_client,
        }
    }

    pub async fn create_user(&self, new_user: NewUser, created_by_admin: bool) -> Result<User, DomainError> {
        // Validate business rules
        if !created_by_admin {
            // Self-registered users can only have User role
            if new_user.role != UserRole::User {
                return Err(DomainError::Unauthorized);
            }
            // Self-registered users cannot be assigned to a company
            if new_user.company_id.is_some() {
                return Err(DomainError::CompanyAssignmentNotAllowed);
            }
        }

        // Check if user already exists
        if self.user_repository.find_by_email(new_user.email.as_str()).await?.is_some() {
            return Err(DomainError::UserAlreadyExists);
        }

        if self.user_repository.find_by_username(new_user.username.as_str()).await?.is_some() {
            return Err(DomainError::UserAlreadyExists);
        }

        // Create user in Keycloak
        self.keycloak_client
            .create_user(&new_user.email, &new_user.password, &new_user.role.to_string())
            .await
            .map_err(|e| {
                DomainError::InvalidCredentials
            })?;

        // Create user in database
        let user = self.user_repository.create(new_user).await?;
        Ok(user)
    }

    pub async fn authenticate_user(&self, credentials: &UserCredentials) -> Result<(User, String), DomainError> {
        // Authenticate with Keycloak
        let token = self.keycloak_client
            .authenticate_user(&credentials.username, &credentials.password)
            .await
            .map_err(|_| DomainError::InvalidCredentials)?;

        // Get user from database
        let user = self.user_repository
            .find_by_username(&credentials.username)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        Ok((user, token))
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, DomainError> {
        self.user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(DomainError::UserNotFound)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError> {
        self.user_repository
            .find_by_email(email)
            .await?
            .ok_or(DomainError::UserNotFound)
    }
}

// Command and Query separation
#[derive(Debug)]
pub struct CreateUserCommand {
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: UserRole,
    pub company_id: Option<Uuid>,
    pub created_by_admin: bool,
}

#[derive(Debug)]
pub struct GetUserQuery {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct AuthenticateUserQuery {
    pub username: String,
    pub password: String,
}