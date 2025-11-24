use crate::domain::value_objects::{Email, Password, CompanyName};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Email,
    pub company: CompanyName,
    pub role: String,
}
