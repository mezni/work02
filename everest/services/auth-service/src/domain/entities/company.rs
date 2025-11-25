use crate::domain::errors::DomainError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Company {
    pub fn new(
        name: String,
        description: Option<String>,
        created_by: Uuid,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Company name cannot be empty".to_string(),
            ));
        }

        if name.len() > 255 {
            return Err(DomainError::Validation("Company name too long".to_string()));
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            name: name.trim().to_string(),
            description: description.map(|d| d.trim().to_string()),
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn update(
        &mut self,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<(), DomainError> {
        if let Some(name) = name {
            if name.trim().is_empty() {
                return Err(DomainError::Validation(
                    "Company name cannot be empty".to_string(),
                ));
            }
            if name.len() > 255 {
                return Err(DomainError::Validation("Company name too long".to_string()));
            }
            self.name = name.trim().to_string();
        }

        if let Some(description) = description {
            self.description = Some(description.trim().to_string());
        }

        self.updated_at = Utc::now();
        Ok(())
    }
}
