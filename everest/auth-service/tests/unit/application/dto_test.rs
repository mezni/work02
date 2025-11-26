use validator::Validate;
use uuid::Uuid;
use auth_service::application::dto::{
    CreateUserDto, UpdateUserDto, CreateCompanyDto, UpdateCompanyDto, 
    UserDto, CompanyDto, LoginRequest, RegisterRequest
};
use auth_service::domain::enums::UserRole;

#[test]
fn test_create_user_dto_validation() {
    // Valid DTO
    let valid_dto = CreateUserDto {
        username: "validuser".to_string(),
        email: "valid@example.com".to_string(),
        password: "password123".to_string(),
        role: UserRole::User,
        company_id: None,
    };
    
    assert!(valid_dto.validate().is_ok());
    
    // Invalid DTO - short username
    let invalid_dto = CreateUserDto {
        username: "ab".to_string(), // Too short
        email: "valid@example.com".to_string(),
        password: "password123".to_string(),
        role: UserRole::User,
        company_id: None,
    };
    
    assert!(invalid_dto.validate().is_err());
    
    // Invalid DTO - invalid email
    let invalid_dto = CreateUserDto {
        username: "validuser".to_string(),
        email: "invalid-email".to_string(),
        password: "password123".to_string(),
        role: UserRole::User,
        company_id: None,
    };
    
    assert!(invalid_dto.validate().is_err());
    
    // Invalid DTO - short password
    let invalid_dto = CreateUserDto {
        username: "validuser".to_string(),
        email: "valid@example.com".to_string(),
        password: "short".to_string(), // Too short
        role: UserRole::User,
        company_id: None,
    };
    
    assert!(invalid_dto.validate().is_err());
}

#[test]
fn test_update_user_dto_validation() {
    // Valid DTO with some fields
    let valid_dto = UpdateUserDto {
        username: Some("newusername".to_string()),
        email: Some("new@example.com".to_string()),
        role: Some(UserRole::Admin),
        company_id: None,
    };
    
    assert!(valid_dto.validate().is_ok());
    
    // Valid DTO with empty fields (all optional)
    let valid_dto = UpdateUserDto {
        username: None,
        email: None,
        role: None,
        company_id: None,
    };
    
    assert!(valid_dto.validate().is_ok());
    
    // Invalid DTO - short username
    let invalid_dto = UpdateUserDto {
        username: Some("ab".to_string()), // Too short
        email: Some("valid@example.com".to_string()),
        role: Some(UserRole::User),
        company_id: None,
    };
    
    assert!(invalid_dto.validate().is_err());
}

#[test]
fn test_create_company_dto_validation() {
    // Valid DTO
    let valid_dto = CreateCompanyDto {
        name: "Valid Company".to_string(),
        description: Some("A valid company description".to_string()),
    };
    
    assert!(valid_dto.validate().is_ok());
    
    // Invalid DTO - empty name
    let invalid_dto = CreateCompanyDto {
        name: "".to_string(), // Empty name
        description: Some("Description".to_string()),
    };
    
    assert!(invalid_dto.validate().is_err());
    
    // Invalid DTO - long description
    let long_description = "a".repeat(1001);
    let invalid_dto = CreateCompanyDto {
        name: "Valid Company".to_string(),
        description: Some(long_description),
    };
    
    assert!(invalid_dto.validate().is_err());
}

#[test]
fn test_user_dto_creation() {
    let user_dto = UserDto {
        id: Uuid::new_v4(),
        keycloak_id: "keycloak-123".to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::Admin,
        company_id: Some(Uuid::new_v4()),
        email_verified: true,
        created_at: "2023-01-01T00:00:00Z".to_string(),
        updated_at: "2023-01-01T00:00:00Z".to_string(),
    };
    
    assert_eq!(user_dto.username, "testuser");
    assert_eq!(user_dto.email, "test@example.com");
    assert_eq!(user_dto.role, UserRole::Admin);
    assert!(user_dto.company_id.is_some());
}

#[test]
fn test_company_dto_creation() {
    let company_dto = CompanyDto {
        id: Uuid::new_v4(),
        name: "Test Company".to_string(),
        description: Some("Test Description".to_string()),
        created_by: Uuid::new_v4(),
        created_at: "2023-01-01T00:00:00Z".to_string(),
        updated_at: "2023-01-01T00:00:00Z".to_string(),
    };
    
    assert_eq!(company_dto.name, "Test Company");
    assert_eq!(company_dto.description, Some("Test Description".to_string()));
}

#[test]
fn test_login_request_creation() {
    let login_request = LoginRequest {
        username: "testuser".to_string(),
        password: "password".to_string(),
    };
    
    assert_eq!(login_request.username, "testuser");
    assert_eq!(login_request.password, "password");
}

#[test]
fn test_register_request_creation() {
    let register_request = RegisterRequest {
        username: "newuser".to_string(),
        email: "newuser@example.com".to_string(),
        password: "password".to_string(),
    };
    
    assert_eq!(register_request.username, "newuser");
    assert_eq!(register_request.email, "newuser@example.com");
    assert_eq!(register_request.password, "password");
}
