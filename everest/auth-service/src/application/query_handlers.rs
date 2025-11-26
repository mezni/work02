use async_trait::async_trait;
use uuid::Uuid;

use crate::application::dto::{CompanyDto, UserDto};
use crate::application::errors::ApplicationError;
use crate::application::queries::*;
use crate::domain::repositories::{CompanyRepository, UserRepository};

#[async_trait]
pub trait QueryHandler<Q, R> {
    async fn handle(&self, query: Q) -> Result<R, ApplicationError>;
}

pub struct GetUserByIdQueryHandler {
    user_repository: Box<dyn UserRepository>,
}

impl GetUserByIdQueryHandler {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl QueryHandler<GetUserByIdQuery, Option<UserDto>> for GetUserByIdQueryHandler {
    async fn handle(&self, query: GetUserByIdQuery) -> Result<Option<UserDto>, ApplicationError> {
        let user = self.user_repository.find_by_id(query.user_id).await?;

        Ok(user.map(|u| UserDto {
            id: u.id,
            keycloak_id: u.keycloak_id,
            username: u.username,
            email: u.email,
            role: u.role,
            company_id: u.company_id,
            email_verified: u.email_verified,
            created_at: u.created_at.to_rfc3339(),
            updated_at: u.updated_at.to_rfc3339(),
        }))
    }
}

pub struct ListUsersQueryHandler {
    user_repository: Box<dyn UserRepository>,
}

impl ListUsersQueryHandler {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl QueryHandler<ListUsersQuery, Vec<UserDto>> for ListUsersQueryHandler {
    async fn handle(&self, query: ListUsersQuery) -> Result<Vec<UserDto>, ApplicationError> {
        let users = if let Some(company_id) = query.company_id {
            self.user_repository.list_by_company(company_id).await?
        } else {
            self.user_repository.list_all().await?
        };

        let users_dto = users
            .into_iter()
            .map(|u| UserDto {
                id: u.id,
                keycloak_id: u.keycloak_id,
                username: u.username,
                email: u.email,
                role: u.role,
                company_id: u.company_id,
                email_verified: u.email_verified,
                created_at: u.created_at.to_rfc3339(),
                updated_at: u.updated_at.to_rfc3339(),
            })
            .collect();

        Ok(users_dto)
    }
}

pub struct GetCompanyByIdQueryHandler {
    company_repository: Box<dyn CompanyRepository>,
}

impl GetCompanyByIdQueryHandler {
    pub fn new(company_repository: Box<dyn CompanyRepository>) -> Self {
        Self { company_repository }
    }
}

#[async_trait]
impl QueryHandler<GetCompanyByIdQuery, Option<CompanyDto>> for GetCompanyByIdQueryHandler {
    async fn handle(
        &self,
        query: GetCompanyByIdQuery,
    ) -> Result<Option<CompanyDto>, ApplicationError> {
        let company = self.company_repository.find_by_id(query.company_id).await?;

        Ok(company.map(|c| CompanyDto {
            id: c.id,
            name: c.name,
            description: c.description,
            created_by: c.created_by,
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.to_rfc3339(),
        }))
    }
}

// Additional query handlers would be implemented similarly
