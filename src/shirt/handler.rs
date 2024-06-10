use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;

use super::adapters::mongo::Mongo;
use super::service::Service;

use crate::user::adapter::postgre::Postgre as UserPG;
use crate::user::service::Service as UserService;

#[derive(Deserialize)]
pub struct UpdateRequest {
    pub shirt_id: String,
    pub new_url: String,
}

#[derive(Deserialize)]
pub struct ClaimRequest {
    pub reedem_code: String,
    pub email: String,
}

#[get("/shirts/")]
async fn get_shirts(service: web::Data<Service<Mongo>>, session: Session) -> impl Responder {
    let user_id = match session.get::<i32>("user_id") {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match service.get_shirts_by_owner(user_id) {
        Ok(shirts) => HttpResponse::Ok().json(shirts),
        Err(e) => e.error_response(),
    }
}

#[get("/shirt/{shirt_id}")]
async fn get_shirt(
    service: web::Data<Service<Mongo>>,
    session: Session,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = match session.get::<i32>("user_id") {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let shirt_id = path.into_inner();

    match service.get_shirt(&shirt_id, user_id) {
        Ok(shirt) => HttpResponse::Ok().json(shirt),
        Err(e) => e.error_response(),
    }
}

#[post("/shirts/claim")]
async fn claim(
    shirt_service: web::Data<Service<Mongo>>,
    user_service: web::Data<UserService<UserPG>>,
    request: web::Json<ClaimRequest>,
) -> impl Responder {
    let shirt_id = match shirt_service.get_id_from_code(&request.reedem_code) {
        Ok(id) => id,
        Err(e) => return e.error_response(),
    };

    // TODO: Create account when email not found (?)

    let user = match user_service.get_user_by_email(&request.email).await {
        Ok(user) => user,
        Err(e) => return e.error_response(),
    };

    match shirt_service.claim_shirt(&shirt_id, user.id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.error_response(),
    }
}

#[post("/shirts/update/")]
async fn update(
    service: web::Data<Service<Mongo>>,
    request: web::Json<UpdateRequest>,
    session: Session,
) -> impl Responder {
    let user_id = match session.get::<i32>("user_id") {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match service.update_url(&request.shirt_id, user_id, &request.new_url) {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => e.error_response(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_shirts);
    cfg.service(update);
}
