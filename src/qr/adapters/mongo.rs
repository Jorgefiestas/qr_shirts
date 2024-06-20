use log::info;
use mongodb::bson::{self, doc};
use mongodb::Collection;
use sqlx::PgPool;

use crate::qr::port::Repository;
use crate::qr::{QrError, QrPage};

#[derive(Clone)]
pub struct QrRepository {
    db_pool: PgPool,
    collection: Collection<QrPage>,
}

impl QrRepository {
    pub fn new(db_pool: PgPool, collection: Collection<QrPage>) -> Self {
        Self {
            db_pool,
            collection,
        }
    }
}

impl Repository for QrRepository {
    async fn create_page(&self, qr_page: &QrPage) -> Result<(), QrError> {
        let filter = doc! { "id": &qr_page.id };

        // TODO: replace with primary key XD
        if let Some(_) = self.collection.find_one(filter, None).await? {
            return Err(QrError::PageAlreadyExists);
        }

        self.collection.insert_one(qr_page, None).await?;
        Ok(())
    }

    async fn get_page(&self, id: &str) -> Result<QrPage, QrError> {
        let filter = doc! { "id": id };
        info!("{:?}", filter);
        match self.collection.find_one(filter, None).await? {
            Some(user_page) => Ok(user_page),
            None => Err(QrError::PageNotFound),
        }
    }

    async fn update_page(&self, qr_page: &QrPage) -> Result<(), QrError> {
        let filter = doc! { "id": &qr_page.id };
        let update = doc! {
            "$set": {
                "template_id": qr_page.template_id,
                "parameters": bson::to_bson(&qr_page.parameters)?,
            }
        };

        let update_result = self.collection.update_one(filter, update, None).await?;
        if update_result.matched_count == 0 {
            return Err(QrError::PageNotFound);
        }
        Ok(())
    }

    async fn create_secret(&self, id: &str, secret: uuid::Uuid) -> Result<(), QrError> {
        sqlx::query!(
            "INSERT INTO shirts (secret, id) VALUES ($1, $2)",
            secret,
            id
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    async fn get_id_from_secret(&self, secret: uuid::Uuid) -> Result<String, QrError> {
        let id: String = sqlx::query_scalar!("SELECT id FROM shirts WHERE secret = $1", secret)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(id)
    }
}
