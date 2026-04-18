FROM rust:1.82 AS builder
WORKDIR /usr/src/tinyurl

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy main to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "" > src/lib.rs
RUN cargo build --release && rm -rf src

# Copy actual source and build
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/tinyurl /usr/local/bin/tinyurl
CMD ["tinyurl"]
