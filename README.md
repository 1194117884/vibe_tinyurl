# Vibe TinyURL

A high-performance URL shortener service written in Rust, ported from the Java Spring Boot implementation.

## Features

- Base62 encoding for short URLs (1-6 characters)
- Sharded database tables by URL length for scalability
- Redis caching for fast redirects
- RESTful API with API key authentication
- Axum web framework with async/await

## Quick Start

```bash
# Install dependencies (Rust, MySQL, Redis)
# Copy environment file
cp .env.example .env
# Edit .env with your database and Redis credentials

# Run the server
cargo run
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| DATABASE_URL | MySQL connection string | - |
| REDIS_URL | Redis connection string | - |
| TINYURL_FORMAT | Short URL format | `https://t.cn/%s` |
| SERVER_PORT | HTTP server port | 3000 |
| RUST_LOG | Log level | info |

## API Endpoints

### Create Short URL (Internal)
```bash
POST /api/inner/tinyurl/create
Header: apiKey: your_api_key
Body: {"url": "https://example.com", "len": 6}

Response: {"code": "0", "message": "成功", "data": "shortUri"}
```

### Visit Short URL (Public)
```bash
GET /api/open/tinyurl/visit/{shortUri}

Response: {"code": "0", "message": "成功", "data": "https://example.com"}
Response Header: origin-url: https://example.com
```

### Redirect
```bash
GET /{shortUri}
# Returns 302 redirect to original URL
```

### Cache Refresh (Internal)
```bash
POST /api/inner/tinyurl/cache/refresh
Header: apiKey: your_api_key
Body: {"short_uri": "abc123"}

Response: {"code": "0", "message": "成功", "data": "abc123"}
```

## Database Schema

The service uses 6 sharded tables based on short URL length:
- `tinyurl1` - 1 character URLs
- `tinyurl2` - 2 character URLs
- ...
- `tinyurl6` - 6 character URLs

Each table has: `id`, `short_uri`, `origin_url`, `create_time`, `user_id`

Also requires a `users` table with: `id`, `name`, `api_key`, `created_at`

## Architecture

- **Axum**: Web framework for HTTP routing
- **SQLx**: Compile-time checked SQL queries with MySQL
- **Redis**: Caching layer for fast lookups
- **Tokio**: Async runtime

## Project Structure

```
src/
├── main.rs           # Application entry point
├── config.rs         # Environment configuration
├── error.rs          # Error handling
├── lib.rs            # Library exports
├── models/           # Data models (User, Tinyurl)
├── db/               # Database pool and queries
├── services/         # Business logic
├── routes/           # HTTP handlers
├── utils/            # Base62 encoding
└── middleware/       # Authentication
```

## Testing

```bash
# Run unit and integration tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Build release binary
cargo build --release
```

## Error Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 501 | Bad request / Invalid parameters |
| 8416001 | User authorization failed (invalid API key) |
| 8416002 | Short URL not found |
| 8416004 | URL length not supported (must be 1-6) |
| 8417005 | No available short URLs for this length |

## Migration from Java

This Rust implementation maintains compatibility with the Java Spring Boot version:
- Same base62 character set for URL generation
- Same database schema for sharded tables
- Same Redis cache key format (`dl:{shortUri}`)
- Same error codes and response format
