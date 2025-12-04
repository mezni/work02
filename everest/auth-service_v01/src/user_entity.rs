use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub enabled: bool,
    pub email_verified: bool,
    pub attributes: HashMap<String, Vec<String>>,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        first_name: String,
        last_name: String,
        company_name: String,
        station_name: String,
    ) -> Self {
        let mut attributes = HashMap::new();
        attributes.insert("company_name".to_string(), vec![company_name]);
        attributes.insert("station_name".to_string(), vec![station_name]);

        Self {
            id: None,
            username,
            email,
            first_name,
            last_name,
            enabled: true,
            email_verified: true,
            attributes,
        }
    }

    pub fn get_company_name(&self) -> Option<&str> {
        self.attributes
            .get("company_name")
            .and_then(|v| v.first())
            .map(|s| s.as_str())
    }

    pub fn get_station_name(&self) -> Option<&str> {
        self.attributes
            .get("station_name")
            .and_then(|v| v.first())
            .map(|s| s.as_str())
    }
}