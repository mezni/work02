use uuid::Uuid;

#[derive(Debug)]
pub struct GetUserQuery {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct GetUserByKeycloakIdQuery {
    pub keycloak_id: String,
}

#[derive(Debug)]
pub struct ListUsersByOrganisationQuery {
    pub organisation_name: String,
    pub requester_id: Uuid,
}
