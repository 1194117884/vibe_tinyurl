use crate::models::{Tinyurl, User};
use sqlx::MySqlPool;

/// Get table name based on short URL length
fn table_name(length: i32) -> &'static str {
    match length {
        1 => "tinyurl1",
        2 => "tinyurl2",
        3 => "tinyurl3",
        4 => "tinyurl4",
        5 => "tinyurl5",
        6 => "tinyurl6",
        _ => panic!("Invalid length: {}", length),
    }
}

/// Find user by API key
pub async fn find_user_by_api_key(pool: &MySqlPool, api_key: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, api_key, create_time FROM users WHERE api_key = ?"
    )
    .bind(api_key)
    .fetch_optional(pool)
    .await
}

/// Find most recent tinyurl by origin URL for a specific length
pub async fn find_by_origin_url(
    pool: &MySqlPool,
    length: i32,
    origin_url: &str,
) -> Result<Option<Tinyurl>, sqlx::Error> {
    let table = table_name(length);
    let sql = format!(
        "SELECT id, short_uri, origin_url, create_time, user_id FROM {}
         WHERE origin_url = ? ORDER BY id DESC LIMIT 1",
        table
    );

    sqlx::query_as::<_, Tinyurl>(&sql)
        .bind(origin_url)
        .fetch_optional(pool)
        .await
}

/// Find tinyurl by short URI (determine table from URI length)
pub async fn find_by_short_uri(
    pool: &MySqlPool,
    short_uri: &str,
) -> Result<Option<Tinyurl>, sqlx::Error> {
    let length = short_uri.len() as i32;
    if length < 1 || length > 6 {
        return Ok(None);
    }
    let table = table_name(length);
    let sql = format!(
        "SELECT id, short_uri, origin_url, create_time, user_id FROM {} WHERE short_uri = ?",
        table
    );

    sqlx::query_as::<_, Tinyurl>(&sql)
        .bind(short_uri)
        .fetch_optional(pool)
        .await
}

/// Create new tinyurl
pub async fn create_tinyurl(
    pool: &MySqlPool,
    length: i32,
    short_uri: &str,
    origin_url: &str,
    user_id: i64,
) -> Result<Tinyurl, sqlx::Error> {
    let table = table_name(length);
    let sql = format!(
        "INSERT INTO {} (short_uri, origin_url, user_id) VALUES (?, ?, ?)",
        table
    );

    let id = sqlx::query(&sql)
        .bind(short_uri)
        .bind(origin_url)
        .bind(user_id)
        .execute(pool)
        .await?
        .last_insert_id() as i64;

    // Fetch the created record
    let select_sql = format!(
        "SELECT id, short_uri, origin_url, create_time, user_id FROM {} WHERE id = ?",
        table
    );

    sqlx::query_as::<_, Tinyurl>(&select_sql)
        .bind(id)
        .fetch_one(pool)
        .await
}

/// Check if short URI exists
pub async fn short_uri_exists(
    pool: &MySqlPool,
    length: i32,
    short_uri: &str,
) -> Result<bool, sqlx::Error> {
    let table = table_name(length);
    let sql = format!(
        "SELECT COUNT(*) FROM {} WHERE short_uri = ?",
        table
    );

    let count: i64 = sqlx::query_scalar(&sql)
        .bind(short_uri)
        .fetch_one(pool)
        .await?;

    Ok(count > 0)
}
