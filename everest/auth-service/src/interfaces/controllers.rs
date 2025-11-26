use actix_web::{web, HttpRequest, HttpResponse};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::application::dto::{
    CompanyDto, CreateCompanyDto, CreateUserDto, LoginRequest, LoginResponse, RegisterRequest,
    UpdateCompanyDto, UpdateUserDto, UserDto,
};
use crate::application::services::{AuthService, AuthServiceImpl};
use crate::domain::repositories::{AuditLogRepository, CompanyRepository, UserRepository};
use crate::infrastructure::auth::KeycloakClient;
use crate::interfaces::errors::{InterfaceError, WebResult};

#[derive(Clone)]
pub struct AuthController {
    auth_service: web::Data<Box<dyn AuthService>>,
}

impl AuthController {
    pub fn new(auth_service: web::Data<Box<dyn AuthService>>) -> Self {
        Self { auth_service }
    }

    pub async fn register(
        &self,
        register_request: web::Json<RegisterRequest>,
    ) -> WebResult<HttpResponse> {
        register_request
            .validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;

        let user_dto = self
            .auth_service
            .register(
                register_request.username.clone(),
                register_request.email.clone(),
                register_request.password.clone(),
            )
            .await?;

        Ok(HttpResponse::Created().json(user_dto))
    }

    pub async fn login(&self, login_request: web::Json<LoginRequest>) -> WebResult<HttpResponse> {
        login_request
            .validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;

        let login_response = self
            .auth_service
            .login(
                login_request.username.clone(),
                login_request.password.clone(),
            )
            .await?;

        Ok(HttpResponse::Ok().json(login_response))
    }

    pub async fn refresh_token(
        &self,
        refresh_request: web::Json<serde_json::Value>,
    ) -> WebResult<HttpResponse> {
        let refresh_token = refresh_request
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InterfaceError::BadRequest("Refresh token is required".to_string()))?;

        let login_response = self
            .auth_service
            .refresh_token(refresh_token.to_string())
            .await?;

        Ok(HttpResponse::Ok().json(login_response))
    }

    pub async fn validate_token(
        &self,
        token_request: web::Json<serde_json::Value>,
    ) -> WebResult<HttpResponse> {
        let token = token_request
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InterfaceError::BadRequest("Token is required".to_string()))?;

        let claims = self.auth_service.validate_token(token.to_string()).await?;

        Ok(HttpResponse::Ok().json(claims))
    }

    pub async fn logout(&self, _request: HttpRequest) -> WebResult<HttpResponse> {
        // In a real implementation, you might want to blacklist the token
        // or call Keycloak's logout endpoint
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Logged out successfully"
        })))
    }
}

#[derive(Clone)]
pub struct UserController {
    user_repository: web::Data<Box<dyn UserRepository>>,
    company_repository: web::Data<Box<dyn CompanyRepository>>,
}

impl UserController {
    pub fn new(
        user_repository: web::Data<Box<dyn UserRepository>>,
        company_repository: web::Data<Box<dyn CompanyRepository>>,
    ) -> Self {
        Self {
            user_repository,
            company_repository,
        }
    }

    pub async fn list_users(
        &self,
        request: HttpRequest,
        query: web::Query<ListUsersQuery>,
    ) -> WebResult<HttpResponse> {
        // Extract user from request (from JWT middleware)
        let current_user = Self::extract_current_user(&request)?;

        // Authorization check
        if !current_user.is_admin() && !current_user.is_partner() && !current_user.is_operator() {
            return Err(InterfaceError::InsufficientPermissions);
        }

        let users = if let Some(company_id) = query.company_id {
            // Non-admin users can only list users from their own company
            if !current_user.is_admin() && current_user.company_id != Some(company_id) {
                return Err(InterfaceError::InsufficientPermissions);
            }
            self.user_repository
                .list_by_company(company_id)
                .await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
        } else {
            // Only admin users can list all users without company filter
            if !current_user.is_admin() {
                return Err(InterfaceError::InsufficientPermissions);
            }
            self.user_repository
                .list_all()
                .await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
        };

        let users_dto: Vec<UserDto> = users
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

        Ok(HttpResponse::Ok().json(users_dto))
    }

    pub async fn get_user(
        &self,
        request: HttpRequest,
        user_id: web::Path<Uuid>,
    ) -> WebResult<HttpResponse> {
        let current_user = Self::extract_current_user(&request)?;
        let target_user_id = user_id.into_inner();

        // Users can view their own profile, admins can view any profile
        // Partners/Operators can view users from their company
        if current_user.id != target_user_id && !current_user.is_admin() {
            let target_user = self
                .user_repository
                .find_by_id(target_user_id)
                .await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
                .ok_or(InterfaceError::NotFound)?;

            if !current_user.can_manage_user(&target_user) {
                return Err(InterfaceError::InsufficientPermissions);
            }
        }

        let user = self
            .user_repository
            .find_by_id(target_user_id)
            .await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?
            .ok_or(InterfaceError::NotFound)?;

        let user_dto = UserDto {
            id: user.id,
            keycloak_id: user.keycloak_id,
            username: user.username,
            email: user.email,
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        };

        Ok(HttpResponse::Ok().json(user_dto))
    }

