use actix_cors::Cors;
use actix_web::{ get, http::{self}, middleware::{self}, web::{self, route}, App, HttpServer};
use contexts::{api_docs::ApiDoc, connection::{create_pool, DbPool}, logger::write_log};
use handlers::{chart_handler::chart_scope, data_handler::data_scope, generic_handler::generic_scope};
use services::generic_service::{self};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod contexts {
    pub mod connection;
    pub mod  model;
    pub mod logger;
    pub mod api_docs;
}

mod handlers {
    pub mod generic_handler;
    pub mod data_handler;
    pub mod chart_handler;
}

mod services {
    pub mod generic_service;
    pub mod data_service;
    pub mod chart_service;
    pub mod validation_service;
}

#[get("/")]
async fn health_check() -> String {
    format!("Welcome to the UBS trade dashboard!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Aktifkan logging
    dotenvy::dotenv().ok();
    let db_pool: DbPool = create_pool("db12877").await.unwrap();

    write_log("INFO", "Test log message: Logging is working");
    println!("🚀 Application started");
    println!("Application running on http://127.0.0.1:8001");
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
            .service(web::scope("/api/v1")
            .service(generic_scope())
            .service(data_scope())
            .service(chart_scope())
        )
        .app_data(web::Data::new(db_pool.clone()))
        .app_data(web::JsonConfig::default().error_handler(generic_service::GenericService::json_error_handler))
        .service(health_check)
        .service(
            SwaggerUi::new("/docs/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .default_service(route().to(generic_service::GenericService::not_found))
        .wrap(middleware::Logger::default()) // Logging middleware
        .wrap(middleware::NormalizePath::trim()) // 🔥 Normalisasi path (opsional)
        .wrap(middleware::Logger::default())
        .wrap(cors)
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })
}