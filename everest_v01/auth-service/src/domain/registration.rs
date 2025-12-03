use serde::{Serialize, Deserialize};
use validator::Validate;
use super::value_objects::{Email, Password};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegistrationRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    #[validate(must_match = "password")]
    pub confirm_password: String,
}

impl RegistrationRequest {
    pub fn validate_domain(&self) -> Result<(), super::error::DomainError> {
        // Validate using validator crate
        self.validate()
            .map_err(|e| super::error::DomainError::InvalidRegistration(e.to_string()))?;
        
        // Additional domain validation
        if self.password != self.confirm_password {
            return Err(super::error::DomainError::InvalidRegistration(
                "Passwords do not match".to_string()
            ));
        }
        
        Ok(())
    }
    
    pub fn to_email(&self) -> Result<Email, super::error::DomainError> {
        Email::new(self.email.clone())
    }
    
    pub fn to_password(&self) -> Result<Password, super::error::DomainError> {
        Password::new(&self.password)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    pub email: String,
    pub role: String,
    pub company_name: String,
    pub station_name: String,
    pub attributes: Vec<(String, String)>,
}

impl Registration {
    pub fn new(email: String) -> Self {
        Self {
            email,
            role: "user".to_string(),
            company_name: String::new(),
            station_name: String::new(),
            attributes: vec![],
        }
    }
    
    pub fn with_role(mut self, role: &str) -> Self {
        self.role = role.to_string();
        self
    }
    
    pub fn with_company_name(mut self, company_name: String) -> Self {
        self.company_name = company_name;
        self
    }
    
    pub fn with_station_name(mut self, station_name: String) -> Self {
        self.station_name = station_name;
        self
    }
    
    pub fn add_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.push((key, value));
        self
    }
    
    pub fn to_keycloak_user(&self) -> KeycloakUserRepresentation {
        KeycloakUserRepresentation {
            username: self.email.clone(),
            email: self.email.clone(),
            enabled: true,
            email_verified: false,
            attributes: Some(
                self.attributes
                    .iter()
                    .map(|(k, v)| (k.clone(), vec![v.clone()]))
                    .collect()
            ),
            credentials: vec![],
            realm_roles: vec![self.role.clone()],
        }
    }
}

// Keycloak specific representation
#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakUserRepresentation {
    pub username: String,
    pub email: String,
    pub enabled: bool,
    pub email_verified: bool,
    pub attributes: Option<std::collections::HashMap<String, Vec<String>>>,
    pub credentials: Vec<KeycloakCredential>,
    pub realm_roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakCredential {
    #[serde(rename = "type")]
    pub cred_type: String,
    pub value: String,
    pub temporary: bool,
}