use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
