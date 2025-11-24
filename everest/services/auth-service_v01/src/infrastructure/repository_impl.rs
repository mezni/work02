use crate::domain::repository::UserRepository;
use crate::domain::models::User;
use crate::infrastructure::keycloak::KeycloakClient;
use anyhow::Result;
#[derive(Clone)]
pub struct UserRepositoryKeycloak {
    pub client: KeycloakClient,
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryKeycloak {
    async fn create_user(&self, user: &User, password: &str) -> Result<String> {
        self.client.create_user(user, password).await
    }

    async fn find_by_username(&self, _username: &str) -> Result<Option<User>> {
        // fetch from Keycloak users
        Ok(None)
    }

    async fn assign_role(&self, user_id: &str, role: &str) -> Result<()> {
        self.client.assign_role(user_id, role).await
    }
}
