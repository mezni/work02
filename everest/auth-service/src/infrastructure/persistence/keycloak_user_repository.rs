use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::{RepositoryError, UserRepository};
use crate::domain::value_objects::{Email, UserId, OrganisationId};
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

    fn to_domain_user(&self, keycloak_user: &UserRepresentation) -> Result<User, RepositoryError> {
        let username = keycloak_user
            .username
            .as_ref()
            .ok_or_else(|| RepositoryError::DatabaseError("Missing username".to_string()))?
            .clone();

        let email_str = keycloak_user
            .email
            .as_ref()
            .ok_or_else(|| RepositoryError::DatabaseError("Missing email".to_string()))?
            .clone();

        let email = Email::new(email_str)
            .map_err(|e| RepositoryError::DatabaseError(format!("Invalid email: {}", e)))?;

        let first_name = keycloak_user.first_name.clone().unwrap_or_default();
        let last_name = keycloak_user.last_name.clone().unwrap_or_default();
        let enabled = keycloak_user.enabled.unwrap_or(true);

        // Generate a keycloak_id from the ID or create a placeholder
        let keycloak_id = keycloak_user.id.clone().unwrap_or_else(|| "unknown".to_string());

        let mut user = User::new(
            keycloak_id,
            username.clone(),
            email,
            first_name,
            last_name,
            "user".to_string(), // default role
        );
        user.enabled = enabled;

        // Set status based on enabled flag
        if !enabled {
            user.status = crate::domain::entities::user::UserStatus::Inactive;
        }

        if let Some(id) = &keycloak_user.id {
            user = user.with_id(UserId::new(id.clone()));
        }

        // Set organisation_id from attributes
        if let Some(org_id) = self.extract_organisation_id(keycloak_user) {
            user.organisation_id = Some(org_id);
        }

        Ok(user)
    }

    fn to_keycloak_user(&self, user: &User) -> UserRepresentation {
        let mut attributes = std::collections::HashMap::new();
        if let Some(org_id) = &user.organisation_id {
            attributes.insert(
                "organisation_id".to_string(),
                vec![org_id.as_i32().to_string()],
            );
        }

        UserRepresentation {
            id: user.id.as_ref().map(|id| id.as_str().to_string()),
            username: Some(user.username.clone()),
            email: Some(user.email.as_str().to_string()),
            first_name: Some(user.first_name.clone()),
            last_name: Some(user.last_name.clone()),
            enabled: Some(user.enabled),
            attributes: if attributes.is_empty() {
                None
            } else {
                Some(attributes)
            },
            ..Default::default()
        }
    }

    /// Extract organisation_id from Keycloak user attributes
    fn extract_organisation_id(&self, keycloak_user: &UserRepresentation) -> Option<OrganisationId> {
        keycloak_user
            .attributes
            .as_ref()
            .and_then(|attrs| attrs.get("organisation_id"))
            .and_then(|org_ids| org_ids.first())
            .and_then(|org_id_str| org_id_str.parse::<i32>().ok())
            .map(OrganisationId::new)
    }
}

