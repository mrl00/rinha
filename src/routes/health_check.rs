use actix_web::{Responder, HttpResponse};

#[actix_web::get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
