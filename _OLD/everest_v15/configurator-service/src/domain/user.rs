use super::types::{
    AuditInfo, OrganizationId, StationId, UserId, UserRole, UserStatus, validate_display_name,
    validate_email,
};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub display_name: String,
    pub role: UserRole,
    pub organization_id: Option<OrganizationId>,
    pub station_id: Option<StationId>,
    pub status: UserStatus,
    pub audit: AuditInfo,
}

impl User {
    pub fn new(
        email: String,
        display_name: String,
        role: UserRole,
        organization_id: Option<OrganizationId>,
        station_id: Option<StationId>,
        created_by: UserId,
    ) -> Result<Self, String> {
        // Validate input
        validate_email(&email)?;
        validate_display_name(&display_name)?;

        // Validate business rules
        Self::validate_creation(&role, organization_id, station_id)?;

        Ok(Self {
            id: UserId::new_v4(),
            email: email.to_lowercase(), // Normalize email
            display_name: display_name.trim().to_string(),
            role,
            organization_id,
            station_id,
            status: UserStatus::Pending,
            audit: AuditInfo::new(created_by),
        })
    }

    fn validate_creation(
        role: &UserRole,
        organization_id: Option<OrganizationId>,
        station_id: Option<StationId>,
    ) -> Result<(), String> {
        match role {
            UserRole::SuperAdmin => {
                if organization_id.is_some() || station_id.is_some() {
                    return Err("Super admin cannot have organization or station".to_string());
                }
            }
            UserRole::Partner => {
                if organization_id.is_none() {
                    return Err("Partner must have an organization".to_string());
                }
                if station_id.is_some() {
                    return Err("Partner cannot have a station".to_string());
                }
            }
            UserRole::Operator => {
                if organization_id.is_none() || station_id.is_none() {
                    return Err("Operator must have both organization and station".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn activate(&mut self, updated_by: UserId) -> Result<(), String> {
        if self.status == UserStatus::Deleted {
            return Err("Cannot activate a deleted user".to_string());
        }

        self.status = UserStatus::Active;
        self.audit.update(updated_by);
        Ok(())
    }

    pub fn deactivate(&mut self, updated_by: UserId) -> Result<(), String> {
        if self.status == UserStatus::Deleted {
            return Err("Cannot deactivate a deleted user".to_string());
        }

        self.status = UserStatus::Inactive;
        self.audit.update(updated_by);
        Ok(())
    }

    pub fn mark_deleted(&mut self, updated_by: UserId) {
        self.status = UserStatus::Deleted;
        self.audit.update(updated_by);
    }

    pub fn update_email(&mut self, new_email: String, updated_by: UserId) -> Result<(), String> {
        validate_email(&new_email)?;
        self.email = new_email.to_lowercase();
        self.audit.update(updated_by);
        Ok(())
    }

    pub fn update_display_name(
        &mut self,
        new_display_name: String,
        updated_by: UserId,
    ) -> Result<(), String> {
        validate_display_name(&new_display_name)?;
        self.display_name = new_display_name.trim().to_string();
        self.audit.update(updated_by);
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }

    pub fn can_manage_organization(&self, organization_id: OrganizationId) -> bool {
        match self.role {
            UserRole::SuperAdmin => true,
            UserRole::Partner => self.organization_id == Some(organization_id),
            UserRole::Operator => false, // Operators cannot manage organizations
        }
    }

    pub fn can_manage_station(&self, station_id: StationId) -> bool {
        match self.role {
            UserRole::SuperAdmin => true,
            UserRole::Partner => true, // Partners can manage all stations in their org
            UserRole::Operator => self.station_id == Some(station_id),
        }
    }
}

// Unit tests for User domain model
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_user_id() -> UserId {
        Uuid::new_v4()
    }

    #[test]
    fn test_create_super_admin_success() {
        let creator_id = create_test_user_id();
        let user = User::new(
            "admin@example.com".to_string(),
            "System Admin".to_string(),
            UserRole::SuperAdmin,
            None,
            None,
            creator_id,
        )
        .unwrap();

        assert_eq!(user.email, "admin@example.com");
        assert_eq!(user.role, UserRole::SuperAdmin);
        assert_eq!(user.organization_id, None);
        assert_eq!(user.station_id, None);
        assert_eq!(user.status, UserStatus::Pending);
    }

    #[test]
    fn test_create_super_admin_with_organization_fails() {
        let creator_id = create_test_user_id();
        let org_id = Uuid::new_v4();

        let result = User::new(
            "admin@example.com".to_string(),
            "System Admin".to_string(),
            UserRole::SuperAdmin,
            Some(org_id),
            None,
            creator_id,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot have organization"));
    }

    #[test]
    fn test_create_partner_without_organization_fails() {
        let creator_id = create_test_user_id();

        let result = User::new(
            "partner@example.com".to_string(),
            "Business Partner".to_string(),
            UserRole::Partner,
            None,
            None,
            creator_id,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must have an organization"));
    }

    #[test]
    fn test_create_operator_without_station_fails() {
        let creator_id = create_test_user_id();
        let org_id = Uuid::new_v4();

        let result = User::new(
            "operator@example.com".to_string(),
            "Station Operator".to_string(),
            UserRole::Operator,
            Some(org_id),
            None,
            creator_id,
        );

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("must have both organization and station")
        );
    }

    #[test]
    fn test_user_activation() {
        let creator_id = create_test_user_id();
        let updater_id = create_test_user_id();
        let mut user = User::new(
            "user@example.com".to_string(),
            "Test User".to_string(),
            UserRole::SuperAdmin,
            None,
            None,
            creator_id,
        )
        .unwrap();

        assert!(user.activate(updater_id).is_ok());
        assert_eq!(user.status, UserStatus::Active);
        assert_eq!(user.audit.updated_by, updater_id);
    }

    #[test]
    fn test_email_normalization() {
        let creator_id = create_test_user_id();
        let user = User::new(
            "Test.User@Example.COM".to_string(),
            "Test User".to_string(),
            UserRole::SuperAdmin,
            None,
            None,
            creator_id,
        )
        .unwrap();

        assert_eq!(user.email, "test.user@example.com");
    }

    #[test]
    fn test_invalid_email_rejected() {
        let creator_id = create_test_user_id();

        let result = User::new(
            "invalid-email".to_string(),
            "Test User".to_string(),
            UserRole::SuperAdmin,
            None,
            None,
            creator_id,
        );

        assert!(result.is_err());
    }
}
