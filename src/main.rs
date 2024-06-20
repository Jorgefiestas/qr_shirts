use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use sqlx::PgPool;
use std::env;
use thiserror::Error;

mod qr;

use crate::qr::adapters::mongo::QrRepository;
use crate::qr::service::Service as QrService;

#[derive(Debug, Error)]
enum InitializationError {
    #[error("PostgreDB error: {0}")]
    PostgreError(#[from] sqlx::Error),

    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("Env error: {0}")]
    EnvError(#[from] std::env::VarError),
}

async fn initialize_service() -> Result<QrService<QrRepository>, InitializationError> {
    const MONGO_DB_NAME: &str = "qr_shirts";
    const MONGO_DB_COLLECTION: &str = "qr_page";

    let postgre_url = env::var("POSTGRE_URL")?;
    let mongo_url = env::var("MONGO_URL")?;

    let client_options = ClientOptions::parse(&mongo_url).await?;
    let client = Client::with_options(client_options)?;

    let database = client.database(MONGO_DB_NAME);
    let collection = database.collection(MONGO_DB_COLLECTION);

    let db_pool = PgPool::connect(&postgre_url)
        .await
        .expect("Failed to create pool");

    let repo = QrRepository::new(db_pool, collection);
    Ok(QrService::new(repo))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let service = initialize_service().await.expect("Initialization Failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .configure(qr::handler::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
