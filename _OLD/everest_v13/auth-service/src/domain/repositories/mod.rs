pub mod user_repository;
pub mod organisation_repository;
pub mod audit_log_repository;
pub mod role_request_repository;

// Re-export RepositoryError
pub use user_repository::RepositoryError;