use actix_web::{web, App, HttpServer};
use log::info;

mod collectors;
mod handlers;
mod models;

use handlers::*;
use handlers::system::{shutdown_handler, reboot_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default())
        .filter(None, log::LevelFilter::Info)
        .filter_module("actix_server", log::LevelFilter::Warn)
        .filter_module("actix_builder", log::LevelFilter::Warn)
        .init();
    
    let port = std::env::var("RONITOR_PORT").unwrap_or_else(|_| "3301".to_string());
    info!("Starting server on port {}", port);
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_handler))
            .route("/metrics", web::get().to(metrics_handler))
            .route("/kernel", web::get().to(kernel_info_handler))
            .route("/network", web::get().to(network_info_handler))
            .route("/processes", web::get().to(processes_handler))
            .route("/control/shutdown", web::post().to(shutdown_handler))
            .route("/control/reboot", web::post().to(reboot_handler))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
