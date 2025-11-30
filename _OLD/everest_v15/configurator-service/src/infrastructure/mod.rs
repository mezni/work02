// configurator-service/src/infrastructure/mod.rs
pub mod repositories;
//pub mod auth;

// Re-export for easy access
pub use repositories::{
    OrganizationRepositoryImpl, RepositoryFactory, StationRepositoryImpl, UnitOfWorkImpl,
    UserRepositoryImpl,
};

impl RepositoryFactory {
    // ... existing methods ...

    pub fn organization_service(
        &self,
    ) -> crate::application::OrganizationApplicationService<OrganizationRepositoryImpl> {
        crate::application::OrganizationApplicationService::new(self.organization_repository())
    }

    pub fn user_service(&self) -> crate::application::UserApplicationService<UserRepositoryImpl> {
        crate::application::UserApplicationService::new(self.user_repository())
    }

    pub fn station_service(
        &self,
    ) -> crate::application::StationApplicationService<StationRepositoryImpl> {
        crate::application::StationApplicationService::new(self.station_repository())
    }

    pub fn composite_service(
        &self,
    ) -> crate::application::CompositeApplicationService<
        UserRepositoryImpl,
        OrganizationRepositoryImpl,
        StationRepositoryImpl,
    > {
        crate::application::CompositeApplicationService::new(
            self.user_repository(),
            self.organization_repository(),
            self.station_repository(),
        )
    }
}
