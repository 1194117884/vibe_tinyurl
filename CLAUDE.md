# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A high-performance URL shortener service written in Rust using Axum. Uses Base62 encoding for short URLs with sharded MySQL tables and Redis caching.

## Common Commands

```bash
# Build the project
cargo build

# Run the server (requires MySQL and Redis)
cargo run

# Run all tests
cargo test

# Run a specific test
cargo test test_short_url_util_base62_roundtrip

# Run with debug logging
RUST_LOG=debug cargo run

# Build release binary
cargo build --release

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Architecture

### Layer Structure

The codebase follows a layered architecture:

1. **Routes** (`src/routes/`): HTTP handlers organized by API type
   - `inner_api.rs`: Internal APIs requiring API key auth (create, cache refresh)
   - `open_api.rs`: Public APIs (visit/resolve short URL)
   - `redirect.rs`: Direct short URL redirects (`/{shortUri}`)

2. **Services** (`src/services/`): Business logic layer
   - `tinyurl_service.rs`: Main service coordinating DB and cache operations
   - `tinyurl_manager.rs`: Database operations for sharded tables

3. **Database** (`src/db/`): Data access layer
   - `pool.rs`: MySQL connection pool setup
   - `queries.rs`: SQLx queries with compile-time checking

4. **Models** (`src/models/`): Data structures
   - `tinyurl.rs`: URL mapping entity
   - `user.rs`: API user with key authentication

5. **Utils** (`src/utils/`): Utilities
   - `short_url.rs`: Base62 encoding/decoding

### Key Design Patterns

- **Sharded Tables**: URLs stored in `tinyurl1` through `tinyurl6` based on short URL length
- **Cache Key Format**: Redis keys use format `dl:{shortUri}` (e.g., `dl:abc123`)
- **Error Codes**: Custom error codes maintained for Java compatibility:
  - `0`: Success
  - `501`: Bad request
  - `8416001`: Unauthorized (bad API key)
  - `8416002`: Not found
  - `8416004`: URL length not supported (must be 1-6)
  - `8417005`: No available short URLs

### State Management

Routes share state via `AppState` struct containing:
- `pool`: MySQL connection pool
- `redis`: Redis client (Arc-wrapped for Clone)
- `tinyurl_service`: Business logic service

State is passed to handlers via Axum's `State` extractor.

## Environment Setup

Copy `.env.example` to `.env` and configure:
- `DATABASE_URL`: MySQL connection string
- `REDIS_URL`: Redis connection string
- `TINYURL_FORMAT`: Short URL format with `%s` placeholder
- `SERVER_PORT`: HTTP server port (default: 3000)
- `RUST_LOG`: Log level (default: info)

## Database Schema

Requires 6 sharded tables (`tinyurl1` to `tinyurl6`) and a `users` table. See README.md for full schema details.
