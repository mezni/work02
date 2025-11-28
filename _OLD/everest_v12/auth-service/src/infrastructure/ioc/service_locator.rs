use crate::{
    domain::repositories::{UserRepository, OrganisationRepository, StationRepository},
    infrastructure::{
        config::AppConfig,
        db::{UserRepositoryPg, OrganisationRepositoryPg, StationRepositoryPg, get_db_pool},
        jwt::token_enricher::JwtTokenEnricher,
    },
    application::handlers::{
        RegisterUserHandler,
        LoginHandler,
        AdminCreateUserHandler,
    },
};

pub struct ServiceLocator {
    user_repo: UserRepositoryPg,
    organisation_repo: OrganisationRepositoryPg,
    station_repo: StationRepositoryPg,
    jwt_enricher: JwtTokenEnricher,
}

impl ServiceLocator {
    pub async fn new(config: AppConfig) -> Result<Self, anyhow::Error> {
        let db_pool = get_db_pool(&config).await?;
        
        let user_repo = UserRepositoryPg::new(db_pool.clone());
        let organisation_repo = OrganisationRepositoryPg::new(db_pool.clone());
        let station_repo = StationRepositoryPg::new(db_pool);
        let jwt_enricher = JwtTokenEnricher::new(&config);

        Ok(ServiceLocator {
            user_repo,
            organisation_repo,
            station_repo,
            jwt_enricher,
        })
    }

    pub fn get_register_handler(&self) -> RegisterUserHandler<UserRepositoryPg> {
        RegisterUserHandler::new(self.user_repo.clone())
    }

    pub fn get_login_handler(&self) -> LoginHandler<UserRepositoryPg> {
        LoginHandler::new(self.user_repo.clone(), self.jwt_enricher.clone())
    }

    pub fn get_admin_create_user_handler(&self) -> AdminCreateUserHandler<UserRepositoryPg, OrganisationRepositoryPg, StationRepositoryPg> {
        AdminCreateUserHandler::new(
            self.user_repo.clone(),
            self.organisation_repo.clone(),
            self.station_repo.clone(),
        )
    }
}
