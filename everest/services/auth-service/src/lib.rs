pub mod domain;
//pub mod application;
pub mod infrastructure;
//pub mod interfaces;

// Re-export specific domain items
pub use domain::{
    entities::{AuditLog, Company, User},
    enums::{AuditAction, UserRole},
    errors::DomainError,
    repositories::{AuditRepository, CompanyRepository, UserRepository},
    value_objects::Email,
};

// Re-export specific infrastructure items
pub use infrastructure::{
    config::Settings,
    create_database,
    database::repositories::{AuditRepositoryImpl, CompanyRepositoryImpl, UserRepositoryImpl},
    errors::InfrastructureError,
    Database,
};
