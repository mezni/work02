use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub password: String,
    #[serde(rename = "companyName", default = "default_company_name")]
    pub company_name: String,
    #[serde(rename = "stationName", default = "default_station_name")]
    pub station_name: String,
}

fn default_company_name() -> String {
    "X".to_string()
}

fn default_station_name() -> String {
    "X".to_string()
}

#[derive(Debug, Deserialize)]
pub struct AuthDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserCreatedResponse {
    pub message: String,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub role: String,
    pub attributes: UserAttributesResponse,
}

#[derive(Debug, Serialize)]
pub struct UserAttributesResponse {
    pub company_name: String,
    pub station_name: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}