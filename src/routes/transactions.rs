use actix_web::{Responder, HttpResponse, web};

use crate::{services::id_exists, domain::{Transaction, TransactionDTO}};

#[actix_web::post("/clientes/{id}/transacoes")]
pub async fn transaction(path: web::Path<u32>, body: web::Json<Transaction>) -> impl Responder {
    let id = path.into_inner();
    if id_exists(id) {
        HttpResponse::Ok().json(body)
    } else {
        HttpResponse::NotFound().json(body)
    }
}
