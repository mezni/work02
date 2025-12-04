use dotenvy::dotenv;
use std::env;

/// Load environment variables from `.env`
pub fn init() {
    dotenv().ok();
}

/// Keycloak token URL
pub fn token_url() -> String {
    env::var("KC_TOKEN_URL")
        .unwrap_or("http://localhost:5080/realms/myrealm/protocol/openid-connect/token".into())
}

/// Keycloak admin client ID
pub fn admin_client() -> String {
    env::var("KC_ADMIN_CLIENT").unwrap_or("backend-admin".into())
}

/// Keycloak admin secret
pub fn admin_secret() -> String {
    env::var("KC_ADMIN_SECRET").unwrap_or("backend-admin-secret".into())
}

/// Keycloak user management URL
pub fn user_url() -> String {
    env::var("KC_USER_URL").unwrap_or("http://localhost:5080/admin/realms/myrealm/users".into())
}

/// Keycloak public client for login
pub fn public_client() -> String {
    env::var("PUBLIC_CLIENT_ID").unwrap_or("auth-client".into())
}
