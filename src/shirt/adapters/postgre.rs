use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::shirt::{port::Repository, Shirt, ShirtError};

#[derive(Clone)]
pub struct Postgre {
    pub db_pool: PgPool,
}

impl Postgre {
    pub fn new(db_pool: PgPool) -> Self {
        Postgre { db_pool }
    }
}

impl Repository for Postgre {
    async fn create_shirt(&self, shirt: &Shirt) -> Result<(), ShirtError> {
        sqlx::query!(
            "INSERT INTO shirts (id, secret, redirect_url) VALUES ($1, $2, $3)",
            shirt.id,
            shirt.secret,
            shirt.redirect_url,
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn update_shirt(&self, id: &str, url: &str) -> Result<(), ShirtError> {
        sqlx::query!("UPDATE shirts SET redirect_url = $2 WHERE id = $1", id, url)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    async fn get_shirt_by_id(&self, id: &str) -> Result<Shirt, ShirtError> {
        let shirt = sqlx::query_as!(
            Shirt,
            "SELECT id, secret, redirect_url FROM shirts WHERE id = $1",
            id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(shirt)
    }

    async fn get_shirt_from_secret(&self, secret: Uuid) -> Result<Shirt, ShirtError> {
        let shirt = sqlx::query_as!(
            Shirt,
            "SELECT id, secret, redirect_url FROM shirts WHERE secret = $1",
            secret
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(shirt)
    }

    async fn get_redirect_url(&self, id: &str) -> Result<String, ShirtError> {
        let url = sqlx::query!("SELECT redirect_url FROM shirts WHERE id = $1", id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(url.redirect_url)
    }
}
