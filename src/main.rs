use std::env;

use axum::{http::Method, Router};
use dotenv::dotenv;
use fish_logger::log_fish;
use log::info;
use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};
use tokio::{net::TcpListener, signal};
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::service_controller;

mod controllers;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("🚀 Server starting...");
    dotenv().ok();
    let app_environment = env::var("APP_ENVIRONMENT").unwrap_or("development".to_string());
    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("80".to_string());

    match app_environment.as_str() {
        "development" => {
            info!("Running in development mode");
        }
        "production" => {
            info!("Running in production mode");
        }
        _ => {
            info!("Running in development mode");
        }
    }

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes = Router::new().merge(service_controller::router().layer(cors));

    let bind_address = app_host.to_owned() + ":" + &app_port;
    let listener = TcpListener::bind(&bind_address).await?;
    axum::serve(listener, routes)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    println!("Connecting to the database...");
    DB.connect::<Ws>(bind_address)
        .await?;

    DB.use_ns("namespace").use_db("database").await?;

    info!("Server stopped.");

    println!("\nWelcome to the Rusty Fish Logger!");
    println!("Type 'quit' at any time to exit the program.\n");

    let _log_fish = log_fish(&DB).await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}