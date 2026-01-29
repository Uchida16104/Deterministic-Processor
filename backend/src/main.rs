use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use log::info;
use std::env;

mod handlers;
mod models;
mod processing;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");
    
    let allowed_origins = env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());
    
    let max_payload_size = env::var("MAX_PAYLOAD_SIZE")
        .unwrap_or_else(|_| "10485760".to_string())
        .parse::<usize>()
        .expect("MAX_PAYLOAD_SIZE must be a valid number");
    
    info!("Starting server on 0.0.0.0:{}", port);
    info!("Allowed CORS origins: {}", allowed_origins);
    info!("Max payload size: {} bytes", max_payload_size);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origins)
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(max_payload_size))
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(handlers::health_check))
                    .route("/process", web::post().to(handlers::process::process_data))
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
