use std::sync::Arc;
use crate::domain::user::User;
use crate::domain::errors::DomainError;
use crate::domain::user_repository::UserRepository;
use crate::infrastructure::keycloak_client::KeycloakClient;
use crate::infrastructure::errors::InfrastructureError;

/// Repository for managing users via Keycloak
#[derive(Clone)]
pub struct KeycloakUserRepository {
    pub client: KeycloakClient,
}

#[async_trait::async_trait]
impl UserRepository for KeycloakUserRepository {
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        self.client
            .create_user(user)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    async fn find_by_username(&self, _username: &str) -> Result<Option<User>, DomainError> {
        // Optional: implement later using Keycloak API
        Ok(None)
    }
}
