use serde::{Deserialize, Serialize};

/// Keycloak realm representation
#[derive(Debug, Serialize, Deserialize)]
pub struct Realm {
    pub id: String,
    pub realm: String,
    pub enabled: bool,
    pub display_name: Option<String>,
}

/// Keycloak client representation
#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub client_id: String,
    pub name: Option<String>,
    pub enabled: bool,
    pub secret: Option<String>,
}

/// Keycloak role representation
#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

/// Keycloak group representation
#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub path: String,
}

/// Keycloak user session
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub id: String,
    pub username: String,
    pub user_id: String,
    pub ip_address: String,
    pub start_time: i64,
    pub last_access: i64,
}

/// Keycloak event representation
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub time: i64,
    pub r#type: String,
    pub realm_id: String,
    pub client_id: Option<String>,
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
    pub details: Option<serde_json::Value>,
}