// configurator-service/src/application/commands.rs
use uuid::Uuid;

use crate::domain::types::{OrganizationId, StationId, UserId, UserRole};

// User Commands
#[derive(Debug, Clone)]
pub struct CreateUserCommand {
    pub email: String,
    pub display_name: String,
    pub role: UserRole,
    pub organization_id: Option<OrganizationId>,
    pub station_id: Option<StationId>,
    pub created_by: UserId,
}

#[derive(Debug, Clone)]
pub struct UpdateUserCommand {
    pub user_id: UserId,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub updated_by: UserId,
}

#[derive(Debug, Clone)]
pub struct ActivateUserCommand {
    pub user_id: UserId,
    pub activated_by: UserId,
}

#[derive(Debug, Clone)]
pub struct DeactivateUserCommand {
    pub user_id: UserId,
    pub deactivated_by: UserId,
}

#[derive(Debug, Clone)]
pub struct DeleteUserCommand {
    pub user_id: UserId,
    pub deleted_by: UserId,
}

// Organization Commands
#[derive(Debug, Clone)]
pub struct CreateOrganizationCommand {
    pub name: String,
    pub created_by: UserId,
}

#[derive(Debug, Clone)]
pub struct UpdateOrganizationCommand {
    pub organization_id: OrganizationId,
    pub name: Option<String>,
    pub updated_by: UserId,
}

#[derive(Debug, Clone)]
pub struct ActivateOrganizationCommand {
    pub organization_id: OrganizationId,
    pub activated_by: UserId,
}

#[derive(Debug, Clone)]
pub struct DeactivateOrganizationCommand {
    pub organization_id: OrganizationId,
    pub deactivated_by: UserId,
}

// Station Commands
#[derive(Debug, Clone)]
pub struct CreateStationCommand {
    pub name: String,
    pub location: Option<String>,
    pub organization_id: OrganizationId,
    pub created_by: UserId,
}

#[derive(Debug, Clone)]
pub struct UpdateStationCommand {
    pub station_id: StationId,
    pub name: Option<String>,
    pub location: Option<String>,
    pub updated_by: UserId,
}

#[derive(Debug, Clone)]
pub struct ActivateStationCommand {
    pub station_id: StationId,
    pub activated_by: UserId,
}

#[derive(Debug, Clone)]
pub struct DeactivateStationCommand {
    pub station_id: StationId,
    pub deactivated_by: UserId,
}

#[derive(Debug, Clone)]
pub struct PutStationInMaintenanceCommand {
    pub station_id: StationId,
    pub updated_by: UserId,
}
