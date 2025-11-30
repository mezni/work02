use crate::domain::organization::Organization;
use crate::errors::AppError;

#[derive(Clone)]
pub struct OrganizationService;

impl OrganizationService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_organization(
        &self,
        name: String,
        address: Option<String>,
    ) -> Result<Organization, AppError> {
        Ok(Organization::new(name, address))
    }
}
