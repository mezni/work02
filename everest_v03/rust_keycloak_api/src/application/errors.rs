#[derive(Debug)]
pub enum ApplicationError {
    DomainError(String),
    InfrastructureError(String),
}
