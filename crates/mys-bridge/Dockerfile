# MySo Bridge Dockerfile for Railway Deployment
# Multi-stage build for optimal Docker image size

# Stage 1: Build dependencies
FROM rust:1.85-slim-bullseye as deps-builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy only Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY crates/mys-bridge/Cargo.toml ./crates/mys-bridge/

# Create dummy source files to build dependencies
RUN mkdir -p crates/mys-bridge/src && \
    echo "fn main() {}" > crates/mys-bridge/src/main.rs && \
    echo "pub fn lib() {}" > crates/mys-bridge/src/lib.rs

# Build dependencies only
RUN cargo build --release --bin mys-bridge
RUN rm -rf crates/mys-bridge/src

# Stage 2: Build application
FROM deps-builder as app-builder

# Copy workspace configuration
COPY rust-toolchain.toml ./
COPY Cargo.toml Cargo.lock ./

# Copy all necessary crates for dependencies
COPY crates/ ./crates/

# Copy MySo Bridge source
COPY crates/mys-bridge/ ./crates/mys-bridge/

# Build the application
RUN cargo build --release --bin mys-bridge

# Stage 3: Create migration runner
FROM deps-builder as migration-builder

# Install diesel CLI for migrations
RUN cargo install diesel_cli --no-default-features --features postgres

# Stage 4: Runtime image
FROM debian:bullseye-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    postgresql-client \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd --create-home --shell /bin/bash app

# Set working directory
WORKDIR /app

# Copy the built binary
COPY --from=app-builder /app/target/release/mys-bridge ./mys-bridge

# Copy migration files
COPY --from=app-builder /app/crates/mys-bridge/migrations ./migrations

# Copy diesel CLI for migrations
COPY --from=migration-builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Create diesel configuration
RUN echo '[print_schema]\nfile = "src/schema.rs"' > diesel.toml

# Create migration script
COPY <<EOF ./run-migrations.sh
#!/bin/bash
set -e

echo "🔄 Running MySo Bridge database migrations..."

# Wait for database to be ready
until pg_isready -h \${DATABASE_HOST:-localhost} -p \${DATABASE_PORT:-5432} -U \${DATABASE_USER:-postgres}; do
    echo "⏳ Waiting for database to be ready..."
    sleep 2
done

echo "✅ Database is ready"

# Run migrations
if [ -d "migrations" ]; then
    echo "🚀 Running migrations..."
    diesel migration run --database-url="\${DATABASE_URL}"
    echo "✅ Migrations completed successfully"
else
    echo "⚠️  No migrations directory found"
fi

echo "🎯 Starting MySo Bridge Node..."
exec ./mys-bridge
EOF

# Make scripts executable
RUN chmod +x ./run-migrations.sh ./mys-bridge

# Change ownership to app user
RUN chown -R app:app /app

# Switch to app user
USER app

# Health check endpoint
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${BRIDGE_NODE_PORT:-8080}/health || exit 1

# Expose ports
EXPOSE 8080 9090

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=""
ENV BRIDGE_NODE_PORT=8080
ENV BRIDGE_METRICS_PORT=9090

# Start the application with migrations
CMD ["./run-migrations.sh"] 