// configurator-service/src/application/dtos.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::types::{
    OrganizationId, OrganizationStatus, StationId, StationStatus, UserId, UserRole, UserStatus,
};

// User DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: UserId,
    pub email: String,
    pub display_name: String,
    pub role: UserRole,
    pub organization_id: Option<OrganizationId>,
    pub station_id: Option<StationId>,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub display_name: String,
    pub role: UserRole,
    pub organization_id: Option<OrganizationId>,
    pub station_id: Option<StationId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummaryDto {
    pub id: UserId,
    pub email: String,
    pub display_name: String,
    pub role: UserRole,
    pub organization_name: Option<String>,
    pub station_name: Option<String>,
    pub status: UserStatus,
}

// Organization DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationDto {
    pub id: OrganizationId,
    pub name: String,
    pub status: OrganizationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganizationDto {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrganizationDto {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSummaryDto {
    pub id: OrganizationId,
    pub name: String,
    pub status: OrganizationStatus,
    pub total_users: u32,
    pub total_stations: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStatisticsDto {
    pub total_users: u32,
    pub active_users: u32,
    pub total_stations: u32,
    pub active_stations: u32,
    pub total_charging_sessions: u64,
    pub total_revenue: f64,
}

// Station DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationDto {
    pub id: StationId,
    pub name: String,
    pub location: Option<String>,
    pub organization_id: OrganizationId,
    pub organization_name: String,
    pub status: StationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStationDto {
    pub name: String,
    pub location: Option<String>,
    pub organization_id: OrganizationId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStationDto {
    pub name: Option<String>,
    pub location: Option<String>,
}

// Pagination DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl Default for PaginationRequest {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
        }
    }
}

impl PaginationRequest {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn per_page(&self) -> u32 {
        self.per_page.unwrap_or(20)
    }

    pub fn offset(&self) -> u32 {
        (self.page() - 1) * self.per_page()
    }
}

// Conversion traits
pub trait IntoDto {
    type Dto;
    fn into_dto(self) -> Self::Dto;
}

pub trait FromDto {
    type Dto;
    fn from_dto(dto: Self::Dto) -> Self;
}
