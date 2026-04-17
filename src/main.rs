mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;
mod utils;
mod middleware;
use config::CONFIG;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Vibe TinyURL starting on port {}", CONFIG.server_port);
}
