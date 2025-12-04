use crate::keycloak_client::TokenResponse;
use crate::user_dto::{AuthDto, CreateUserDto, UserAttributesResponse, UserCreatedResponse};
use crate::user_entity::User;
use crate::user_repository::UserRepository;
use reqwest::Client;

const DEFAULT_ROLE: &str = "user";

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn register_user(
        &self,
        dto: CreateUserDto,
        client: &Client,
    ) -> Result<UserCreatedResponse, String> {
        println!("=== CREATE USER REQUEST ===");
        println!("Username: {}", dto.username);

        let user = User::new(
            dto.username.clone(),
            dto.email.clone(),
            dto.first_name.clone(),
            dto.last_name.clone(),
            dto.company_name.clone(),
            dto.station_name.clone(),
        );

        let user_id = self
            .repository
            .create_user(user, dto.password, DEFAULT_ROLE, client)
            .await?;

        if let Some(ref id) = user_id {
            println!("✓ User created with ID: {}", id);
        } else {
            println!("✓ User created (no ID returned)");
        }
        println!("✓ Role '{}' assigned", DEFAULT_ROLE);
        println!("✓ Attributes added for company_name and station_name");

        Ok(UserCreatedResponse {
            message: format!("User created successfully with '{}' role", DEFAULT_ROLE),
            user_id,
            role: DEFAULT_ROLE.to_string(),
            attributes: UserAttributesResponse {
                company_name: dto.company_name,
                station_name: dto.station_name,
            },
        })
    }

    pub async fn authenticate_user(
        &self,
        dto: AuthDto,
        client: &Client,
    ) -> Result<TokenResponse, String> {
        println!("=== AUTHENTICATE USER REQUEST ===");
        println!("Username: {}", dto.username);

        let token_response = self
            .repository
            .authenticate(&dto.username, &dto.password, client)
            .await?;

        println!("✓ Authentication successful");

        Ok(token_response)
    }
}