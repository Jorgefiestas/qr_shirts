use uuid::Uuid;

use super::port::Repository;
use super::{Shirt, ShirtError};

const ADMIN_PASS: &str = "password";

#[derive(Clone)]
pub struct Service<R: Repository> {
    repo: R,
}

impl<R: Repository> Service<R> {
    pub fn new(repo: R) -> Self {
        Service { repo }
    }

    pub async fn get_redirect_url(&self, id: &str) -> Result<String, ShirtError> {
        self.repo.get_redirect_url(id).await
    }

    pub async fn get_shirt_from_secret(&self, secret: Uuid) -> Result<Shirt, ShirtError> {
        self.repo.get_shirt_from_secret(secret).await
    }

    pub async fn update_url(&self, secret: Uuid, new_url: &str) -> Result<(), ShirtError> {
        let shirt = self.repo.get_shirt_from_secret(secret).await?;
        self.repo.update_shirt(&shirt.id, new_url).await
    }

    pub async fn create_shirt(
        &self,
        id: &str,
        redirect_url: &str,
        password: &str,
    ) -> Result<(), ShirtError> {
        if password != ADMIN_PASS {
            return Err(ShirtError::Unauthorized);
        }

        let secret = Uuid::new_v4();
        let shirt = Shirt {
            id: id.to_string(),
            redirect_url: redirect_url.to_string(),
            secret,
        };

        self.repo.create_shirt(&shirt).await
    }
}
