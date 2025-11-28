// Keycloak integration module
// This would contain the actual Keycloak client implementation
// For now, we'll use a simple authentication system
pub mod keycloak_client;

pub use keycloak_client::KeycloakClient;