    pub async fn create_user(
        &self,
        request: HttpRequest,
        create_user_dto: web::Json<CreateUserDto>,
    ) -> WebResult<HttpResponse> {
        create_user_dto
            .validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;

        let current_user = Self::extract_current_user(&request)?;

        // Only admins can create users with specific roles
        if !current_user.is_admin() {
            if let Some(role) = &create_user_dto.role {
                if matches!(role, crate::domain::enums::UserRole::Admin) {
                    return Err(InterfaceError::InsufficientPermissions);
                }
            }

            // Partners/Operators can only create users for their company
            if let Some(company_id) = create_user_dto.company_id {
                if !current_user.can_manage_company(company_id) {
                    return Err(InterfaceError::InsufficientPermissions);
                }
            }
        }

        // This is a simplified implementation
        // In a real app, you'd use the AuthService to create users in both Keycloak and local DB
        Ok(HttpResponse::NotImplemented().json(serde_json::json!({
            "message": "User creation through this endpoint is not implemented. Use registration endpoint instead."
        })))
    }

    pub async fn update_user(
        &self,
        request: HttpRequest,
        user_id: web::Path<Uuid>,
        update_user_dto: web::Json<UpdateUserDto>,
    ) -> WebResult<HttpResponse> {
        update_user_dto
            .validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;

        let current_user = Self::extract_current_user(&request)?;
        let target_user_id = user_id.into_inner();

        let mut target_user = self
            .user_repository
            .find_by_id(target_user_id)
            .await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?
            .ok_or(InterfaceError::NotFound)?;

        // Authorization check
        if !current_user.can_manage_user(&target_user) {
            return Err(InterfaceError::InsufficientPermissions);
        }

        // Update user fields
        if let Some(username) = &update_user_dto.username {
            target_user.username = username.clone();
        }

        if let Some(email) = &update_user_dto.email {
            target_user.email = email.clone();
        }

        if let Some(role) = &update_user_dto.role {
            // Only admins can change roles to admin
            if matches!(role, crate::domain::enums::UserRole::Admin) && !current_user.is_admin() {
                return Err(InterfaceError::InsufficientPermissions);
            }
            target_user.role = role.clone();
        }

        if let Some(company_id) = update_user_dto.company_id {
            // Authorization check for company assignment
            if !current_user.can_manage_company(company_id) {
                return Err(InterfaceError::InsufficientPermissions);
            }
            target_user.company_id = Some(company_id);
        }

        let updated_user = self
            .user_repository
            .update(&target_user)
            .await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?;

        let user_dto = UserDto {
            id: updated_user.id,
            keycloak_id: updated_user.keycloak_id,
            username: updated_user.username,
            email: updated_user.email,
            role: updated_user.role,
            company_id: updated_user.company_id,
            email_verified: updated_user.email_verified,
            created_at: updated_user.created_at.to_rfc3339(),
            updated_at: updated_user.updated_at.to_rfc3339(),
        };

        Ok(HttpResponse::Ok().json(user_dto))
    }

    fn extract_current_user(request: &HttpRequest) -> WebResult<crate::domain::entities::User> {
        // This would extract the user from JWT claims added by middleware
        // For now, return a mock admin user for testing
        Ok(crate::domain::entities::User::new(
            "mock-keycloak-id".to_string(),
            "admin".to_string(),
            "admin@example.com".to_string(),
            crate::domain::enums::UserRole::Admin,
            None,
        )
        .map_err(|e| InterfaceError::ApplicationError(e.into()))?)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ListUsersQuery {
    pub company_id: Option<Uuid>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Clone)]
pub struct CompanyController {
    company_repository: web::Data<Box<dyn CompanyRepository>>,
    user_repository: web::Data<Box<dyn UserRepository>>,
}

impl CompanyController {
    pub fn new(
        company_repository: web::Data<Box<dyn CompanyRepository>>,
        user_repository: web::Data<Box<dyn UserRepository>>,
    ) -> Self {
        Self {
            company_repository,
            user_repository,
        }
    }

    pub async fn list_companies(
        &self,
        request: HttpRequest,
        query: web::Query<ListCompaniesQuery>,
    ) -> WebResult<HttpResponse> {
        let current_user = Self::extract_current_user(&request)?;

        let companies = if current_user.is_admin() {
            self.company_repository
                .list_all()
                .await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
        } else {
            self.company_repository
                .list_by_user(current_user.id)
                .await
                .map_err(|e| InterfaceError::ApplicationError(e.into()))?
        };

        let companies_dto: Vec<CompanyDto> = companies
            .into_iter()
            .map(|c| CompanyDto {
                id: c.id,
                name: c.name,
                description: c.description,
                created_by: c.created_by,
                created_at: c.created_at.to_rfc3339(),
                updated_at: c.updated_at.to_rfc3339(),
            })
            .collect();

        Ok(HttpResponse::Ok().json(companies_dto))
    }

