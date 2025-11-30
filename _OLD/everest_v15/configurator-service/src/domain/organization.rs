use super::types::{AuditInfo, OrganizationId, OrganizationStatus, UserId, validate_display_name};

#[derive(Debug, Clone)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: String,
    pub status: OrganizationStatus,
    pub audit: AuditInfo,
}

impl Organization {
    /// Create a new organization
    pub fn new(name: String, created_by: UserId) -> Result<Self, String> {
        // Validate input
        validate_display_name(&name)?;

        Ok(Self {
            id: OrganizationId::new_v4(),
            name: name.trim().to_string(),
            status: OrganizationStatus::Active,
            audit: AuditInfo::new(created_by),
        })
    }

    /// Activate the organization
    pub fn activate(&mut self, updated_by: UserId) -> Result<(), String> {
        if self.status == OrganizationStatus::Active {
            return Err("Organization is already active".to_string());
        }

        self.status = OrganizationStatus::Active;
        self.audit.update(updated_by);
        Ok(())
    }

    /// Deactivate the organization
    pub fn deactivate(&mut self, updated_by: UserId) -> Result<(), String> {
        if self.status == OrganizationStatus::Inactive {
            return Err("Organization is already inactive".to_string());
        }

        self.status = OrganizationStatus::Inactive;
        self.audit.update(updated_by);
        Ok(())
    }

    /// Update organization name
    pub fn update_name(&mut self, new_name: String, updated_by: UserId) -> Result<(), String> {
        validate_display_name(&new_name)?;

        self.name = new_name.trim().to_string();
        self.audit.update(updated_by);
        Ok(())
    }

    /// Check if organization is active
    pub fn is_active(&self) -> bool {
        self.status == OrganizationStatus::Active
    }

    /// Validate if the organization can perform operations
    pub fn can_perform_operations(&self) -> Result<(), String> {
        if !self.is_active() {
            return Err("Organization is inactive and cannot perform operations".to_string());
        }
        Ok(())
    }
}

