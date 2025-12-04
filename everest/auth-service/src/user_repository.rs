use crate::keycloak_client::{CreateUserDto, KeycloakClient, RoleMapping};
use std::collections::HashMap;
use std::error::Error;

pub struct UserRepository {
    keycloak: KeycloakClient,
}

impl UserRepository {
    pub fn new(keycloak: KeycloakClient) -> Self {
        Self { keycloak }
    }

    pub async fn create_user(
        &self,
        username: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
        attributes: Option<HashMap<String, Vec<String>>>,
    ) -> Result<String, Box<dyn Error>> {
        let new_user = CreateUserDto {
            username: username.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            enabled: true,
            credentials: vec![crate::keycloak_client::Credential {
                cred_type: "password".to_string(),
                value: password.to_string(),
                temporary: false,
            }],
            attributes,
        };

        self.keycloak.create_user(&new_user).await?;
        let user = self.keycloak.get_user_by_username(username).await?;
        let role: RoleMapping = self.keycloak.get_role("user").await?;
        self.keycloak.assign_realm_role(&user.id, &role).await?;

        Ok(user.id)
    }
}
