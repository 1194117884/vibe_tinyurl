use crate::db::queries;
use crate::error::AppError;
use crate::models::Tinyurl;
use crate::services::TinyurlManager;
use redis::{AsyncCommands, Client};
use sqlx::MySqlPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct TinyurlService {
    pool: MySqlPool,
    redis: Arc<Client>,
    manager: TinyurlManager,
}

impl TinyurlService {
    pub fn new(pool: MySqlPool, redis: Client) -> Self {
        Self {
            pool,
            redis: Arc::new(redis),
            manager: TinyurlManager::new(),
        }
    }

    /// Create new short URL
    pub async fn create(
        &self,
        api_key: &str,
        url: &str,
        length: i32,
    ) -> Result<String, AppError> {
        tracing::info!("Creating {}-char short URL for: {}", length, url);

        // Validate length
        if length < 1 || length > 6 {
            return Err(AppError::BadRequest("短链长度不支持！".to_string()));
        }

        // Verify API key
        let user = queries::find_user_by_api_key(&self.pool, api_key)
            .await?
            .ok_or_else(|| AppError::Unauthorized("用户授权异常！".to_string()))?;

        // Find existing or create new
        let tinyurl = self.manager.find_or_create(&self.pool, length, user.id, url).await?;

        // Cache the result
        self.cache_tinyurl(&tinyurl).await?;

        // Return short URI (not full URL)
        Ok(tinyurl.short_uri.clone())
    }

    /// Visit by short URI (with cache)
    pub async fn visit_by_uri(&self, short_uri: &str) -> Result<String, AppError> {
        tracing::info!("Resolving short URI: {}", short_uri);

        // Try cache first
        let cache_key = format!("dl:{}", short_uri);
        let mut conn = self.redis.get_multiplexed_async_connection().await?;

        if let Ok(cached) = conn.get::<_, String>(&cache_key).await {
            tracing::debug!("Cache hit for: {}", short_uri);
            return Ok(cached);
        }

        // Cache miss - query database
        tracing::debug!("Cache miss for: {}", short_uri);
        let tinyurl = self.manager
            .find_by_short_uri(&self.pool, short_uri)
            .await?
            .ok_or_else(|| AppError::NotFound("链接已失效！".to_string()))?;

        // Update cache
        self.cache_tinyurl(&tinyurl).await?;

        Ok(tinyurl.origin_url)
    }

    /// Refresh cache for a short URI
    pub async fn refresh(&self, api_key: &str, short_uri: &str) -> Result<String, AppError> {
        // Verify API key
        let _user = queries::find_user_by_api_key(&self.pool, api_key)
            .await?
            .ok_or_else(|| AppError::Unauthorized("用户授权异常！".to_string()))?;

        // Find in database
        let tinyurl = self.manager
            .find_by_short_uri(&self.pool, short_uri)
            .await?
            .ok_or_else(|| AppError::NotFound("链接已失效！".to_string()))?;

        // Update cache
        self.cache_tinyurl(&tinyurl).await?;

        Ok(tinyurl.short_uri.clone())
    }

    /// Cache tinyurl in Redis
    async fn cache_tinyurl(&self, tinyurl: &Tinyurl) -> Result<(), AppError> {
        let cache_key = format!("dl:{}", tinyurl.short_uri);
        let mut conn = self.redis.get_multiplexed_async_connection().await?;

        // Store just the origin URL for simplicity
        conn.set::<_, _, ()>(&cache_key, &tinyurl.origin_url).await?;

        Ok(())
    }
}
