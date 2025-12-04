use crate::keycloak_client::{CreateUserDto, KeycloakClient, RoleMapping};
use std::collections::HashMap;

pub struct UserRepository {
    kc: KeycloakClient,
}

impl UserRepository {
    pub fn new(kc: KeycloakClient) -> Self {
        Self { kc }
    }

    pub async fn register_user(
        &self,
        username: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
        attributes: Option<HashMap<String, Vec<String>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 1️⃣ Create user
        let user = CreateUserDto {
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

        self.kc.create_user(&user).await?;

        // 2️⃣ Fetch user ID
        let user_info = self.kc.get_user_by_username(username).await?;
        let user_id = user_info.id.clone();

        // 3️⃣ Assign default role "user"
        let role: RoleMapping = self.kc.get_role("user").await?;
        self.kc.assign_realm_role(&user_id, &role).await?;

        Ok(user_id)
    }
}
