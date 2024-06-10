use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod redirect;
mod shirt;

use crate::shirt::adapters::postgre::Postgre as ShirtDB;
use crate::shirt::service::Service as ShirtService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    let shirt_repo = ShirtDB::new(db_pool);
    let shirt_service = ShirtService::new(shirt_repo);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shirt_service.clone()))
            .service(redirect::read_qr)
            .configure(shirt::handler::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
