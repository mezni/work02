use crate::user_dto::{CreateUserDto, RoleMapping};
use crate::keycloak_client::KeycloakClient;
use std::error::Error;

pub struct UserRepository {
    kc_client: KeycloakClient,
}

impl UserRepository {
    pub fn new(kc_client: KeycloakClient) -> Self {
        Self { kc_client }
    }

    pub async fn register_user(
        &self,
        token: &str,
        user: &CreateUserDto,
        role: &RoleMapping,
    ) -> Result<(), Box<dyn Error>> {
        self.kc_client.create_user(token, user).await?;
        let user_id = self.kc_client.get_user_id(token, &user.username).await?;
        self.kc_client.assign_role(token, &user_id, role).await?;
        Ok(())
    }
}
