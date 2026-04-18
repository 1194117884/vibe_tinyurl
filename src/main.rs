mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;
mod utils;

use crate::config::CONFIG;
use crate::db::create_pool;
use crate::routes::create_routes;
use axum::serve;
use redis::Client;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tinyurl=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Vibe TinyURL starting...");

    // Create database pool
    let pool = create_pool(&CONFIG.database_url).await?;
    tracing::info!("Database pool created");

    // Create Redis client
    let redis = Client::open(CONFIG.redis_url.clone())?;
    tracing::info!("Redis client created");

    // Create routes
    let app = create_routes(pool, redis);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.server_port));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server listening on {}", addr);

    serve(listener, app).await?;

    Ok(())
}
