use actix_session::{storage, SessionMiddleware};
use actix_web::{cookie, App, HttpServer};

mod shirt;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_store = storage::RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();
    let secret_key = cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .configure(user::handler::configure)
            .configure(shirt::handler::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
