use crate::db::queries;
use crate::error::AppError;
use crate::models::Tinyurl;
use crate::utils::ShortUrlUtil;
use sqlx::MySqlPool;

pub struct TinyurlManager;

impl TinyurlManager {
    pub fn new() -> Self {
        Self
    }

    /// Find existing URL or create new one
    pub async fn find_or_create(
        &self,
        pool: &MySqlPool,
        length: i32,
        user_id: i64,
        origin_url: &str,
    ) -> Result<Tinyurl, AppError> {
        // First check if URL already exists
        if let Some(existing) = queries::find_by_origin_url(pool, length, origin_url).await? {
            return Ok(existing);
        }

        // Generate unique short URI
        let short_uri = self.generate_unique_short_uri(pool, length).await?;

        // Create new record
        let tinyurl = queries::create_tinyurl(pool, length, &short_uri, origin_url, user_id).await?;
        Ok(tinyurl)
    }

    /// Find by short URI (any length 1-6)
    pub async fn find_by_short_uri(
        &self,
        pool: &MySqlPool,
        short_uri: &str,
    ) -> Result<Option<Tinyurl>, AppError> {
        queries::find_by_short_uri(pool, short_uri).await.map_err(AppError::from)
    }

    /// Generate a unique short URI that doesn't exist in database
    async fn generate_unique_short_uri(
        &self,
        pool: &MySqlPool,
        length: i32,
    ) -> Result<String, AppError> {
        let util = ShortUrlUtil::new(length as usize);
        let max_attempts = 100;

        for _ in 0..max_attempts {
            let candidate = util.random_str();
            let exists = queries::short_uri_exists(pool, length, &candidate).await?;
            if !exists {
                return Ok(candidate);
            }
        }

        Err(AppError::BadRequest("此长度下的短链，没有可用额度！".to_string()))
    }
}
