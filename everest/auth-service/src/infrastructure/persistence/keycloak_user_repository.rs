use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::{RepositoryError, UserRepository};
use crate::domain::value_objects::{Email, UserId};
use crate::infrastructure::keycloak::client::KeycloakClient;
use async_trait::async_trait;
use keycloak::types::{CredentialRepresentation, UserRepresentation};
use std::sync::Arc;
use tracing::{debug, error, info};

pub struct KeycloakUserRepository {
    client: Arc<KeycloakClient>,
}

impl KeycloakUserRepository {
    pub fn new(client: Arc<KeycloakClient>) -> Self {
        Self { client }
    }

    fn to_domain_user(&self, keycloak_user: UserRepresentation) -> Result<User, RepositoryError> {
        let username = keycloak_user
            .username
            .ok_or_else(|| RepositoryError::DatabaseError("Missing username".to_string()))?;

        let email_str = keycloak_user
            .email
            .ok_or_else(|| RepositoryError::DatabaseError("Missing email".to_string()))?;

        let email = Email::new(email_str)
            .map_err(|e| RepositoryError::DatabaseError(format!("Invalid email: {}", e)))?;

        let first_name = keycloak_user.first_name.unwrap_or_default();
        let last_name = keycloak_user.last_name.unwrap_or_default();
        let enabled = keycloak_user.enabled.unwrap_or(true);

        let mut user = User::new(username, email, first_name, last_name);
        user.enabled = enabled;

        if let Some(id) = keycloak_user.id {
            user = user.with_id(UserId::new(id));
        }

        Ok(user)
    }

    fn to_keycloak_user(&self, user: &User) -> UserRepresentation {
        UserRepresentation {
            id: user.id.as_ref().map(|id| id.as_str().to_string()),
            username: Some(user.username.clone()),
            email: Some(user.email.as_str().to_string()),
            first_name: Some(user.first_name.clone()),
            last_name: Some(user.last_name.clone()),
            enabled: Some(user.enabled),
            ..Default::default()
        }
    }
}

#[async_trait]
impl UserRepository for KeycloakUserRepository {
    async fn create(&self, user: &User, password: &str) -> Result<UserId, RepositoryError> {
        info!("Creating user: {}", user.username);

        let admin = self.client.get_admin().await;
        let realm = self.client.realm();
        let keycloak_user = self.to_keycloak_user(user);

        admin
            .realm_users_post(realm, keycloak_user)
            .await
            .map_err(|e| {
                error!("Failed to create user: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        // Fetch the created user to get the ID
        let created_user = self
            .find_by_username(&user.username)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(user.username.clone()))?;

        let user_id = created_user
            .id
            .ok_or_else(|| RepositoryError::DatabaseError("No user ID returned".to_string()))?;

        // Set password
        let credential = CredentialRepresentation {
            type_: Some("password".to_string()),
            value: Some(password.to_string()),
            temporary: Some(false),
            ..Default::default()
        };

        admin
            .realm_users_with_user_id_reset_password_put(realm, user_id.as_str(), credential)
            .await
            .map_err(|e| {
                error!("Failed to set password: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        info!("User created successfully: {}", user_id);
        Ok(user_id)
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepositoryError> {
        debug!("Finding user by ID: {}", id);

        let admin = self.client.get_admin().await;
        let realm = self.client.realm();

        let keycloak_user = admin
            .realm_users_with_user_id_get(realm, id.as_str(), None)
            .await
            .map_err(|e| {
                error!("Failed to find user by ID: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        Ok(Some(self.to_domain_user(keycloak_user)?))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError> {
        debug!("Finding user by username: {}", username);

        let admin = self.client.get_admin().await;
        let realm = self.client.realm();

        let users = admin
            .realm_users_get(
                realm,
                None,
                None,
                None,
                Some(true),
                None,
                None,
                None,
                None,
                None,
                Some(username.to_string()),
                None,
                None,
                None,
                None,
            )
            .await
            .map_err(|e| {
                error!("Failed to find user by username: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        if let Some(keycloak_user) = users.into_iter().next() {
            Ok(Some(self.to_domain_user(keycloak_user)?))
        } else {
            Ok(None)
        }
    }

    async fn find_all(&self) -> Result<Vec<User>, RepositoryError> {
        debug!("Finding all users");

        let admin = self.client.get_admin().await;
        let realm = self.client.realm();

        let keycloak_users = admin
            .realm_users_get(
                realm, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            )
            .await
            .map_err(|e| {
                error!("Failed to find all users: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        keycloak_users
            .into_iter()
            .map(|ku| self.to_domain_user(ku))
            .collect()
    }

    async fn update(&self, user: &User) -> Result<(), RepositoryError> {
        info!("Updating user: {}", user.username);

        let user_id = user
            .id
            .as_ref()
            .ok_or_else(|| RepositoryError::DatabaseError("User ID is required".to_string()))?;

        let admin = self.client.get_admin().await;
        let realm = self.client.realm();
        let keycloak_user = self.to_keycloak_user(user);

        admin
            .realm_users_with_user_id_put(realm, user_id.as_str(), keycloak_user)
            .await
            .map_err(|e| {
                error!("Failed to update user: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        info!("User updated successfully");
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError> {
        info!("Deleting user: {}", id);

        let admin = self.client.get_admin().await;
        let realm = self.client.realm();

        admin
            .realm_users_with_user_id_delete(realm, id.as_str())
            .await
            .map_err(|e| {
                error!("Failed to delete user: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        info!("User deleted successfully");
        Ok(())
    }

    async fn assign_role(&self, user_id: &UserId, role_name: &str) -> Result<(), RepositoryError> {
        info!("Assigning role '{}' to user: {}", role_name, user_id);

        let admin = self.client.get_admin().await;
        let realm = self.client.realm();

        let role = admin
            .realm_roles_with_role_name_get(realm, role_name)
            .await
            .map_err(|e| {
                error!("Failed to get role: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        admin
            .realm_users_with_user_id_role_mappings_realm_post(realm, user_id.as_str(), vec![role])
            .await
            .map_err(|e| {
                error!("Failed to assign role: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        info!("Role assigned successfully");
        Ok(())
    }

    async fn get_roles(&self, user_id: &UserId) -> Result<Vec<String>, RepositoryError> {
        debug!("Getting roles for user: {}", user_id);

        let admin = self.client.get_admin().await;
        let realm = self.client.realm();

        let roles = admin
            .realm_users_with_user_id_role_mappings_realm_get(realm, user_id.as_str())
            .await
            .map_err(|e| {
                error!("Failed to get roles: {}", e);
                RepositoryError::DatabaseError(e.to_string())
            })?;

        Ok(roles.into_iter().filter_map(|r| r.name).collect())
    }
}
