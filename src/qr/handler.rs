use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::adapters::mongo::QrRepository;
use super::service::Service;
use super::{PageType, QrPage};

const ADMIN_PASSWORD: &str = "password";

#[derive(Serialize, Deserialize)]
struct GenerateRequest {
    id: String,
    admin_password: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateRequest {
    secret: Uuid,
    page: QrPage,
}

#[post("/qr/generate")]
async fn generate_qr(
    request: Json<GenerateRequest>,
    service: web::Data<Service<QrRepository>>,
) -> impl Responder {
    if request.admin_password != ADMIN_PASSWORD {
        return HttpResponse::Unauthorized().finish();
    }

    match service.generate_qr(&request.id).await {
        Ok(()) => HttpResponse::Ok().body("QR successfully generated"),
        Err(e) => e.error_response(),
    }
}

#[get("/by/{shirt_id}")]
async fn get_user_page(
    service: web::Data<Service<QrRepository>>,
    path: web::Path<String>,
) -> impl Responder {
    let shirt_id = path.into_inner();
    match service.get_page(&shirt_id).await {
        Ok(PageType::Html(html)) => HttpResponse::Ok().content_type("text/html").body(html),
        Ok(PageType::Redirect(url)) => {
            return HttpResponse::Found()
                .append_header(("Location", url))
                .finish()
        }
        Err(e) => e.error_response(),
    }
}

#[get("/edit/{shirt_secret}")]
async fn edit_user_page(
    service: web::Data<Service<QrRepository>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let secret = path.into_inner();

    match service.get_edit_page(secret).await {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => e.error_response(),
    }
}

#[post("/update")]
async fn update_user_page(
    service: web::Data<Service<QrRepository>>,
    request: Json<UpdateRequest>,
) -> impl Responder {
    match service.update_page(request.secret, &request.page).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => e.error_response(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_page)
        .service(edit_user_page)
        .service(generate_qr)
        .service(update_user_page);
}
