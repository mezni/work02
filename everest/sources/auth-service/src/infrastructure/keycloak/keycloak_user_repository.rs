// src/infrastructure/keycloak/keycloak_user_repository.rs
use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::entities::User;
use crate::domain::value_objects::{UserId, Email, Username, PhoneNumber};
use crate::domain::repositories::UserRepository;
use crate::domain::DomainError;
use crate::infrastructure::keycloak::KeycloakClient;
use crate::infrastructure::keycloak::models::{KeycloakUser, CreateKeycloakUser, UpdateKeycloakUser, KeycloakCredential};
use chrono::{DateTime, TimeZone, Utc};

#[derive(Clone)]
pub struct KeycloakUserRepository {
    keycloak_client: Arc<dyn KeycloakClient>,
}

impl KeycloakUserRepository {
    pub fn new(keycloak_client: Arc<dyn KeycloakClient>) -> Self {
        Self { keycloak_client }
    }

    fn to_domain_user(&self, keycloak_user: KeycloakUser) -> Result<User, DomainError> {
        let username = Username::new(&keycloak_user.username)?;
        let email = Email::new(&keycloak_user.email)?;
        
        let created_at = keycloak_user.created_timestamp
            .map(|ts| Utc.timestamp_opt(ts / 1000, 0).unwrap())
            .unwrap_or_else(Utc::now);

        let user = User {
            id: UserId::new(keycloak_user.id)?,
            keycloak_id: keycloak_user.id,
            username,
            email,
            first_name: keycloak_user.first_name,
            last_name: keycloak_user.last_name,
            phone_number: PhoneNumber::new("")?, // Keycloak doesn't store phone numbers by default
            avatar_url: None,
            is_active: keycloak_user.enabled,
            is_email_verified: keycloak_user.email_verified,
            last_login_at: None, // Would need to fetch from Keycloak sessions
            created_at,
            updated_at: created_at, // Keycloak doesn't provide updated_at
        };

        Ok(user)
    }

    fn to_keycloak_user(&self, user: &User) -> CreateKeycloakUser {
        CreateKeycloakUser {
            username: user.username().to_string(),
            email: user.email().to_string(),
            first_name: user.first_name().map(|s| s.to_string()),
            last_name: user.last_name().map(|s| s.to_string()),
            enabled: user.is_active(),
            credentials: Vec::new(), // No password for admin-created users
            attributes: None,
        }
    }

    fn to_update_keycloak_user(&self, user: &User) -> UpdateKeycloakUser {
        UpdateKeycloakUser {
            username: user.username().to_string(),
            email: user.email().to_string(),
            first_name: user.first_name().map(|s| s.to_string()),
            last_name: user.last_name().map(|s| s.to_string()),
            enabled: user.is_active(),
        }
    }
}

#[async_trait]
impl UserRepository for KeycloakUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        match self.keycloak_client.get_user_by_id(id.as_str()).await {
            Ok(Some(keycloak_user)) => {
                let user = self.to_domain_user(keycloak_user)?;
                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(DomainError::Repository(format!("Keycloak error: {}", e))),
        }
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError> {
        match self.keycloak_client.get_user_by_id(keycloak_id).await {
            Ok(Some(keycloak_user)) => {
                let user = self.to_domain_user(keycloak_user)?;
                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(DomainError::Repository(format!("Keycloak error: {}", e))),
        }
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        match self.keycloak_client.get_user_by_email(email.as_str()).await {
            Ok(Some(keycloak_user)) => {
                let user = self.to_domain_user(keycloak_user)?;
                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(DomainError::Repository(format!("Keycloak error: {}", e))),
        }
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError> {
        match self.keycloak_client.get_user_by_username(username.as_str()).await {
            Ok(Some(keycloak_user)) => {
                let user = self.to_domain_user(keycloak_user)?;
                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(DomainError::Repository(format!("Keycloak error: {}", e))),
        }
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        // Check if user exists in Keycloak
        if let Some(existing_user) = self.find_by_keycloak_id(user.keycloak_id()).await? {
            // Update existing user
            let update_user = self.to_update_keycloak_user(user);
            self.keycloak_client
                .update_user(user.keycloak_id(), &update_user)
                .await
                .map_err(|e| DomainError::Repository(format!("Failed to update user in Keycloak: {}", e)))?;
        } else {
            // Create new user
            let create_user = self.to_keycloak_user(user);
            self.keycloak_client
                .create_user(&create_user)
                .await
                .map_err(|e| DomainError::Repository(format!("Failed to create user in Keycloak: {}", e)))?;
        }
        
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        self.keycloak_client
            .delete_user(id.as_str())
            .await
            .map_err(|e| DomainError::Repository(format!("Failed to delete user from Keycloak: {}", e)))?;
        
        Ok(())
    }

    async fn list_users(&self, page: u32, page_size: u32, active_only: bool) -> Result<Vec<User>, DomainError> {
        let first = (page - 1) * page_size;
        let max = page_size;

        let keycloak_users = self.keycloak_client
            .list_users(Some(first), Some(max))
            .await
            .map_err(|e| DomainError::Repository(format!("Failed to list users from Keycloak: {}", e)))?;

        let mut users = Vec::new();
        for keycloak_user in keycloak_users {
            if active_only && !keycloak_user.enabled {
                continue;
            }
            
            match self.to_domain_user(keycloak_user) {
                Ok(user) => users.push(user),
                Err(e) => {
                    // Log conversion error but continue with other users
                    eprintln!("Failed to convert Keycloak user to domain user: {}", e);
                }
            }
        }

        Ok(users)
    }
}