use async_trait::async_trait;
use auth_service::application::dto::{CompanyDto, UserDto};
use auth_service::application::queries::{GetCompanyByIdQuery, GetUserByIdQuery, ListUsersQuery};
use auth_service::application::query_handlers::{
    GetCompanyByIdQueryHandler, GetUserByIdQueryHandler, ListUsersQueryHandler, QueryHandler,
};
use auth_service::domain::entities::{Company, User};
use auth_service::domain::enums::UserRole;
use auth_service::domain::errors::DomainError;
use auth_service::domain::repositories::{CompanyRepository, UserRepository};
use uuid::Uuid;

// Mock UserRepository for query tests
struct MockUserRepository {
    users: Vec<User>,
}

impl MockUserRepository {
    fn new() -> Self {
        let user1 = User::new(
            "keycloak-1".to_string(),
            "user1".to_string(),
            "user1@example.com".to_string(),
            UserRole::User,
            None,
        )
        .unwrap();

        let user2 = User::new(
            "keycloak-2".to_string(),
            "user2".to_string(),
            "user2@example.com".to_string(),
            UserRole::Admin,
            None,
        )
        .unwrap();

        Self {
            users: vec![user1, user2],
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, _user: &User) -> Result<User, DomainError> {
        unimplemented!()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        Ok(self.users.iter().find(|u| u.id == id).cloned())
    }

    async fn find_by_keycloak_id(&self, _keycloak_id: &str) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn find_by_email(&self, _email: &str) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn find_by_username(&self, _username: &str) -> Result<Option<User>, DomainError> {
        Ok(None)
    }

    async fn update(&self, _user: &User) -> Result<User, DomainError> {
        unimplemented!()
    }

    async fn delete(&self, _id: Uuid) -> Result<(), DomainError> {
        unimplemented!()
    }

    async fn list_by_company(&self, _company_id: Uuid) -> Result<Vec<User>, DomainError> {
        Ok(self.users.clone())
    }

    async fn list_all(&self) -> Result<Vec<User>, DomainError> {
        Ok(self.users.clone())
    }
}

// Mock CompanyRepository for query tests
struct MockCompanyRepository {
    companies: Vec<Company>,
}

impl MockCompanyRepository {
    fn new() -> Self {
        let company1 = Company::new(
            "Company 1".to_string(),
            Some("Description 1".to_string()),
            Uuid::new_v4(),
        );

        let company2 = Company::new(
            "Company 2".to_string(),
            Some("Description 2".to_string()),
            Uuid::new_v4(),
        );

        Self {
            companies: vec![company1, company2],
        }
    }
}

#[async_trait]
impl CompanyRepository for MockCompanyRepository {
    async fn create(&self, _company: &Company) -> Result<Company, DomainError> {
        unimplemented!()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Company>, DomainError> {
        Ok(self.companies.iter().find(|c| c.id == id).cloned())
    }

    async fn find_by_name(&self, _name: &str) -> Result<Option<Company>, DomainError> {
        Ok(None)
    }

    async fn update(&self, _company: &Company) -> Result<Company, DomainError> {
        unimplemented!()
    }

    async fn delete(&self, _id: Uuid) -> Result<(), DomainError> {
        unimplemented!()
    }

    async fn list_all(&self) -> Result<Vec<Company>, DomainError> {
        Ok(self.companies.clone())
    }

    async fn list_by_user(&self, _user_id: Uuid) -> Result<Vec<Company>, DomainError> {
        Ok(self.companies.clone())
    }
}

#[tokio::test]
async fn test_get_user_by_id_query_handler() {
    let user_repo = Box::new(MockUserRepository::new());
    let handler = GetUserByIdQueryHandler::new(user_repo);

    let users = MockUserRepository::new().users;
    let test_user_id = users[0].id;

    let query = GetUserByIdQuery {
        user_id: test_user_id,
    };
    let result = handler.handle(query).await;

    assert!(result.is_ok());
    let user_dto = result.unwrap();
    assert!(user_dto.is_some());
    assert_eq!(user_dto.unwrap().id, test_user_id);
}

#[tokio::test]
async fn test_get_user_by_id_query_handler_not_found() {
    let user_repo = Box::new(MockUserRepository::new());
    let handler = GetUserByIdQueryHandler::new(user_repo);

    let query = GetUserByIdQuery {
        user_id: Uuid::new_v4(),
    }; // Non-existent user
    let result = handler.handle(query).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_list_users_query_handler() {
    let user_repo = Box::new(MockUserRepository::new());
    let handler = ListUsersQueryHandler::new(user_repo);

    let query = ListUsersQuery {
        company_id: None,
        role: None,
        page: 1,
        page_size: 10,
    };

    let result = handler.handle(query).await;

    assert!(result.is_ok());
    let users_dto = result.unwrap();
    assert_eq!(users_dto.len(), 2);
}

#[tokio::test]
async fn test_get_company_by_id_query_handler() {
    let company_repo = Box::new(MockCompanyRepository::new());
    let handler = GetCompanyByIdQueryHandler::new(company_repo);

    let companies = MockCompanyRepository::new().companies;
    let test_company_id = companies[0].id;

    let query = GetCompanyByIdQuery {
        company_id: test_company_id,
    };
    let result = handler.handle(query).await;

    assert!(result.is_ok());
    let company_dto = result.unwrap();
    assert!(company_dto.is_some());
    assert_eq!(company_dto.unwrap().id, test_company_id);
}
