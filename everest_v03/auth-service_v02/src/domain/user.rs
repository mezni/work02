use crate::domain::errors::DomainError;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,               // Mandatory ID (UUID or string)
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password_hash: String,     // Domain stores only hashed password
    pub role: String,              // default: "USER"
    pub company_name: String,      // default: ""
    pub station_name: String,      // default: ""
}

impl User {
    /// Default role and empty company/station constants
    pub const DEFAULT_ROLE: &'static str = "USER";
    pub const DEFAULT_COMPANY: &'static str = "";
    pub const DEFAULT_STATION: &'static str = "";

    /// Create a new User aggregate using default role, company_name, station_name
    pub fn new(
        id: String,
        username: String,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
        password_hash: String,
    ) -> Result<Self, DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::ValidationError("id cannot be empty".into()));
        }

        if username.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "username cannot be empty".into(),
            ));
        }

        if !email.contains('@') {
            return Err(DomainError::ValidationError(
                "email must contain '@'".into(),
            ));
        }

        Ok(Self {
            id,
            username,
            email,
            first_name,
            last_name,
            password_hash,
            role: Self::DEFAULT_ROLE.to_string(),
            company_name: Self::DEFAULT_COMPANY.to_string(),
            station_name: Self::DEFAULT_STATION.to_string(),
        })
    }

    /// Create a fully custom User with all fields
    pub fn new_full(
        id: String,
        username: String,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
        password_hash: String,
        role: String,
        company_name: String,
        station_name: String,
    ) -> Result<Self, DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::ValidationError("id cannot be empty".into()));
        }

        if username.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "username cannot be empty".into(),
            ));
        }

        if !email.contains('@') {
            return Err(DomainError::ValidationError(
                "email must contain '@'".into(),
            ));
        }

        Ok(Self {
            id,
            username,
            email,
            first_name,
            last_name,
            password_hash,
            role,
            company_name,
            station_name,
        })
    }
}
