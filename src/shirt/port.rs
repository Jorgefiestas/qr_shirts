use uuid::Uuid;

use super::{Shirt, ShirtError};

pub trait Repository {
    async fn create_shirt(&self, shirt: &Shirt) -> Result<(), ShirtError>;

    async fn update_shirt(&self, id: &str, url: &str) -> Result<(), ShirtError>;

    async fn get_shirt_by_id(&self, id: &str) -> Result<Shirt, ShirtError>;
    async fn get_redirect_url(&self, id: &str) -> Result<String, ShirtError>;
    async fn get_shirt_from_secret(&self, secret: Uuid) -> Result<Shirt, ShirtError>;
}
