# Build stage
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /usr/src/app

# Environment variables for edition 2024 support
ENV RUSTC_BOOTSTRAP=1
ENV RUSTFLAGS="--cfg procmacro2_semver_exempt"

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a temporary dummy project to compile dependencies
RUN mkdir -p src && \
    echo 'fn main() { println!("Dummy implementation"); }' > src/main.rs && \
    RUSTC_BOOTSTRAP=1 cargo build --release && \
    rm -rf src

# Copy the actual code
COPY . .

# Build the application
RUN RUSTC_BOOTSTRAP=1 cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m appuser
USER appuser
WORKDIR /home/appuser

# Copy binary and config from builder
COPY --from=builder /usr/src/app/target/release/emergency .
COPY --from=builder /usr/src/app/.env .

# Expose the application port
EXPOSE 5000

# Run the application
CMD ["./emergency"]