use axum::Router;
use sqlx::MySqlPool;
use redis::Client;
use std::sync::Arc;

pub mod inner_api;
pub mod open_api;
pub mod redirect;

use crate::services::TinyurlService;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub redis: Arc<Client>,
    pub tinyurl_service: TinyurlService,
}

pub fn create_routes(pool: MySqlPool, redis: Client) -> Router {
    let tinyurl_service = TinyurlService::new(pool.clone(), redis.clone());

    let state = AppState {
        pool,
        redis: Arc::new(redis),
        tinyurl_service,
    };

    Router::new()
        .merge(inner_api::routes())
        .merge(open_api::routes())
        .merge(redirect::routes())
        .with_state(state)
}