// Organization domain events
#[derive(Debug, Clone)]
pub enum OrganizationEvent {
    Created {
        organization_id: OrganizationId,
        name: String,
        created_by: UserId,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    Activated {
        organization_id: OrganizationId,
        activated_by: UserId,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    Deactivated {
        organization_id: OrganizationId,
        deactivated_by: UserId,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    NameUpdated {
        organization_id: OrganizationId,
        old_name: String,
        new_name: String,
        updated_by: UserId,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}

impl OrganizationEvent {
    pub fn organization_created(organization: &Organization) -> Self {
        OrganizationEvent::Created {
            organization_id: organization.id,
            name: organization.name.clone(),
            created_by: organization.audit.created_by,
            timestamp: organization.audit.created_at,
        }
    }

    pub fn organization_activated(organization: &Organization, activated_by: UserId) -> Self {
        OrganizationEvent::Activated {
            organization_id: organization.id,
            activated_by,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn organization_deactivated(organization: &Organization, deactivated_by: UserId) -> Self {
        OrganizationEvent::Deactivated {
            organization_id: organization.id,
            deactivated_by,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn organization_name_updated(
        organization: &Organization,
        old_name: String,
        updated_by: UserId,
    ) -> Self {
        OrganizationEvent::NameUpdated {
            organization_id: organization.id,
            old_name,
            new_name: organization.name.clone(),
            updated_by,
            timestamp: chrono::Utc::now(),
        }
    }
}

// Organization domain service for complex business logic
pub struct OrganizationService;

impl OrganizationService {
    /// Validate if an organization can be deleted
    /// In a real application, this would check for dependencies like users, stations, etc.
    pub fn can_delete_organization(_organization_id: OrganizationId) -> Result<(), String> {
        // TODO: Implement actual validation logic
        // For now, we'll assume organizations can always be "soft deleted" via deactivation
        Ok(())
    }

    /// Validate organization name uniqueness
    /// This would typically check against a repository
    pub fn is_name_unique(_name: &str) -> Result<(), String> {
        // TODO: Implement actual uniqueness check against database
        // For now, we'll assume all names are unique
        Ok(())
    }

    /// Calculate organization statistics
    /// This would typically aggregate data from multiple sources
    pub fn calculate_statistics(_organization_id: OrganizationId) -> OrganizationStatistics {
        // TODO: Implement actual statistics calculation
        OrganizationStatistics::default()
    }
}

/// Organization statistics
#[derive(Debug, Clone)]
pub struct OrganizationStatistics {
    pub total_users: u32,
    pub active_users: u32,
    pub total_stations: u32,
    pub active_stations: u32,
    pub total_charging_sessions: u64,
    pub total_revenue: f64,
}

impl Default for OrganizationStatistics {
    fn default() -> Self {
        Self {
            total_users: 0,
            active_users: 0,
            total_stations: 0,
            active_stations: 0,
            total_charging_sessions: 0,
            total_revenue: 0.0,
        }
    }
}

// Organization value objects
#[derive(Debug, Clone)]
pub struct OrganizationSummary {
    pub id: OrganizationId,
    pub name: String,
    pub status: OrganizationStatus,
    pub total_users: u32,
    pub total_stations: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl OrganizationSummary {
    pub fn from_organization(
        organization: &Organization,
        total_users: u32,
        total_stations: u32,
    ) -> Self {
        Self {
            id: organization.id,
            name: organization.name.clone(),
            status: organization.status.clone(),
            total_users,
            total_stations,
            created_at: organization.audit.created_at,
        }
    }
}

// Unit tests for Organization domain model
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_user_id() -> UserId {
        Uuid::new_v4()
    }

    #[test]
    fn test_create_organization_success() {
        let creator_id = create_test_user_id();
        let organization = Organization::new("EV Charging Corp".to_string(), creator_id).unwrap();

        assert_eq!(organization.name, "EV Charging Corp");
        assert_eq!(organization.status, OrganizationStatus::Active);
        assert_eq!(organization.audit.created_by, creator_id);
    }

    #[test]
    fn test_create_organization_with_empty_name_fails() {
        let creator_id = create_test_user_id();

        let result = Organization::new("   ".to_string(), creator_id);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_organization_activation() {
        let creator_id = create_test_user_id();
        let updater_id = create_test_user_id();
        let mut organization = Organization::new("Test Org".to_string(), creator_id).unwrap();

        // Try to activate an already active organization
        let result = organization.activate(updater_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already active"));

        // Deactivate then activate
        organization.deactivate(updater_id).unwrap();
        assert_eq!(organization.status, OrganizationStatus::Inactive);

        organization.activate(updater_id).unwrap();
        assert_eq!(organization.status, OrganizationStatus::Active);
        assert_eq!(organization.audit.updated_by, updater_id);
    }

    #[test]
    fn test_organization_deactivation() {
        let creator_id = create_test_user_id();
        let updater_id = create_test_user_id();
        let mut organization = Organization::new("Test Org".to_string(), creator_id).unwrap();

        organization.deactivate(updater_id).unwrap();
        assert_eq!(organization.status, OrganizationStatus::Inactive);

        // Try to deactivate an already inactive organization
        let result = organization.deactivate(updater_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already inactive"));
    }

    #[test]
    fn test_organization_name_update() {
        let creator_id = create_test_user_id();
        let updater_id = create_test_user_id();
        let mut organization = Organization::new("Old Name".to_string(), creator_id).unwrap();

        organization
            .update_name("New Name".to_string(), updater_id)
            .unwrap();
        assert_eq!(organization.name, "New Name");
        assert_eq!(organization.audit.updated_by, updater_id);
    }

    #[test]
    fn test_organization_name_trimming() {
        let creator_id = create_test_user_id();
        let organization =
            Organization::new("   EV Charging Corp   ".to_string(), creator_id).unwrap();

        assert_eq!(organization.name, "EV Charging Corp");
    }

    #[test]
    fn test_organization_operations_check() {
        let creator_id = create_test_user_id();
        let updater_id = create_test_user_id();
        let mut organization = Organization::new("Test Org".to_string(), creator_id).unwrap();

        // Active organization can perform operations
        assert!(organization.can_perform_operations().is_ok());

        // Inactive organization cannot perform operations
        organization.deactivate(updater_id).unwrap();
        let result = organization.can_perform_operations();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("inactive"));
    }

    #[test]
    fn test_organization_event_creation() {
        let creator_id = create_test_user_id();
        let organization = Organization::new("Event Test Org".to_string(), creator_id).unwrap();

        let created_event = OrganizationEvent::organization_created(&organization);

        match created_event {
            OrganizationEvent::Created {
                organization_id,
                name,
                created_by,
                ..
            } => {
                assert_eq!(organization_id, organization.id);
                assert_eq!(name, "Event Test Org");
                assert_eq!(created_by, creator_id);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_organization_summary_creation() {
        let creator_id = create_test_user_id();
        let organization = Organization::new("Summary Test Org".to_string(), creator_id).unwrap();

        let summary = OrganizationSummary::from_organization(&organization, 10, 5);

        assert_eq!(summary.id, organization.id);
        assert_eq!(summary.name, "Summary Test Org");
        assert_eq!(summary.total_users, 10);
        assert_eq!(summary.total_stations, 5);
        assert_eq!(summary.status, OrganizationStatus::Active);
    }
}
