use actix_web::{get, web, HttpResponse, Responder, ResponseError};

use crate::shirt::adapters::postgre::Postgre;
use crate::shirt::service::Service;

#[get("/by/{shirt_id}")]
async fn read_qr(
    shirt_service: web::Data<Service<Postgre>>,
    path: web::Path<String>,
) -> impl Responder {
    let shirt_id = path.into_inner();

    let target = match shirt_service.get_redirect_url(&shirt_id).await {
        Ok(url) => url,
        Err(e) => return e.error_response(),
    };

    HttpResponse::Found()
        .append_header(("Location", target))
        .finish()
}
