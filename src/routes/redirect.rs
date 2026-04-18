use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::get,
    Router,
};

use crate::routes::AppState;
use crate::error::AppError;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/:short_uri", get(redirect))
}

async fn redirect(
    State(state): State<AppState>,
    Path(short_uri): Path<String>,
) -> Result<Redirect, AppError> {
    let origin_url = state.tinyurl_service.visit_by_uri(&short_uri).await?;
    Ok(Redirect::temporary(&origin_url))
}
