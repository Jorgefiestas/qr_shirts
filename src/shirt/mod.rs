use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

pub mod adapters;
pub mod handler;
pub mod port;
pub mod service;

// TODO: move reedem_code and redirect_url to separate DB manager?

#[derive(Serialize)]
pub struct Shirt {
    id: String,
    secret: Uuid,
    redirect_url: String,
}

#[derive(Debug, Error)]
pub enum ShirtError {
    #[error("Database error!")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Unauthorized action!")]
    Unauthorized,
}

impl ResponseError for ShirtError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body("Something broke!")
    }
}
