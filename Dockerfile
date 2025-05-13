# Build stage (using newer Rust version)
FROM rust:1.77-bookworm as builder

WORKDIR /usr/src/hospital

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy dependency files first
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy actual source code
COPY src src/
COPY migrations migrations/
COPY .env ./

# Touch main.rs to force rebuild
RUN touch src/main.rs

# Build actual application
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy binary and environment file
COPY --from=builder /usr/src/hospital/target/release/hospital /usr/local/bin/
COPY --from=builder /usr/src/hospital/.env .

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=5000
EXPOSE 5000

ENTRYPOINT ["hospital"]