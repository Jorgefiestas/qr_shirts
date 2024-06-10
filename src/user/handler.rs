use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;

use super::adapter::postgre::Postgre;
use super::service::Service;

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[post("/signup/")]
async fn signup(
    service: web::Data<Service<Postgre>>,
    signup_data: web::Json<SignUpRequest>,
) -> impl Responder {
    match service.create_user(signup_data.0).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => e.error_response(),
    }
}

#[post("/login/")]
async fn login(
    service: web::Data<Service<Postgre>>,
    login_data: web::Json<LoginRequest>,
    session: Session,
) -> impl Responder {
    match service
        .authenticate_user(&login_data.email, &login_data.password)
        .await
    {
        Ok(id) => {
            session.insert("user_id", &id).expect("To set session id");
            session.renew();
            HttpResponse::Ok().finish()
        }
        Err(e) => e.error_response(),
    }
}

#[get("/logout/")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().finish()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(signup);
    cfg.service(login);
    cfg.service(logout);
}
