// configurator-service/src/application/queries.rs
use uuid::Uuid;

use crate::domain::types::{
    OrganizationId, OrganizationStatus, StationId, StationStatus, UserId, UserRole, UserStatus,
};

// User Queries
#[derive(Debug, Clone)]
pub struct GetUserQuery {
    pub user_id: UserId,
}

#[derive(Debug, Clone)]
pub struct GetUserByEmailQuery {
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct ListUsersQuery {
    pub organization_id: Option<OrganizationId>,
    pub role: Option<UserRole>,
    pub status: Option<UserStatus>,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone)]
pub struct ListOrganizationUsersQuery {
    pub organization_id: OrganizationId,
    pub role: Option<UserRole>,
    pub status: Option<UserStatus>,
    pub page: u32,
    pub per_page: u32,
}

// Organization Queries
#[derive(Debug, Clone)]
pub struct GetOrganizationQuery {
    pub organization_id: OrganizationId,
}

#[derive(Debug, Clone)]
pub struct ListOrganizationsQuery {
    pub status: Option<OrganizationStatus>,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone)]
pub struct GetOrganizationStatisticsQuery {
    pub organization_id: OrganizationId,
}

// Station Queries
#[derive(Debug, Clone)]
pub struct GetStationQuery {
    pub station_id: StationId,
}

#[derive(Debug, Clone)]
pub struct ListStationsQuery {
    pub organization_id: Option<OrganizationId>,
    pub status: Option<StationStatus>,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone)]
pub struct ListOrganizationStationsQuery {
    pub organization_id: OrganizationId,
    pub status: Option<StationStatus>,
    pub page: u32,
    pub per_page: u32,
}

// Default implementations
impl Default for ListUsersQuery {
    fn default() -> Self {
        Self {
            organization_id: None,
            role: None,
            status: None,
            page: 1,
            per_page: 20,
        }
    }
}

impl Default for ListOrganizationsQuery {
    fn default() -> Self {
        Self {
            status: None,
            page: 1,
            per_page: 20,
        }
    }
}

impl Default for ListStationsQuery {
    fn default() -> Self {
        Self {
            organization_id: None,
            status: None,
            page: 1,
            per_page: 20,
        }
    }
}