    pub async fn get_company(
        &self,
        request: HttpRequest,
        company_id: web::Path<Uuid>,
    ) -> WebResult<HttpResponse> {
        let current_user = Self::extract_current_user(&request)?;
        let target_company_id = company_id.into_inner();

        let company = self
            .company_repository
            .find_by_id(target_company_id)
            .await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?
            .ok_or(InterfaceError::NotFound)?;

        // Authorization check
        if !current_user.can_manage_company(company.id) {
            return Err(InterfaceError::InsufficientPermissions);
        }

        let company_dto = CompanyDto {
            id: company.id,
            name: company.name,
            description: company.description,
            created_by: company.created_by,
            created_at: company.created_at.to_rfc3339(),
            updated_at: company.updated_at.to_rfc3339(),
        };

        Ok(HttpResponse::Ok().json(company_dto))
    }

    pub async fn create_company(
        &self,
        request: HttpRequest,
        create_company_dto: web::Json<CreateCompanyDto>,
    ) -> WebResult<HttpResponse> {
        create_company_dto
            .validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;

        let current_user = Self::extract_current_user(&request)?;

        // Only admin users can create companies
        if !current_user.is_admin() {
            return Err(InterfaceError::InsufficientPermissions);
        }

        let company = crate::domain::entities::Company::new(
            create_company_dto.name.clone(),
            create_company_dto.description.clone(),
            current_user.id,
        );

        let created_company = self
            .company_repository
            .create(&company)
            .await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?;

        let company_dto = CompanyDto {
            id: created_company.id,
            name: created_company.name,
            description: created_company.description,
            created_by: created_company.created_by,
            created_at: created_company.created_at.to_rfc3339(),
            updated_at: created_company.updated_at.to_rfc3339(),
        };

        Ok(HttpResponse::Created().json(company_dto))
    }

    pub async fn update_company(
        &self,
        request: HttpRequest,
        company_id: web::Path<Uuid>,
        update_company_dto: web::Json<UpdateCompanyDto>,
    ) -> WebResult<HttpResponse> {
        update_company_dto
            .validate()
            .map_err(|e| InterfaceError::ValidationError(e.to_string()))?;

        let current_user = Self::extract_current_user(&request)?;
        let target_company_id = company_id.into_inner();

        let mut company = self
            .company_repository
            .find_by_id(target_company_id)
            .await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?
            .ok_or(InterfaceError::NotFound)?;

        // Authorization check
        if !current_user.can_manage_company(company.id) {
            return Err(InterfaceError::InsufficientPermissions);
        }

        // Update company fields
        if let Some(name) = &update_company_dto.name {
            company.name = name.clone();
        }

        if let Some(description) = &update_company_dto.description {
            company.description = Some(description.clone());
        }

        let updated_company = self
            .company_repository
            .update(&company)
            .await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?;

        let company_dto = CompanyDto {
            id: updated_company.id,
            name: updated_company.name,
            description: updated_company.description,
            created_by: updated_company.created_by,
            created_at: updated_company.created_at.to_rfc3339(),
            updated_at: updated_company.updated_at.to_rfc3339(),
        };

        Ok(HttpResponse::Ok().json(company_dto))
    }

    pub async fn list_company_users(
        &self,
        request: HttpRequest,
        company_id: web::Path<Uuid>,
        query: web::Query<ListCompanyUsersQuery>,
    ) -> WebResult<HttpResponse> {
        let current_user = Self::extract_current_user(&request)?;
        let target_company_id = company_id.into_inner();

        // Authorization check
        if !current_user.can_manage_company(target_company_id) {
            return Err(InterfaceError::InsufficientPermissions);
        }

        let users = self
            .user_repository
            .list_by_company(target_company_id)
            .await
            .map_err(|e| InterfaceError::ApplicationError(e.into()))?;

        let users_dto: Vec<UserDto> = users
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

        Ok(HttpResponse::Ok().json(users_dto))
    }

    fn extract_current_user(request: &HttpRequest) -> WebResult<crate::domain::entities::User> {
        // This would extract the user from JWT claims added by middleware
        // For now, return a mock admin user for testing
        Ok(crate::domain::entities::User::new(
            "mock-keycloak-id".to_string(),
            "admin".to_string(),
            "admin@example.com".to_string(),
            crate::domain::enums::UserRole::Admin,
            None,
        )
        .map_err(|e| InterfaceError::ApplicationError(e.into()))?)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ListCompaniesQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ListCompanyUsersQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
