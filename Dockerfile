# 1. Build stage
FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

# Ensure OpenSSL headers are available at build time
RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo build --release

# 2. Runtime stage (must support OpenSSL 3)
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL 3 is in libssl3)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/emergency .

EXPOSE 5000
CMD ["./emergency"]
