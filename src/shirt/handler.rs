use std::fs::read_to_string;
use std::path::Path;

use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use super::adapters::postgre::Postgre;
use super::service::Service;

#[derive(Deserialize)]
pub struct UpdateForm {
    pub secret: Uuid,
    pub new_url: String,
}

#[derive(Deserialize)]
pub struct CreateForm {
    pub id: String,
    pub url: String,
    pub admin_pass: String,
}

#[get("/shirts/edit/{shirt_secret}")]
async fn edit_shirt(service: web::Data<Service<Postgre>>, path: web::Path<Uuid>) -> impl Responder {
    let secret = path.into_inner();

    let shirt = match service.get_shirt_from_secret(secret).await {
        Ok(shirt) => shirt,
        Err(e) => return e.error_response(),
    };

    let path = Path::new("./static/update_form.html");

    let form_html = match read_to_string(path) {
        Ok(form) => form,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let form_html = form_html
        .replace("{{shirt_id}}", &shirt.id)
        .replace("{{shirt_secret}}", &secret.to_string())
        .replace("{{redirect_url}}", &shirt.redirect_url);

    HttpResponse::Ok().content_type("text/html").body(form_html)
}

#[post("/shirts/new")]
async fn new_shirt(
    service: web::Data<Service<Postgre>>,
    request: web::Json<CreateForm>,
) -> impl Responder {
    let response = service.create_shirt(&request.id, &request.url, &request.admin_pass);
    match response.await {
        Ok(()) => HttpResponse::Ok().body("Shirt succesfully created!"),
        Err(e) => e.error_response(),
    }
}

#[post("/shirts/update")]
async fn update_shirt(
    service: web::Data<Service<Postgre>>,
    request: web::Json<UpdateForm>,
) -> impl Responder {
    match service.update_url(request.secret, &request.new_url).await {
        Ok(()) => HttpResponse::Ok()
            .content_type("text/html")
            .body("Shirt succesfully updated!"),
        Err(e) => e.error_response(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(new_shirt);
    cfg.service(edit_shirt);
    cfg.service(update_shirt);
}
