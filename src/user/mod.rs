use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use bcrypt::BcryptError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod adapter;
pub mod handler;
pub mod port;
pub mod service;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Bcrypt error {0}")]
    BcryptError(#[from] BcryptError),

    #[error("Incorrect username or password")]
    AuthenticationError,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
