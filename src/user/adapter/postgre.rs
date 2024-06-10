use crate::user::port::Repository;
use crate::user::{User, UserError};

pub struct Postgre {}

impl Postgre {
    pub fn new() -> Self {}
}

impl Repository for Postgre {
    async fn create_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
    ) -> Result<(), UserError> {
        todo!()
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<User, UserError> {
        todo!()
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, UserError> {
        todo!()
    }

    async fn get_user_password(&self, email: &str) -> Result<String, UserError> {
        todo!()
    }
}
