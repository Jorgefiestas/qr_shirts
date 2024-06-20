use uuid::Uuid;

use super::{QrError, QrPage};

pub trait Repository {
    async fn create_page(&self, user_page: &QrPage) -> Result<(), QrError>;
    async fn get_page(&self, shirt_id: &str) -> Result<QrPage, QrError>;
    async fn update_page(&self, user_page: &QrPage) -> Result<(), QrError>;
    async fn create_secret(&self, id: &str, secret: Uuid) -> Result<(), QrError>;
    async fn get_id_from_secret(&self, secret: Uuid) -> Result<String, QrError>;
}
