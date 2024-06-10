use bcrypt::{hash, verify, DEFAULT_COST};
use regex::Regex;

use super::handlers::SignUpRequest;
use super::port::Repository;
use super::{User, UserError};

pub struct Service<R: Repository> {
    repo: R,
}

impl<R: Repository> Service<R> {
    pub fn new(repo: R) -> Self {
        Service { repo }
    }

    fn validate_user_data(signup_data: &SignUpRequest) -> bool {
        // TODO wrap regex in thread safe static lazy
        let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
        email_regex.is_match(&signup_data.email)
    }

    pub async fn create_user(&self, signup_request: SignUpRequest) -> Result<(), UserError> {
        if !Self::validate_user_data(&signup_request) {
            return Err(UserError::AuthenticationError);
        }
        if let Ok(_) = self.repo.get_user_by_email(&signup_request.email).await {
            return Err(UserError::AuthenticationError);
        }

        self.repo
            .create_user(
                &signup_request.email,
                &signup_request.username,
                &hash(signup_request.password, DEFAULT_COST)?,
            )
            .await
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<i32, UserError> {
        let hashed_password = self.repo.get_user_password(email).await?;

        match verify(password, &hashed_password) {
            Ok(true) => Ok(self.get_user_by_email(email).await?.id),
            Ok(false) => Err(UserError::AuthenticationError),
            Err(e) => Err(UserError::BcryptError(e)),
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, UserError> {
        self.repo.get_user_by_email(email).await
    }
}
