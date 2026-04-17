use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct ApiKey(pub String);

/// Extract API key from header
pub async fn extract_api_key(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // API key extraction is optional here
    // Individual handlers will validate
    let _api_key = request
        .headers()
        .get("apiKey")
        .and_then(|v| v.to_str().ok())
        .map(|s| ApiKey(s.to_string()));

    // Store in request extensions if needed
    // For now, handlers extract directly from headers

    let response = next.run(request).await;
    Ok(response)
}
