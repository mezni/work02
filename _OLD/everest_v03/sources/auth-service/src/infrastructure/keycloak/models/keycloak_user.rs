// src/infrastructure/keycloak/models/keycloak_user.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeycloakUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub enabled: bool,
    pub email_verified: bool,
    pub attributes: Option<std::collections::HashMap<String, Vec<String>>>,
    pub created_timestamp: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CreateKeycloakUser {
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub enabled: bool,
    pub credentials: Vec<KeycloakCredential>,
    pub attributes: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize)]
pub struct KeycloakCredential {
    #[serde(rename = "type")]
    pub cred_type: String,
    pub value: String,
    pub temporary: bool,
}

#[derive(Debug, Serialize)]
pub struct UpdateKeycloakUser {
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub enabled: bool,
}