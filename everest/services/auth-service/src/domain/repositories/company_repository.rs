use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Company;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn create(&self, company: &Company) -> Result<(), DomainError>;

    async fn update(&self, company: &Company) -> Result<(), DomainError>;

    async fn delete(&self, company_id: &Uuid) -> Result<(), DomainError>;

    async fn find_by_id(&self, company_id: &Uuid) -> Result<Option<Company>, DomainError>;

    async fn find_by_name(&self, name: &str) -> Result<Option<Company>, DomainError>; // Add this method

    async fn find_all(&self, page: u32, page_size: u32) -> Result<Vec<Company>, DomainError>;

    async fn find_by_creator(&self, user_id: &Uuid) -> Result<Vec<Company>, DomainError>;

    async fn exists_by_name(&self, name: &str) -> Result<bool, DomainError>;
}
