use super::{User, UserError};

pub trait Repository {
    async fn create_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
    ) -> Result<(), UserError>;

    async fn get_user_by_id(&self, user_id: i32) -> Result<User, UserError>;
    async fn get_user_by_email(&self, email: &str) -> Result<User, UserError>;
    async fn get_user_password(&self, email: &str) -> Result<String, UserError>;
}
