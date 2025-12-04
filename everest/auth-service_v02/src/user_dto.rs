use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Credential {
    #[serde(rename = "type")]
    pub cred_type: String,
    pub value: String,
    pub temporary: bool,
}

#[derive(Serialize)]
pub struct CreateUserDto {
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub enabled: bool,
    pub credentials: Vec<Credential>,
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Deserialize)]
pub struct KeycloakUser {
    pub id: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoleMapping {
    pub id: String,
    pub name: String,
}
