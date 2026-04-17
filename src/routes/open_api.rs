use axum::{
    extract::{Path, State},
    http::HeaderMap,
    routing::get,
    Router,
    Json,
};
use serde_json::json;

use crate::routes::AppState;
use crate::error::AppError;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/open/tinyurl/visit/{short_uri}", get(visit))
}

async fn visit(
    State(state): State<AppState>,
    Path(short_uri): Path<String>,
) -> Result<(HeaderMap, Json<serde_json::Value>), AppError> {
    let origin_url = state.tinyurl_service.visit_by_uri(&short_uri).await?;

    let mut headers = HeaderMap::new();
    headers.insert("origin-url", origin_url.parse().unwrap());

    Ok((headers, Json(json!({
        "code": "0",
        "message": "成功",
        "data": origin_url
    }))))
}
