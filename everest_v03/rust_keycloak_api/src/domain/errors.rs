#[derive(Debug)]
pub enum DomainError {
    RepositoryError(String),
    ValidationError(String),
}
