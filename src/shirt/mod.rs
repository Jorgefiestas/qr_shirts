use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

pub mod adapters;
pub mod handler;
pub mod port;
pub mod service;

// TODO: move reedem_code and redirect_url to separate DB manager?

#[derive(Serialize)]
pub struct Shirt {
    id: String,
    owner_id: Option<i32>,
    redirect_url: String,
    reedem_code: String,
}

#[derive(Debug, Error)]
pub enum ShirtError {
    #[error("Shirt already has an owner!")]
    AlreadyOwned,

    #[error("Unauthorized action!")]
    HasNoOwner,

    #[error("Unauthorized action!")]
    Unauthorized,
}

impl ResponseError for ShirtError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body("Something broke!")
    }
}
