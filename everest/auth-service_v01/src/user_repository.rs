use crate::keycloak_client::KeycloakClient;
use crate::user_dto::ServiceError; // 💥 CHANGE: Import ServiceError
use crate::user_entity::User;
use keycloak::types::{CredentialRepresentation, UserRepresentation};
//use keycloak::errors::Error as KeycloakError; // 💥 CHANGE: Import KeycloakError
use keycloak::Error as KeycloakError;
use reqwest::Client;

pub struct UserRepository {
    keycloak_client: KeycloakClient,
}

impl UserRepository {
    pub fn new(keycloak_client: KeycloakClient) -> Self {
        Self { keycloak_client }
    }

    pub async fn create_user(
        &self,
        user: User,
        password: String,
        role_name: &str,
        client: &Client,
    ) -> Result<Option<String>, ServiceError> { // 💥 CHANGE: Result uses ServiceError
        let (admin, _token) = self.keycloak_client.get_admin_client(client).await.map_err(ServiceError::Internal)?;
        let realm = self.keycloak_client.get_realm();

        let user_rep = UserRepresentation {
            username: Some(user.username.clone()),
            email: Some(user.email.clone()),
            first_name: Some(user.first_name.clone()),
            last_name: Some(user.last_name.clone()),
            enabled: Some(user.enabled),
            email_verified: Some(user.email_verified),
            realm_roles: Some(vec![role_name.to_string()]),
            attributes: Some(user.attributes.clone()),
            credentials: Some(vec![CredentialRepresentation {
                r#type_: Some("password".to_string()),
                value: Some(password),
                temporary: Some(false),
                ..Default::default()
            }]),
            ..Default::default()
        };

        match admin.realm_users_post(realm, user_rep).await {
            Ok(response) => {
                let user_id = response.to_id().map(|id| id.to_string());
                Ok(user_id)
            },
            Err(KeycloakError::Keycloak(e)) if e.status == 409 => {
                // Keycloak returns 409 Conflict if the user/email already exists
                Err(ServiceError::Conflict(format!("User creation failed: {}", e.error_message.unwrap_or_else(|| "User already exists".to_string()))))
            },
            Err(e) => {
                // Map all other errors (400, 500, network errors, etc.) to internal
                Err(ServiceError::Internal(format!("Failed to create user in Keycloak: {:?}", e)))
            }
        }
    }

    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
        client: &Client,
    ) -> Result<crate::keycloak_client::TokenResponse, ServiceError> { // 💥 CHANGE: Result uses ServiceError
        self.keycloak_client
            .authenticate_user(username, password, client)
            .await
            .map_err(|e| ServiceError::Unauthorized(e)) // Authentication failure is Unauthorized
    }
}