#[async_trait]
impl UserRepository for KeycloakUserRepository {
    async fn create(&self, user: &User, password: &str) -> Result<UserId, RepositoryError> {
        info!("Creating user in Keycloak: {}", user.username);

        let realm = self.client.realm();
        let keycloak_user = self.to_keycloak_user(user);

        // Create user in Keycloak
        let _response = self.client.with_retry(|_admin| {
            let keycloak_user = keycloak_user.clone();
            async move {
                // Use the client directly instead of the admin parameter
                let admin = self.client.get_admin().await;
                admin.realm_users_post(realm, keycloak_user).await
            }
        }).await.map_err(|e| {
            error!("Failed to create user in Keycloak: {}", e);
            RepositoryError::DatabaseError(format!("Keycloak error: {}", e))
        })?;

        // Fetch the created user to get the ID
        let created_user = self
            .find_by_username(&user.username)
            .await?
            .ok_or_else(|| RepositoryError::DatabaseError("Failed to fetch created user".to_string()))?;

        let user_id = created_user
            .id
            .clone()
            .ok_or_else(|| RepositoryError::DatabaseError("No user ID returned".to_string()))?;

        // Set password for the user
        let credential = CredentialRepresentation {
            type_: Some("password".to_string()),
            value: Some(password.to_string()),
            temporary: Some(false),
            ..Default::default()
        };

        let user_id_str = user_id.as_str().to_string();
        self.client.with_retry(move |_admin| {
            let user_id_str = user_id_str.clone();
            let credential = credential.clone();
            async move {
                let admin = self.client.get_admin().await;
                admin.realm_users_with_user_id_reset_password_put(
                    realm,
                    &user_id_str,
                    credential,
                ).await
            }
        }).await.map_err(|e| {
            error!("Failed to set password for user: {}", e);
            RepositoryError::DatabaseError(format!("Failed to set password: {}", e))
        })?;

        info!("User created successfully in Keycloak: {}", user_id);
        Ok(user_id)
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepositoryError> {
        debug!("Finding user by ID in Keycloak: {}", id);

        let realm = self.client.realm();
        let id_str = id.as_str().to_string();

        let keycloak_user = self.client.with_retry(move |_admin| {
            let id_str = id_str.clone();
            async move {
                let admin = self.client.get_admin().await;
                admin.realm_users_with_user_id_get(realm, &id_str, None).await
            }
        }).await;

        match keycloak_user {
            Ok(user) => {
                let domain_user = self.to_domain_user(&user)?;
                Ok(Some(domain_user))
            }
            Err(keycloak::KeycloakError::HttpFailure { status, .. }) if status == 404 => {
                debug!("User not found in Keycloak: {}", id);
                Ok(None)
            }
            Err(e) => {
                error!("Failed to find user by ID in Keycloak: {}", e);
                Err(RepositoryError::DatabaseError(format!("Keycloak error: {}", e)))
            }
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError> {
        debug!("Finding user by username in Keycloak: {}", username);

        let realm = self.client.realm();
        let username_clone = username.to_string();

        let users = self.client.with_retry(move |_admin| {
            let username = username_clone.clone();
            async move {
                let admin = self.client.get_admin().await;
                admin.realm_users_get(
                    realm,
                    None,                    // brief_representation
                    None,                    // email
                    None,                    // email_verified
                    None,                    // enabled
                    None,                    // exact
                    None,                    // first
                    None,                    // first_name
                    None,                    // last_name
                    None,                    // max
                    Some(username),          // search (username)
                    None,                    // username_exact
                    None,                    // last
                    None,                    // q
                    None,                    // search
                ).await
            }
        }).await.map_err(|e| {
            error!("Failed to find user by username in Keycloak: {}", e);
            RepositoryError::DatabaseError(format!("Keycloak error: {}", e))
        })?;

        if let Some(keycloak_user) = users.into_iter().next() {
            let domain_user = self.to_domain_user(&keycloak_user)?;
            Ok(Some(domain_user))
        } else {
            debug!("User not found by username in Keycloak: {}", username);
            Ok(None)
        }
    }

    async fn find_all(&self) -> Result<Vec<User>, RepositoryError> {
        debug!("Finding all users in Keycloak");

        let realm = self.client.realm();

        let keycloak_users = self.client.with_retry(|_admin| async move {
            let admin = self.client.get_admin().await;
            admin.realm_users_get(
                realm,
                None, // brief_representation
                None, // email
                None, // email_verified
                None, // enabled
                None, // exact
                None, // first
                None, // first_name
                None, // last_name
                None, // max
                None, // search
                None, // username_exact
                None, // last
                None, // q
                None, // search
            ).await
        }).await.map_err(|e| {
            error!("Failed to find all users in Keycloak: {}", e);
            RepositoryError::DatabaseError(format!("Keycloak error: {}", e))
        })?;

        let mut domain_users = Vec::new();
        for keycloak_user in keycloak_users {
            match self.to_domain_user(&keycloak_user) {
                Ok(user) => {
                    domain_users.push(user);
                }
                Err(e) => {
                    error!("Failed to convert Keycloak user to domain user: {}", e);
                    // Continue with other users instead of failing the entire request
                }
            }
        }

        Ok(domain_users)
    }

    async fn update(&self, user: &User) -> Result<(), RepositoryError> {
        info!("Updating user in Keycloak: {}", user.username);

        let user_id = user
            .id
            .as_ref()
            .ok_or_else(|| RepositoryError::DatabaseError("User ID is required".to_string()))?;

        let realm = self.client.realm();
        let keycloak_user = self.to_keycloak_user(user);
        let user_id_str = user_id.as_str().to_string();

        self.client.with_retry(move |_admin| {
            let user_id_str = user_id_str.clone();
            let keycloak_user = keycloak_user.clone();
            async move {
                let admin = self.client.get_admin().await;
                admin.realm_users_with_user_id_put(realm, &user_id_str, keycloak_user).await
            }
        }).await.map_err(|e| {
            error!("Failed to update user in Keycloak: {}", e);
            RepositoryError::DatabaseError(format!("Keycloak error: {}", e))
        })?;

        info!("User updated successfully in Keycloak: {}", user_id);
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError> {
        info!("Deleting user from Keycloak: {}", id);

        let realm = self.client.realm();
        let id_str = id.as_str().to_string();

        self.client.with_retry(move |_admin| {
            let id_str = id_str.clone();
            async move {
                let admin = self.client.get_admin().await;
                admin.realm_users_with_user_id_delete(realm, &id_str).await
            }
        }).await.map_err(|e| {
            error!("Failed to delete user from Keycloak: {}", e);
            RepositoryError::DatabaseError(format!("Keycloak error: {}", e))
        })?;

        info!("User deleted successfully from Keycloak: {}", id);
        Ok(())
    }

    async fn assign_role(&self, user_id: &UserId, role_name: &str) -> Result<(), RepositoryError> {
        info!("Assigning role '{}' to user in Keycloak: {}", role_name, user_id);

        let realm = self.client.realm();
        let user_id_str = user_id.as_str().to_string();
        let role_name_clone = role_name.to_string();

        // Get the role from Keycloak
        let role = self.client.with_retry(move |_admin| {
            let role_name = role_name_clone.clone();
            async move {
                let admin = self.client.get_admin().await;
                admin.realm_roles_with_role_name_get(realm, &role_name).await
            }
        }).await.map_err(|e| {
            error!("Failed to get role from Keycloak: {}", e);
            RepositoryError::DatabaseError(format!("Keycloak error: {}", e))
        })?;

        // Assign the role to the user
        let role_clone = role.clone();
        self.client.with_retry(move |_admin| {
            let user_id_str = user_id_str.clone();
            let role = role_clone.clone();
            async move {
                let admin = self.client.get_admin().await;
                admin.realm_users_with_user_id_role_mappings_realm_post(
                    realm,
                    &user_id_str,
                    vec![role],
                ).await
            }
        }).await.map_err(|e| {
            error!("Failed to assign role in Keycloak: {}", e);
            RepositoryError::DatabaseError(format!("Keycloak error: {}", e))
        })?;

        info!("Role '{}' assigned successfully to user: {}", role_name, user_id);
        Ok(())
    }

    async fn get_roles(&self, user_id: &UserId) -> Result<Vec<String>, RepositoryError> {
        debug!("Getting roles for user from Keycloak: {}", user_id);

        let realm = self.client.realm();
        let user_id_str = user_id.as_str().to_string();

        let roles = self.client.with_retry(move |_admin| {
            let user_id_str = user_id_str.clone();
            async move {
                let admin = self.client.get_admin().await;
                admin.realm_users_with_user_id_role_mappings_realm_get(realm, &user_id_str).await
            }
        }).await.map_err(|e| {
            error!("Failed to get roles from Keycloak: {}", e);
            RepositoryError::DatabaseError(format!("Keycloak error: {}", e))
        })?;

        let role_names: Vec<String> = roles.into_iter().filter_map(|r| r.name).collect();
        Ok(role_names)
    }

    async fn assign_to_organisation(
        &self,
        user_id: &UserId,
        organisation_id: &OrganisationId,
    ) -> Result<(), RepositoryError> {
        info!(
            "Assigning user {} to organisation {} in Keycloak",
            user_id, organisation_id
        );

        // First, get the current user
        let mut user = self
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(user_id.as_str().to_string()))?;

        // Update the organisation in the user object
        user.organisation_id = Some(organisation_id.clone());

        // Update the user in Keycloak with the new organisation
        self.update(&user).await?;

        info!(
            "User {} successfully assigned to organisation {} in Keycloak",
            user_id, organisation_id
        );
        Ok(())
    }
}