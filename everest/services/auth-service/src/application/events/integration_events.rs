use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Integration events for communicating with other microservices

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreatedIntegrationEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdatedIntegrationEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleChangedIntegrationEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub old_role: String,
    pub new_role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyCreatedIntegrationEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyUpdatedIntegrationEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

// Event mapper to convert domain events to integration events
pub struct IntegrationEventMapper;

impl IntegrationEventMapper {
    pub fn map_user_created(
        user_id: Uuid,
        username: String,
        email: String,
        role: String,
        company_id: Option<Uuid>,
    ) -> UserCreatedIntegrationEvent {
        UserCreatedIntegrationEvent {
            event_id: Uuid::new_v4(),
            event_type: "user.created".to_string(),
            occurred_at: Utc::now(),
            user_id,
            username,
            email,
            role,
            company_id,
        }
    }

    pub fn map_user_updated(
        user_id: Uuid,
        username: String,
        email: String,
        role: String,
        company_id: Option<Uuid>,
    ) -> UserUpdatedIntegrationEvent {
        UserUpdatedIntegrationEvent {
            event_id: Uuid::new_v4(),
            event_type: "user.updated".to_string(),
            occurred_at: Utc::now(),
            user_id,
            username,
            email,
            role,
            company_id,
        }
    }

    pub fn map_user_role_changed(
        user_id: Uuid,
        old_role: String,
        new_role: String,
    ) -> UserRoleChangedIntegrationEvent {
        UserRoleChangedIntegrationEvent {
            event_id: Uuid::new_v4(),
            event_type: "user.role_changed".to_string(),
            occurred_at: Utc::now(),
            user_id,
            old_role,
            new_role,
        }
    }

    pub fn map_company_created(
        company_id: Uuid,
        name: String,
        description: Option<String>,
        created_by: Uuid,
    ) -> CompanyCreatedIntegrationEvent {
        CompanyCreatedIntegrationEvent {
            event_id: Uuid::new_v4(),
            event_type: "company.created".to_string(),
            occurred_at: Utc::now(),
            company_id,
            name,
            description,
            created_by,
        }
    }

    pub fn map_company_updated(
        company_id: Uuid,
        name: String,
        description: Option<String>,
    ) -> CompanyUpdatedIntegrationEvent {
        CompanyUpdatedIntegrationEvent {
            event_id: Uuid::new_v4(),
            event_type: "company.updated".to_string(),
            occurred_at: Utc::now(),
            company_id,
            name,
            description,
        }
    }
}