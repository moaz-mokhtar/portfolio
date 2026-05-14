# --- Build Stage ---
FROM rust:1-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  curl \
  && rm -rf /var/lib/apt/lists/*

# Install wasm32 target and cargo-leptos
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked

WORKDIR /app
COPY . .

# Build the application
# --bin-features ssr --lib-features hydrate are defaults in Cargo.toml
RUN cargo leptos build --release

# --- Runtime Stage ---
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL for potential HTTPS/APIs)
RUN apt-get update && apt-get install -y \
  libssl-dev \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary
# Site-root is target/site per Cargo.toml
COPY --from:builder /app/target/release/portfolio /app/portfolio
COPY --from:builder /app/target/site /app/target/site

# Set environment variables
ENV LEPTOS_SITE_ROOT=target/site
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080
ENV LEPTOS_ENV=PROD

EXPOSE 8080

CMD ["/app/portfolio"]
