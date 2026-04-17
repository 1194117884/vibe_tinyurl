use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tinyurl {
    pub id: i64,
    pub short_uri: String,
    pub origin_url: String,
    pub create_time: DateTime<Utc>,
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
    pub len: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CacheRefreshRequest {
    pub short_uri: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUrlResponse {
    pub short_url: String,
}
