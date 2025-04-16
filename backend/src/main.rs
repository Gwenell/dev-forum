use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use sea_orm::DatabaseConnection;

mod api;
mod auth;
mod config;
mod db;
mod error;
mod middleware;
mod models;
mod services;
mod utils;
// mod ws; // Commented out until implemented

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Load configuration
    let config = config::Config::from_env();
    let port = config.port;

    // Connect to the database
    let db_connection = db::establish_connection()
        .await
        .expect("Failed to connect to the database");
    
    // Run migrations
    db::run_migrations(&db_connection)
        .await
        .expect("Failed to run database migrations");

    log::info!("Starting server on port {}", port);

    // Start the HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin(&config.frontend_url)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db_connection.clone()))
            .configure(api::configure_routes)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .workers(2)
    .run()
    .await
}
