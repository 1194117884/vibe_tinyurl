use axum::{
    extract::{Json, State},
    http::HeaderMap,
    routing::post,
    Router,
};
use serde_json::json;

use crate::config::CONFIG;
use crate::models::{CacheRefreshRequest, CreateUrlRequest};
use crate::routes::AppState;
use crate::error::AppError;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create))
        .route("/cache/refresh", post(cache_refresh))
}

async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateUrlRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let api_key = headers
        .get("apiKey")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let length = request.len.unwrap_or(6);
    let short_uri = state.tinyurl_service.create(api_key, &request.url, length).await?;
    let full_url = CONFIG.tinyurl_format.replace("%s", &short_uri);

    Ok(Json(json!({
        "code": "0",
        "message": "成功",
        "data": full_url
    })))
}

async fn cache_refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CacheRefreshRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let api_key = headers
        .get("apiKey")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let short_uri = state.tinyurl_service.refresh(api_key, &request.short_uri).await?;
    let full_url = CONFIG.tinyurl_format.replace("%s", &short_uri);

    Ok(Json(json!({
        "code": "0",
        "message": "成功",
        "data": full_url
    })))
}
