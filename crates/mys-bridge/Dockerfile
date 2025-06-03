# MySo Bridge Dockerfile for Railway Deployment
FROM rust:1.85-slim-bullseye as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy all source files
COPY . .

# Build the application
RUN cargo build --release --bin mys-bridge
RUN cargo build --release --bin migrate

# Runtime stage
FROM debian:bullseye-slim

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

# Copy the built binaries
COPY --from=builder /app/target/release/mys-bridge ./mys-bridge
COPY --from=builder /app/target/release/migrate ./migrate

# Copy migration files
COPY --from=builder /app/migrations ./migrations

# Create migration script that uses our custom migrate binary
RUN cat > run-migrations.sh << 'EOF'
#!/bin/bash
set -e

echo "🔄 Running MySo Bridge database migrations..."

# Parse DATABASE_URL to get connection details for pg_isready
if [[ $DATABASE_URL =~ postgres://([^:]+):([^@]+)@([^:]+):([^/]+)/(.+) ]]; then
    DB_USER="${BASH_REMATCH[1]}"
    DB_HOST="${BASH_REMATCH[3]}"
    DB_PORT="${BASH_REMATCH[4]}"
else
    echo "⚠️  Could not parse DATABASE_URL for health check"
    DB_HOST="localhost"
    DB_PORT="5432"
    DB_USER="postgres"
fi

# Wait for database to be ready
echo "⏳ Waiting for database at $DB_HOST:$DB_PORT..."
until pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" 2>/dev/null; do
    echo "⏳ Database not ready, waiting..."
    sleep 2
done

echo "✅ Database is ready"

# Run migrations using our custom migrate binary
if [ -d "migrations" ]; then
    echo "🚀 Running migrations..."
    ./migrate
    echo "✅ Migrations completed successfully"
else
    echo "⚠️  No migrations directory found"
fi

echo "🎯 Starting MySo Bridge Node..."
exec ./mys-bridge
EOF

# Make scripts executable
RUN chmod +x run-migrations.sh mys-bridge migrate

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