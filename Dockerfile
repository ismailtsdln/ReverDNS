# Build stage
FROM rust:1.70 as builder

WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY benches ./benches
COPY tests ./tests

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/reverdns /usr/local/bin/reverdns

# Create non-root user
RUN useradd -m -u 1000 reverdns
USER reverdns

# Set entrypoint
ENTRYPOINT ["reverdns"]
CMD ["--help"]
