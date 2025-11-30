use super::entities::User;
use super::value_objects::Role;

pub struct AuthorizationService;

impl AuthorizationService {
    pub fn can_create_user(requester_role: &Role, target_role: &Role) -> bool {
        match requester_role {
            Role::Admin => true,
            Role::Partner => matches!(target_role, Role::Operator),
            Role::Operator => false,
        }
    }

    pub fn can_manage_user(requester: &User, target: &User) -> bool {
        match requester.role {
            Role::Admin => true,
            Role::Partner => {
                matches!(target.role, Role::Operator)
                    && requester.organisation_name == target.organisation_name
            }
            Role::Operator => false,
        }
    }

    pub fn filter_accessible_organisations(
        user: &User,
        requested_orgs: Vec<String>,
    ) -> Vec<String> {
        match user.role {
            Role::Admin => requested_orgs,
            Role::Partner | Role::Operator => {
                if let Some(org) = &user.organisation_name {
                    requested_orgs
                        .into_iter()
                        .filter(|o| o == org.value())
                        .collect()
                } else {
                    vec![]
                }
            }
        }
    }
}
