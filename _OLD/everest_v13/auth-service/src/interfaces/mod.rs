pub mod handlers;
pub mod middleware;
pub mod routes;

// src/interfaces/mod.rs

use std::sync::Arc;

use crate::application::services::{
    auth_service::AuthService, 
    user_service::UserService,
    organisation_service::OrganisationService,
    audit_service::AuditService,
    role_request_service::RoleRequestService,
};


pub struct AppState {
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
    pub organisation_service: Arc<OrganisationService>,
    pub audit_service: Arc<AuditService>,
    pub role_request_service: Arc<RoleRequestService>,
}

impl AppState {
    pub fn new(
        user_service: Arc<UserService>,
        auth_service: Arc<AuthService>,
        organisation_service: Arc<OrganisationService>,
        audit_service: Arc<AuditService>,
        role_request_service: Arc<RoleRequestService>,
    ) -> Self {
        Self {
            user_service,
            auth_service,
            organisation_service,
            audit_service,
            role_request_service,
        }
    }
}