// configurator-service/src/application/services.rs
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::{
    organization::{Organization, OrganizationService as DomainOrganizationService},
    repositories::{
        OrganizationRepository, OrganizationRepositoryExt, RepositoryError, StationRepository,
        UnitOfWork, UserRepository, UserRepositoryExt,
    },
    station::Station,
    types::{
        OrganizationId, OrganizationStatus, StationId, StationStatus, UserId, UserRole, UserStatus,
    },
    user::User,
};

use super::{commands::*, dtos::*, queries::*};

// Application Error Type
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain validation error: {0}")]
    DomainValidation(String),

    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("User not found: {0}")]
    UserNotFound(UserId),

    #[error("Organization not found: {0}")]
    OrganizationNotFound(OrganizationId),

    #[error("Station not found: {0}")]
    StationNotFound(StationId),

    #[error("Email already exists: {0}")]
    EmailAlreadyExists(String),

    #[error("Organization name already exists: {0}")]
    OrganizationNameAlreadyExists(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Concurrent modification detected")]
    ConcurrentModification,
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;

// User Application Service
#[derive(Debug)]
pub struct UserApplicationService<U: UserRepository> {
    user_repository: U,
}

impl<U: UserRepository> UserApplicationService<U> {
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }

    // Command handlers
    pub async fn create_user(&self, command: CreateUserCommand) -> ApplicationResult<UserDto> {
        // Check if email already exists
        if self
            .user_repository
            .email_exists(&command.email)
            .await
            .map_err(ApplicationError::Repository)?
        {
            return Err(ApplicationError::EmailAlreadyExists(command.email));
        }

        // Create user domain entity
        let user = User::new(
            command.email,
            command.display_name,
            command.role,
            command.organization_id,
            command.station_id,
            command.created_by,
        )
        .map_err(ApplicationError::DomainValidation)?;

        // Save user
        self.user_repository
            .save(user.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        // Convert to DTO
        Ok(self.user_to_dto(user))
    }

    pub async fn update_user(&self, command: UpdateUserCommand) -> ApplicationResult<UserDto> {
        // Get existing user
        let mut user = self
            .user_repository
            .find_active_by_id(command.user_id)
            .await
            .map_err(|_| ApplicationError::UserNotFound(command.user_id))?;

        // Update fields if provided
        if let Some(email) = command.email {
            // Check if new email is available
            if email != user.email
                && self
                    .user_repository
                    .email_exists(&email)
                    .await
                    .map_err(ApplicationError::Repository)?
            {
                return Err(ApplicationError::EmailAlreadyExists(email));
            }
            user.update_email(email, command.updated_by)
                .map_err(ApplicationError::DomainValidation)?;
        }

        if let Some(display_name) = command.display_name {
            user.update_display_name(display_name, command.updated_by)
                .map_err(ApplicationError::DomainValidation)?;
        }

        // Save updated user
        self.user_repository
            .save(user.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(self.user_to_dto(user))
    }

    pub async fn activate_user(&self, command: ActivateUserCommand) -> ApplicationResult<UserDto> {
        let mut user = self
            .user_repository
            .find_by_id(command.user_id)
            .await
            .map_err(|_| ApplicationError::UserNotFound(command.user_id))?;

        user.activate(command.activated_by)
            .map_err(ApplicationError::DomainValidation)?;

        self.user_repository
            .save(user.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(self.user_to_dto(user))
    }

    pub async fn deactivate_user(
        &self,
        command: DeactivateUserCommand,
    ) -> ApplicationResult<UserDto> {
        let mut user = self
            .user_repository
            .find_by_id(command.user_id)
            .await
            .map_err(|_| ApplicationError::UserNotFound(command.user_id))?;

        user.deactivate(command.deactivated_by)
            .map_err(ApplicationError::DomainValidation)?;

        self.user_repository
            .save(user.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(self.user_to_dto(user))
    }

    pub async fn delete_user(&self, command: DeleteUserCommand) -> ApplicationResult<()> {
        let user_exists = self
            .user_repository
            .find_by_id(command.user_id)
            .await
            .is_ok();
        if !user_exists {
            return Err(ApplicationError::UserNotFound(command.user_id));
        }

        self.user_repository
            .soft_delete(command.user_id, command.deleted_by)
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(())
    }

    // Query handlers
    pub async fn get_user(&self, query: GetUserQuery) -> ApplicationResult<UserDto> {
        let user = self
            .user_repository
            .find_active_by_id(query.user_id)
            .await
            .map_err(|_| ApplicationError::UserNotFound(query.user_id))?;

        Ok(self.user_to_dto(user))
    }

    pub async fn get_user_by_email(
        &self,
        query: GetUserByEmailQuery,
    ) -> ApplicationResult<Option<UserDto>> {
        let user = self
            .user_repository
            .find_active_by_email(&query.email)
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(user.map(|u| self.user_to_dto(u)))
    }

    pub async fn list_users(
        &self,
        query: ListUsersQuery,
    ) -> ApplicationResult<PaginatedResponse<UserDto>> {
        // This is a simplified implementation
        // In a real application, you'd have a more sophisticated query builder
        let all_users = if let Some(org_id) = query.organization_id {
            self.user_repository
                .find_by_organization(org_id)
                .await
                .map_err(ApplicationError::Repository)?
        } else {
            // For simplicity, get all users and filter
            // In production, implement proper pagination at repository level
            self.user_repository
                .find_by_status(UserStatus::Active)
                .await
                .map_err(ApplicationError::Repository)?
        };

        // Apply filters
        let mut filtered_users = all_users
            .into_iter()
            .filter(|user| {
                query.role.map(|role| user.role == role).unwrap_or(true)
                    && query
                        .status
                        .map(|status| user.status == status)
                        .unwrap_or(true)
            })
            .collect::<Vec<_>>();

        // Apply pagination
        let total = filtered_users.len() as u64;
        let start = ((query.page - 1) * query.per_page) as usize;
        let end = std::cmp::min(start + query.per_page as usize, filtered_users.len());

        let paginated_users = if start < filtered_users.len() {
            filtered_users.drain(start..end).collect()
        } else {
            Vec::new()
        };

        let user_dtos = paginated_users
            .into_iter()
            .map(|user| self.user_to_dto(user))
            .collect();

        Ok(PaginatedResponse {
            data: user_dtos,
            page: query.page,
            per_page: query.per_page,
            total,
            total_pages: (total as f64 / query.per_page as f64).ceil() as u64,
        })
    }

    // Helper methods
    fn user_to_dto(&self, user: User) -> UserDto {
        UserDto {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            role: user.role,
            organization_id: user.organization_id,
            station_id: user.station_id,
            status: user.status,
            created_at: user.audit.created_at,
            updated_at: user.audit.updated_at,
        }
    }
}

// Organization Application Service
#[derive(Debug)]
pub struct OrganizationApplicationService<O: OrganizationRepository> {
    organization_repository: O,
}

impl<O: OrganizationRepository> OrganizationApplicationService<O> {
    pub fn new(organization_repository: O) -> Self {
        Self {
            organization_repository,
        }
    }

    // Command handlers
    pub async fn create_organization(
        &self,
        command: CreateOrganizationCommand,
    ) -> ApplicationResult<OrganizationDto> {
        // Check if organization name is available
        if !self
            .organization_repository
            .is_name_available(&command.name)
            .await
            .map_err(ApplicationError::Repository)?
        {
            return Err(ApplicationError::OrganizationNameAlreadyExists(
                command.name,
            ));
        }

        // Create organization domain entity
        let organization = Organization::new(command.name, command.created_by)
            .map_err(ApplicationError::DomainValidation)?;

        // Save organization
        self.organization_repository
            .save(organization.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(self.organization_to_dto(organization))
    }

    pub async fn update_organization(
        &self,
        command: UpdateOrganizationCommand,
    ) -> ApplicationResult<OrganizationDto> {
        let mut organization = self
            .organization_repository
            .find_active_by_id(command.organization_id)
            .await
            .map_err(|_| ApplicationError::OrganizationNotFound(command.organization_id))?;

        if let Some(name) = command.name {
            // Check if new name is available
            if name != organization.name
                && !self
                    .organization_repository
                    .is_name_available(&name)
                    .await
                    .map_err(ApplicationError::Repository)?
            {
                return Err(ApplicationError::OrganizationNameAlreadyExists(name));
            }
            organization
                .update_name(name, command.updated_by)
                .map_err(ApplicationError::DomainValidation)?;
        }

        self.organization_repository
            .save(organization.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(self.organization_to_dto(organization))
    }

    pub async fn activate_organization(
        &self,
        command: ActivateOrganizationCommand,
    ) -> ApplicationResult<OrganizationDto> {
        let mut organization = self
            .organization_repository
            .find_by_id(command.organization_id)
            .await
            .map_err(|_| ApplicationError::OrganizationNotFound(command.organization_id))?;

        organization
            .activate(command.activated_by)
            .map_err(ApplicationError::DomainValidation)?;

        self.organization_repository
            .save(organization.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(self.organization_to_dto(organization))
    }

    pub async fn deactivate_organization(
        &self,
        command: DeactivateOrganizationCommand,
    ) -> ApplicationResult<OrganizationDto> {
        let mut organization = self
            .organization_repository
            .find_by_id(command.organization_id)
            .await
            .map_err(|_| ApplicationError::OrganizationNotFound(command.organization_id))?;

        organization
            .deactivate(command.deactivated_by)
            .map_err(ApplicationError::DomainValidation)?;

        self.organization_repository
            .save(organization.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(self.organization_to_dto(organization))
    }

    // Query handlers
    pub async fn get_organization(
        &self,
        query: GetOrganizationQuery,
    ) -> ApplicationResult<OrganizationDto> {
        let organization = self
            .organization_repository
            .find_active_by_id(query.organization_id)
            .await
            .map_err(|_| ApplicationError::OrganizationNotFound(query.organization_id))?;

        Ok(self.organization_to_dto(organization))
    }

    pub async fn list_organizations(
        &self,
        query: ListOrganizationsQuery,
    ) -> ApplicationResult<PaginatedResponse<OrganizationDto>> {
        let (organizations, total) = if let Some(status) = query.status {
            let orgs = self
                .organization_repository
                .find_by_status(status)
                .await
                .map_err(ApplicationError::Repository)?;
            (orgs, orgs.len() as u64)
        } else {
            self.organization_repository
                .list_all(query.page, query.per_page)
                .await
                .map_err(ApplicationError::Repository)?
        };

        let organization_dtos = organizations
            .into_iter()
            .map(|org| self.organization_to_dto(org))
            .collect();

        Ok(PaginatedResponse {
            data: organization_dtos,
            page: query.page,
            per_page: query.per_page,
            total,
            total_pages: (total as f64 / query.per_page as f64).ceil() as u64,
        })
    }

    pub async fn get_organization_summary(
        &self,
        query: GetOrganizationQuery,
    ) -> ApplicationResult<OrganizationSummaryDto> {
        let summary = self
            .organization_repository
            .get_summary(query.organization_id)
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(OrganizationSummaryDto {
            id: summary.id,
            name: summary.name,
            status: summary.status,
            total_users: summary.total_users,
            total_stations: summary.total_stations,
            created_at: summary.created_at,
        })
    }

    pub async fn get_organization_statistics(
        &self,
        query: GetOrganizationStatisticsQuery,
    ) -> ApplicationResult<OrganizationStatisticsDto> {
        let stats = self
            .organization_repository
            .get_statistics(query.organization_id)
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(OrganizationStatisticsDto {
            total_users: stats.total_users,
            active_users: stats.active_users,
            total_stations: stats.total_stations,
            active_stations: stats.active_stations,
            total_charging_sessions: stats.total_charging_sessions,
            total_revenue: stats.total_revenue,
        })
    }

    // Helper methods
    fn organization_to_dto(&self, organization: Organization) -> OrganizationDto {
        OrganizationDto {
            id: organization.id,
            name: organization.name,
            status: organization.status,
            created_at: organization.audit.created_at,
            updated_at: organization.audit.updated_at,
        }
    }
}

// Station Application Service
#[derive(Debug)]
pub struct StationApplicationService<S: StationRepository> {
    station_repository: S,
}

impl<S: StationRepository> StationApplicationService<S> {
    pub fn new(station_repository: S) -> Self {
        Self { station_repository }
    }

    // Command handlers
    pub async fn create_station(
        &self,
        command: CreateStationCommand,
    ) -> ApplicationResult<StationDto> {
        let station = Station::new(command.name, command.organization_id, command.created_by)
            .map_err(ApplicationError::DomainValidation)?;

        // Set location if provided
        let station = if let Some(location) = command.location {
            // In a real implementation, you'd have a method to update location
            // For now, we'll create a new station with location
            Station {
                location: Some(location),
                ..station
            }
        } else {
            station
        };

        self.station_repository
            .save(station.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        // Note: organization_name would need to be fetched separately
        Ok(StationDto {
            id: station.id,
            name: station.name,
            location: station.location,
            organization_id: station.organization_id,
            organization_name: "".to_string(), // Would be populated in a real implementation
            status: station.status,
            created_at: station.audit.created_at,
            updated_at: station.audit.updated_at,
        })
    }

    pub async fn update_station(
        &self,
        command: UpdateStationCommand,
    ) -> ApplicationResult<StationDto> {
        let mut station = self
            .station_repository
            .find_by_id(command.station_id)
            .await
            .map_err(|_| ApplicationError::StationNotFound(command.station_id))?;

        // In a real implementation, you'd have proper update methods
        // For now, we'll create a new station with updated fields
        let updated_station = Station {
            name: command.name.unwrap_or(station.name),
            location: command.location.or(station.location),
            audit: {
                let mut audit = station.audit;
                audit.update(command.updated_by);
                audit
            },
            ..station
        };

        self.station_repository
            .save(updated_station.clone())
            .await
            .map_err(ApplicationError::Repository)?;

        // Note: organization_name would need to be fetched separately
        Ok(StationDto {
            id: updated_station.id,
            name: updated_station.name,
            location: updated_station.location,
            organization_id: updated_station.organization_id,
            organization_name: "".to_string(),
            status: updated_station.status,
            created_at: updated_station.audit.created_at,
            updated_at: updated_station.audit.updated_at,
        })
    }

    // Query handlers
    pub async fn get_station(&self, query: GetStationQuery) -> ApplicationResult<StationDto> {
        let station = self
            .station_repository
            .find_by_id(query.station_id)
            .await
            .map_err(|_| ApplicationError::StationNotFound(query.station_id))?;

        // Note: organization_name would need to be fetched separately
        Ok(StationDto {
            id: station.id,
            name: station.name,
            location: station.location,
            organization_id: station.organization_id,
            organization_name: "".to_string(),
            status: station.status,
            created_at: station.audit.created_at,
            updated_at: station.audit.updated_at,
        })
    }

    pub async fn list_stations(
        &self,
        query: ListStationsQuery,
    ) -> ApplicationResult<PaginatedResponse<StationDto>> {
        let stations = if let Some(org_id) = query.organization_id {
            self.station_repository
                .find_by_organization(org_id)
                .await
                .map_err(ApplicationError::Repository)?
        } else {
            // Simplified implementation - in production, implement proper repository method
            Vec::new()
        };

        // Apply status filter
        let filtered_stations = stations
            .into_iter()
            .filter(|station| {
                query
                    .status
                    .map(|status| station.status == status)
                    .unwrap_or(true)
            })
            .collect::<Vec<_>>();

        // Apply pagination
        let total = filtered_stations.len() as u64;
        let start = ((query.page - 1) * query.per_page) as usize;
        let end = std::cmp::min(start + query.per_page as usize, filtered_stations.len());

        let paginated_stations = if start < filtered_stations.len() {
            filtered_stations.drain(start..end).collect()
        } else {
            Vec::new()
        };

        let station_dtos = paginated_stations
            .into_iter()
            .map(|station| StationDto {
                id: station.id,
                name: station.name,
                location: station.location,
                organization_id: station.organization_id,
                organization_name: "".to_string(), // Would be populated in real implementation
                status: station.status,
                created_at: station.audit.created_at,
                updated_at: station.audit.updated_at,
            })
            .collect();

        Ok(PaginatedResponse {
            data: station_dtos,
            page: query.page,
            per_page: query.per_page,
            total,
            total_pages: (total as f64 / query.per_page as f64).ceil() as u64,
        })
    }
}

// Composite Application Service that coordinates multiple repositories
#[derive(Debug)]
pub struct CompositeApplicationService<
    U: UserRepository,
    O: OrganizationRepository,
    S: StationRepository,
> {
    user_service: UserApplicationService<U>,
    organization_service: OrganizationApplicationService<O>,
    station_service: StationApplicationService<S>,
}

impl<U: UserRepository, O: OrganizationRepository, S: StationRepository>
    CompositeApplicationService<U, O, S>
{
    pub fn new(user_repository: U, organization_repository: O, station_repository: S) -> Self {
        Self {
            user_service: UserApplicationService::new(user_repository),
            organization_service: OrganizationApplicationService::new(organization_repository),
            station_service: StationApplicationService::new(station_repository),
        }
    }

    // Delegate methods to individual services
    pub fn users(&self) -> &UserApplicationService<U> {
        &self.user_service
    }

    pub fn organizations(&self) -> &OrganizationApplicationService<O> {
        &self.organization_service
    }

    pub fn stations(&self) -> &StationApplicationService<S> {
        &self.station_service
    }
}

// Unit tests for application services
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::tests::MockUserRepository;

    #[tokio::test]
    async fn test_user_application_service_creation() {
        let mock_repo = MockUserRepository;
        let service = UserApplicationService::new(mock_repo);

        // Test that service can be created
        assert!(true); // Basic compilation test
    }

    // More tests would be added here with proper mock implementations
}
