use actix_web::{HttpResponse, web};
use tracing::{info, error};
use uuid::Uuid;
use crate::application::commands::{
    CreateCompanyCommand, UpdateCompanyCommand, DeleteCompanyCommand
};
use crate::application::dto::{
    CreateCompanyRequest, UpdateCompanyRequest, CompanyDto, CompanyListResponse, CompanyUsersResponse
};
use crate::application::handlers::command_handlers::CompanyCommandHandler;
use crate::application::handlers::query_handlers::{CompanyQueryHandler, UserQueryHandler};
use crate::common::response::ApiResponse;
use crate::domain::repositories::{CompanyRepository, UserRepository};
use crate::domain::enums::UserRole;

pub struct CompanyController<T: CompanyRepository, U: UserRepository> {
    company_handler: CompanyCommandHandler<T, U>,
    company_query_handler: CompanyQueryHandler<T, U>,
    user_query_handler: UserQueryHandler<U>,
}

impl<T: CompanyRepository, U: UserRepository> CompanyController<T, U> {
    pub fn new(
        company_handler: CompanyCommandHandler<T, U>,
        company_query_handler: CompanyQueryHandler<T, U>,
        user_query_handler: UserQueryHandler<U>,
    ) -> Self {
        Self {
            company_handler,
            company_query_handler,
            user_query_handler,
        }
    }

    pub async fn create_company(
        &self,
        request: CreateCompanyRequest,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Creating new company: {}", request.name);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        // Get requester user
        let requester_query = crate::application::queries::GetUserByIdQuery {
            user_id: requester_uuid,
            requested_by: requester_uuid,
        };
        let requester = self.user_query_handler.handle_get_user_by_id(requester_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(requester_id))?;

        // Check if user can create companies (only admins)
        if !requester.role.can_manage_companies() {
            return Err(crate::application::errors::ApplicationError::Authorization(
                "Insufficient permissions to create companies".to_string(),
            ));
        }

        let command = CreateCompanyCommand {
            name: request.name,
            description: request.description,
            created_by: requester.id,
        };

        let company = self.company_handler.handle_create_company(command, &requester).await?;
        let company_dto = CompanyDto::from(company);

        let response = ApiResponse::success(company_dto, "Company created successfully");
        Ok(HttpResponse::Created().json(response))
    }

    pub async fn get_company(
        &self,
        company_id: Uuid,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Getting company: {}", company_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let query = crate::application::queries::GetCompanyByIdQuery {
            company_id,
            requested_by: requester_uuid,
        };

        if let Some(company) = self.company_query_handler.handle_get_company_by_id(query).await? {
            let company_dto = CompanyDto::from(company);
            let response = ApiResponse::success(company_dto, "Company retrieved successfully");
            Ok(HttpResponse::Ok().json(response))
        } else {
            Err(crate::application::errors::ApplicationError::CompanyNotFound(company_id.to_string()))
        }
    }

    pub async fn list_companies(
        &self,
        query: crate::interfaces::routes::company_routes::ListCompaniesQuery,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Listing companies");

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(20);

        let list_query = crate::application::queries::ListCompaniesQuery {
            page,
            page_size,
            requested_by: requester_uuid,
        };

        let companies = self.company_query_handler.handle_list_companies(list_query).await?;
        let company_dtos: Vec<CompanyDto> = companies.into_iter().map(CompanyDto::from).collect();

        let response = CompanyListResponse {
            companies: company_dtos,
            total: companies.len() as u64,
            page,
            page_size,
            total_pages: (companies.len() as f64 / page_size as f64).ceil() as u32,
        };

        let api_response = ApiResponse::success(response, "Companies retrieved successfully");
        Ok(HttpResponse::Ok().json(api_response))
    }

    pub async fn update_company(
        &self,
        company_id: Uuid,
        request: UpdateCompanyRequest,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Updating company: {}", company_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        // Get requester user
        let requester_query = crate::application::queries::GetUserByIdQuery {
            user_id: requester_uuid,
            requested_by: requester_uuid,
        };
        let requester = self.user_query_handler.handle_get_user_by_id(requester_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(requester_id))?;

        // Check if user can update companies (only admins)
        if !requester.role.can_manage_companies() {
            return Err(crate::application::errors::ApplicationError::Authorization(
                "Insufficient permissions to update companies".to_string(),
            ));
        }

        let command = UpdateCompanyCommand {
            name: request.name,
            description: request.description,
        };

        let company = self.company_handler.handle_update_company(company_id, command, &requester).await?;
        let company_dto = CompanyDto::from(company);

        let response = ApiResponse::success(company_dto, "Company updated successfully");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn delete_company(
        &self,
        company_id: Uuid,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Deleting company: {}", company_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        // Get requester user
        let requester_query = crate::application::queries::GetUserByIdQuery {
            user_id: requester_uuid,
            requested_by: requester_uuid,
        };
        let requester = self.user_query_handler.handle_get_user_by_id(requester_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(requester_id))?;

        // Check if user can delete companies (only admins)
        if !requester.role.can_manage_companies() {
            return Err(crate::application::errors::ApplicationError::Authorization(
                "Insufficient permissions to delete companies".to_string(),
            ));
        }

        let command = DeleteCompanyCommand {
            company_id,
            deleted_by: requester.id,
        };

        self.company_handler.handle_delete_company(command, &requester).await?;

        let response = ApiResponse::success((), "Company deleted successfully");
        Ok(HttpResponse::NoContent().json(response))
    }

    pub async fn get_company_users(
        &self,
        company_id: Uuid,
        query: crate::interfaces::routes::company_routes::CompanyUsersQuery,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Getting users for company: {}", company_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(20);

        let users_query = crate::application::queries::GetCompanyUsersQuery {
            company_id,
            page,
            page_size,
            requested_by: requester_uuid,
        };

        let users = self.company_query_handler.handle_get_company_users(users_query).await?;
        let user_dtos: Vec<crate::application::dto::UserDto> = users.into_iter().map(crate::application::dto::UserDto::from).collect();

        let response = CompanyUsersResponse {
            users: user_dtos,
            total: users.len() as u64,
            page,
            page_size,
            total_pages: (users.len() as f64 / page_size as f64).ceil() as u32,
        };

        let api_response = ApiResponse::success(response, "Company users retrieved successfully");
        Ok(HttpResponse::Ok().json(api_response))
    }
}