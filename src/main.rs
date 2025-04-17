use actix_cors::Cors;
use actix_web::{ get, http::{self}, middleware::{self}, web::{self, route}, App, HttpServer};
use contexts::{connection::{create_pool, DbPool}, logger::write_log};
use handlers::generic_handler::generic_scope;
use services::generic_service::{self};

mod contexts {
    pub mod connection;
    pub mod  model;
    pub mod logger;
}

mod handlers {
    pub mod generic_handler;
}

mod services {
    pub mod generic_service;
}

#[get("/")]
async fn health_check() -> String {
    format!("Welcome to the UBS trade dashboard!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Aktifkan logging
    dotenvy::dotenv().ok();
    let db_pool: DbPool = create_pool("S21Plus_RO").await.unwrap();

    write_log("INFO", "Test log message: Logging is working");
    println!("🚀 Application started");
    println!("Application running on http://127.0.0.1:8000");
    println!("🚀 Welcome to the UBS trade dashboard!");
    
    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allow_any_origin() // Allow semua request
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .service(web::scope("/v1")
            .service(generic_scope())
        )
        .app_data(web::Data::new(db_pool.clone()))
        .app_data(web::JsonConfig::default().error_handler(generic_service::GenericService::json_error_handler))
        .service(health_check)
        .default_service(route().to(generic_service::GenericService::not_found))
        .wrap(middleware::Logger::default()) // Logging middleware
        .wrap(middleware::NormalizePath::trim()) // 🔥 Normalisasi path (opsional)
        .wrap(middleware::Logger::default())
        .wrap(cors)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })
}