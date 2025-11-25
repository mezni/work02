use tracing::{info, warn};
use uuid::Uuid;

use crate::application::queries::{
    GetUserByIdQuery, GetUserByEmailQuery, ListUsersQuery, GetUserProfileQuery, SearchUsersQuery,
    GetCompanyByIdQuery, ListCompaniesQuery, GetCompanyUsersQuery, SearchCompaniesQuery,
    GetAuditLogsQuery, GetUserAuditLogsQuery, GetCompanyAuditLogsQuery,
};
use crate::application::errors::ApplicationError;
use crate::domain::entities::User;
use crate::domain::value_objects::Email;
use crate::domain::repositories::{UserRepository, CompanyRepository, AuditRepository};

pub struct UserQueryHandler<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> UserQueryHandler<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn handle_get_user_by_id(
        &self,
        query: GetUserByIdQuery,
    ) -> Result<Option<User>, ApplicationError> {
        let user = self.user_repository.find_by_id(&query.user_id).await?;
        
        // If user exists, check if requester has permission to view this user
        if let Some(target_user) = &user {
            let requester = self.user_repository.find_by_id(&query.requested_by).await?
                .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;
            
            if !requester.can_manage_user(target_user) && requester.id != target_user.id {
                return Ok(None); // Return None instead of error for security
            }
        }

        Ok(user)
    }

    pub async fn handle_get_user_by_email(
        &self,
        query: GetUserByEmailQuery,
    ) -> Result<Option<User>, ApplicationError> {
        let email = Email::new(query.email)?;
        let user = self.user_repository.find_by_email(&email).await?;
        
        // Permission check similar to get_user_by_id
        if let Some(target_user) = &user {
            let requester = self.user_repository.find_by_id(&query.requested_by).await?
                .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;
            
            if !requester.can_manage_user(target_user) && requester.id != target_user.id {
                return Ok(None);
            }
        }

        Ok(user)
    }

    pub async fn handle_list_users(
        &self,
        query: ListUsersQuery,
    ) -> Result<Vec<User>, ApplicationError> {
        let requester = self.user_repository.find_by_id(&query.requested_by).await?
            .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;

        let users = if requester.is_admin() {
            // Admin can see all users
            if let Some(company_id) = query.company_id {
                self.user_repository.find_by_company(&company_id).await?
            } else {
                // For simplicity, return empty for now - in real implementation,
                // you'd have a method to get all users with pagination
                Vec::new()
            }
        } else if requester.is_partner() || requester.is_operator() {
            // Partner/Operator can only see users in their company
            if let Some(company_id) = requester.company_id {
                self.user_repository.find_by_company(&company_id).await?
            } else {
                Vec::new()
            }
        } else {
            // Regular users can only see themselves
            vec![requester]
        };

        // Filter by role if specified
        let filtered_users = if let Some(role_filter) = query.role {
            users.into_iter()
                .filter(|user| user.role.to_string() == role_filter)
                .collect()
        } else {
            users
        };

        // Apply pagination (simplified - in real implementation, do this at DB level)
        let start_index = ((query.page - 1) * query.page_size) as usize;
        let end_index = std::cmp::min(start_index + query.page_size as usize, filtered_users.len());
        
        let paginated_users = if start_index < filtered_users.len() {
            filtered_users[start_index..end_index].to_vec()
        } else {
            Vec::new()
        };

        Ok(paginated_users)
    }

    pub async fn handle_get_user_profile(
        &self,
        query: GetUserProfileQuery,
    ) -> Result<User, ApplicationError> {
        let user = self.user_repository.find_by_id(&query.user_id).await?
            .ok_or_else(|| ApplicationError::UserNotFound(query.user_id.to_string()))?;

        Ok(user)
    }

    pub async fn handle_search_users(
        &self,
        query: SearchUsersQuery,
    ) -> Result<Vec<User>, ApplicationError> {
        let requester = self.user_repository.find_by_id(&query.requested_by).await?
            .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;

        // In a real implementation, you'd have a search method in the repository
        // For now, we'll simulate search by getting all users and filtering
        let all_users = if requester.is_admin() {
            // Simulate getting all users - in real impl, use proper search
            Vec::new()
        } else if let Some(company_id) = requester.company_id {
            self.user_repository.find_by_company(&company_id).await?
        } else {
            vec![requester.clone()]
        };

        // Simple search simulation
        let search_lower = query.query.to_lowercase();
        let filtered_users = all_users.into_iter()
            .filter(|user| {
                user.username.to_lowercase().contains(&search_lower) ||
                user.email.value().to_lowercase().contains(&search_lower)
            })
            .collect();

        Ok(filtered_users)
    }
}

pub struct CompanyQueryHandler<T: CompanyRepository, U: UserRepository> {
    company_repository: T,
    user_repository: U,
}

impl<T: CompanyRepository, U: UserRepository> CompanyQueryHandler<T, U> {
    pub fn new(company_repository: T, user_repository: U) -> Self {
        Self { company_repository, user_repository }
    }

