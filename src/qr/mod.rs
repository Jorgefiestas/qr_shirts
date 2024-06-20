use std::collections::HashMap;

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod adapters;
pub mod handler;
mod port;
pub mod service;
mod templates;

#[derive(Debug, Serialize, Deserialize)]
pub struct QrPage {
    id: String,
    template_id: i32,
    parameters: HashMap<String, String>,
}

pub enum PageType {
    Html(String),
    Redirect(String),
}

#[derive(Debug, Error)]
pub enum QrError {
    #[error("Rendering error!")]
    RenderingError(#[from] askama::Error),

    #[error("PostgreDB error!")]
    PostgreError(#[from] sqlx::Error),

    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("Bson error: {0}")]
    BSONError(#[from] mongodb::bson::ser::Error),

    #[error("Invalid template ID!")]
    InvalidTemplateID,

    #[error("Template parameter not found!")]
    TemplateParameterNotFound,

    #[error("User page not found!")]
    PageNotFound,

    #[error("Trying to create an already existing user page!")]
    PageAlreadyExists,

    #[error("Unauthorized")]
    Unauthorized,
}

impl ResponseError for QrError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body("Something broke!")
    }
}
