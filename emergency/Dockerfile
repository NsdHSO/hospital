# Use official Rust image for building
FROM rust:1.73-alpine as builder

# Create app directory
WORKDIR /usr/src/hospital

# Copy dependency specifications
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies (this step is cached unless Cargo.toml/Cargo.lock change)
RUN cargo build --release

# Now copy actual source code
COPY . .

# Build actual application (overwrite the dummy build)
RUN cargo build --release

# Final stage using small Alpine image
FROM alpine:3.18

# Install runtime dependencies (if needed)
RUN apk add --no-cache libgcc

# Copy compiled binary from builder
COPY --from=builder /usr/src/hospital/target/release/hospital /usr/local/bin/

# Install CA certificates (needed for HTTPS requests if your app makes any)
RUN apk add --no-cache ca-certificates

# Set entrypoint
ENTRYPOINT ["hospital"]