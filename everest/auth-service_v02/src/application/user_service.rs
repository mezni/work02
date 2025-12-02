use crate::domain::user::User;
use crate::infrastructure::user_repository::UserRepository;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    pub repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn register(&self, username: String, email: String) -> User {
        let user = User::new(username, email);
        self.repo.save(&user).await;
        user
    }
}