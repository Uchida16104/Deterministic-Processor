pub mod process;

use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "deterministic-processor",
        "version": "0.1.0"
    }))
}
