use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Redis(redis::RedisError),
    NotFound(String),
    Unauthorized(String),
    BadRequest(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Redis(e) => write!(f, "Redis error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "500", "Database error"),
            AppError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "500", "Cache error"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "8416002", "链接已失效！"),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "8416001", "用户授权异常！"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "501", "参数错误"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "500", "服务器错误"),
        };

        let body = Json(json!({
            "code": code,
            "message": message,
            "data": null
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Record not found".to_string()),
            _ => AppError::Database(err),
        }
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::Redis(err)
    }
}