    pub async fn handle_get_company_by_id(
        &self,
        query: GetCompanyByIdQuery,
    ) -> Result<Option<crate::domain::entities::Company>, ApplicationError> {
        let company = self.company_repository.find_by_id(&query.company_id).await?;
        
        // Check permissions
        if let Some(company) = &company {
            let requester = self.user_repository.find_by_id(&query.requested_by).await?
                .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;
            
            if !requester.has_company_access(&company.id) {
                return Ok(None);
            }
        }

        Ok(company)
    }

    pub async fn handle_list_companies(
        &self,
        query: ListCompaniesQuery,
    ) -> Result<Vec<crate::domain::entities::Company>, ApplicationError> {
        let requester = self.user_repository.find_by_id(&query.requested_by).await?
            .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;

        let companies = if requester.is_admin() {
            // Admin can see all companies
            self.company_repository.find_all(query.page, query.page_size).await?
        } else if let Some(company_id) = requester.company_id {
            // Non-admin users can only see their own company
            if let Some(company) = self.company_repository.find_by_id(&company_id).await? {
                vec![company]
            } else {
                Vec::new()
            }
        } else {
            // Users without company can't see any companies
            Vec::new()
        };

        Ok(companies)
    }

    pub async fn handle_get_company_users(
        &self,
        query: GetCompanyUsersQuery,
    ) -> Result<Vec<User>, ApplicationError> {
        let requester = self.user_repository.find_by_id(&query.requested_by).await?
            .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;

        // Check if requester has access to this company
        if !requester.has_company_access(&query.company_id) {
            return Ok(Vec::new());
        }

        let users = self.user_repository.find_by_company(&query.company_id).await?;

        // Apply pagination
        let start_index = ((query.page - 1) * query.page_size) as usize;
        let end_index = std::cmp::min(start_index + query.page_size as usize, users.len());
        
        let paginated_users = if start_index < users.len() {
            users[start_index..end_index].to_vec()
        } else {
            Vec::new()
        };

        Ok(paginated_users)
    }

    pub async fn handle_search_companies(
        &self,
        query: SearchCompaniesQuery,
    ) -> Result<Vec<crate::domain::entities::Company>, ApplicationError> {
        let requester = self.user_repository.find_by_id(&query.requested_by).await?
            .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;

        // In real implementation, you'd have a search method
        // For now, simulate by getting all accessible companies and filtering
        let accessible_companies = if requester.is_admin() {
            self.company_repository.find_all(1, 1000).await? // Get first 1000 companies
        } else if let Some(company_id) = requester.company_id {
            if let Some(company) = self.company_repository.find_by_id(&company_id).await? {
                vec![company]
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Simple search simulation
        let search_lower = query.query.to_lowercase();
        let filtered_companies = accessible_companies.into_iter()
            .filter(|company| company.name.to_lowercase().contains(&search_lower))
            .collect();

        Ok(filtered_companies)
    }
}

pub struct AuditQueryHandler<T: AuditRepository, U: UserRepository> {
    audit_repository: T,
    user_repository: U,
}

impl<T: AuditRepository, U: UserRepository> AuditQueryHandler<T, U> {
    pub fn new(audit_repository: T, user_repository: U) -> Self {
        Self { audit_repository, user_repository }
    }

    pub async fn handle_get_audit_logs(
        &self,
        query: GetAuditLogsQuery,
    ) -> Result<Vec<crate::domain::entities::AuditLog>, ApplicationError> {
        let requester = self.user_repository.find_by_id(&query.requested_by).await?
            .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;

        // Only admin can view all audit logs
        if !requester.is_admin() {
            return Err(ApplicationError::Authorization(
                "Only administrators can view audit logs".to_string(),
            ));
        }

        let logs = self.audit_repository.search(
            query.user_id.as_deref(),
            query.company_id.as_ref(),
            query.action,
            query.start_date,
            query.end_date,
            query.page,
            query.page_size,
        ).await?;

        Ok(logs)
    }

    pub async fn handle_get_user_audit_logs(
        &self,
        query: GetUserAuditLogsQuery,
    ) -> Result<Vec<crate::domain::entities::AuditLog>, ApplicationError> {
        let requester = self.user_repository.find_by_id(&query.requested_by).await?
            .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;

        // Users can only view their own audit logs unless they're admin
        if !requester.is_admin() && requester.id.to_string() != query.user_id {
            return Err(ApplicationError::Authorization(
                "Can only view your own audit logs".to_string(),
            ));
        }

        let logs = self.audit_repository.find_by_user_id(
            &query.user_id,
            query.page,
            query.page_size,
        ).await?;

        Ok(logs)
    }

    pub async fn handle_get_company_audit_logs(
        &self,
        query: GetCompanyAuditLogsQuery,
    ) -> Result<Vec<crate::domain::entities::AuditLog>, ApplicationError> {
        let requester = self.user_repository.find_by_id(&query.requested_by).await?
            .ok_or_else(|| ApplicationError::UserNotFound(query.requested_by.to_string()))?;

        // Check if requester has access to this company
        if !requester.has_company_access(&query.company_id) {
            return Err(ApplicationError::Authorization(
                "No access to this company's audit logs".to_string(),
            ));
        }

        let logs = self.audit_repository.find_by_company_id(
            &query.company_id,
            query.page,
            query.page_size,
        ).await?;

        Ok(logs)
    }
}