# Build stage
FROM rust:1.75-slim as builder

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the entire project (since we're in a workspace)
COPY . .

# Build the specific binary
RUN cargo build --release --bin mys-price-oracle

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false oracle

# Create data directory with proper permissions
RUN mkdir -p /app/data && chown oracle:oracle /app/data

# Copy the binary from builder stage
COPY --from=builder /app/target/release/mys-price-oracle /usr/local/bin/mys-price-oracle

# Make binary executable
RUN chmod +x /usr/local/bin/mys-price-oracle

# Switch to non-root user
USER oracle

# Set working directory
WORKDIR /app

# Expose health check port (Railway will set $PORT)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:${PORT:-8080}/health || exit 1

# Run the oracle with environment variable configuration
CMD ["mys-price-oracle", "--env"] 