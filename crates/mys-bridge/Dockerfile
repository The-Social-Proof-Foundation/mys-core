# MySo Bridge Dockerfile for Railway Deployment
FROM rust:1.85-slim-bullseye as builder

# Install system dependencies needed for build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the entire workspace (Railway needs complete context)
COPY . .

# Build only the mys-bridge binary (faster than building entire workspace)
RUN cargo build --release -p mys-bridge --bin mys-bridge

# Install diesel CLI for migrations
RUN cargo install diesel_cli --no-default-features --features postgres --root /usr/local

# Runtime stage
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
COPY --from=builder /app/target/release/mys-bridge ./mys-bridge

# Copy diesel CLI
COPY --from=builder /usr/local/bin/diesel /usr/local/bin/diesel

# Copy migration files
COPY --from=builder /app/crates/mys-bridge/migrations ./migrations

# Create diesel configuration
RUN echo '[print_schema]\nfile = "src/schema.rs"' > diesel.toml

# Create startup script using multiple RUN commands to avoid heredoc parsing issues
RUN echo '#!/bin/bash' > start.sh
RUN echo 'set -e' >> start.sh
RUN echo '' >> start.sh
RUN echo 'echo "🔄 Starting MySo Bridge..."' >> start.sh
RUN echo '' >> start.sh
RUN echo '# Generate bridge authority key file from environment variable' >> start.sh
RUN echo 'if [ -n "$BRIDGE_AUTHORITY_KEY" ]; then' >> start.sh
RUN echo '    echo "$BRIDGE_AUTHORITY_KEY" > /tmp/bridge_authority_key' >> start.sh
RUN echo '    BRIDGE_AUTHORITY_KEY_PATH="/tmp/bridge_authority_key"' >> start.sh
RUN echo 'else' >> start.sh
RUN echo '    echo "❌ BRIDGE_AUTHORITY_KEY environment variable is required"' >> start.sh
RUN echo '    exit 1' >> start.sh
RUN echo 'fi' >> start.sh
RUN echo '' >> start.sh
RUN echo '# Set default values for configuration' >> start.sh
RUN echo 'MYS_RPC_URL="${MYS_RPC_URL:-https://fullnode.mainnet.mysocial.network:443}"' >> start.sh
RUN echo 'ETH_RPC_URL="${ETH_RPC_URL:-https://sepolia.base.org}"' >> start.sh
RUN echo 'BRIDGE_NODE_PORT="${BRIDGE_NODE_PORT:-8080}"' >> start.sh
RUN echo 'BRIDGE_METRICS_PORT="${BRIDGE_METRICS_PORT:-9090}"' >> start.sh
RUN echo 'BRIDGE_CONTRACT_ADDRESS="${BRIDGE_CONTRACT_ADDRESS:-0x196D0E3f7b50F29531d1bd006AB3D4c2CB731DC6}"' >> start.sh
RUN echo 'SOURCE_CHAIN_ID="${SOURCE_CHAIN_ID:-84532}"' >> start.sh
RUN echo 'TARGET_CHAIN_ID="${TARGET_CHAIN_ID:-1}"' >> start.sh
RUN echo 'RUN_CLIENT="${RUN_CLIENT:-false}"' >> start.sh
RUN echo '' >> start.sh
RUN echo '# Generate bridge node configuration file' >> start.sh
RUN echo 'cat > /tmp/bridge_config.yaml << CONFIG_EOF' >> start.sh
RUN echo 'server-listen-port: ${BRIDGE_NODE_PORT}' >> start.sh
RUN echo 'metrics-port: ${BRIDGE_METRICS_PORT}' >> start.sh
RUN echo 'bridge-authority-key-path: "${BRIDGE_AUTHORITY_KEY_PATH}"' >> start.sh
RUN echo 'run-client: ${RUN_CLIENT}' >> start.sh
RUN echo 'approved-governance-actions: []' >> start.sh
RUN echo '' >> start.sh
RUN echo 'mys:' >> start.sh
RUN echo '  mys-rpc-url: "${MYS_RPC_URL}"' >> start.sh
RUN echo '  mys-bridge-chain-id: ${TARGET_CHAIN_ID}' >> start.sh
RUN echo '' >> start.sh
RUN echo 'eth:' >> start.sh
RUN echo '  eth-rpc-url: "${ETH_RPC_URL}"' >> start.sh
RUN echo '  eth-bridge-proxy-address: "${BRIDGE_CONTRACT_ADDRESS}"' >> start.sh
RUN echo '  eth-bridge-chain-id: ${SOURCE_CHAIN_ID}' >> start.sh
RUN echo '  eth-contracts-start-block-fallback: 0' >> start.sh
RUN echo '' >> start.sh
RUN echo 'metrics:' >> start.sh
RUN echo '  push-url: "https://bridge-metrics.testnet.mysocial.network:8443/publish/metrics"' >> start.sh
RUN echo 'CONFIG_EOF' >> start.sh
RUN echo '' >> start.sh
RUN echo 'echo "✅ Bridge configuration generated"' >> start.sh
RUN echo '' >> start.sh
RUN echo '# Wait for database to be ready if DATABASE_URL is provided' >> start.sh
RUN echo 'if [ -n "$DATABASE_URL" ]; then' >> start.sh
RUN echo '    echo "📊 Checking database connection..."' >> start.sh
RUN echo '    until pg_isready -d "$DATABASE_URL"; do' >> start.sh
RUN echo '        echo "⏳ Waiting for database..."' >> start.sh
RUN echo '        sleep 2' >> start.sh
RUN echo '    done' >> start.sh
RUN echo '    echo "✅ Database is ready"' >> start.sh
RUN echo '' >> start.sh
RUN echo '    # Run migrations if they exist' >> start.sh
RUN echo '    if [ -d "migrations" ]; then' >> start.sh
RUN echo '        echo "🚀 Running database migrations..."' >> start.sh
RUN echo '        diesel migration run --database-url="$DATABASE_URL" || echo "⚠️ Migrations failed or already applied"' >> start.sh
RUN echo '    fi' >> start.sh
RUN echo 'else' >> start.sh
RUN echo '    echo "⚠️ No DATABASE_URL provided, skipping database setup"' >> start.sh
RUN echo 'fi' >> start.sh
RUN echo '' >> start.sh
RUN echo 'echo "🎯 Starting MySo Bridge Node..."' >> start.sh
RUN echo 'echo "📝 Using config file: /tmp/bridge_config.yaml"' >> start.sh
RUN echo '' >> start.sh
RUN echo '# Start the bridge node with the generated config' >> start.sh
RUN echo 'exec ./mys-bridge --config-path /tmp/bridge_config.yaml "$@"' >> start.sh

# Make scripts executable
RUN chmod +x start.sh mys-bridge

# Change ownership to app user
RUN chown -R app:app /app

# Switch to app user
USER app

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:${BRIDGE_NODE_PORT:-8080}/health || exit 1

# Expose ports
EXPOSE 8080 9090

# Set environment variables
ENV RUST_LOG=info \
    BRIDGE_NODE_PORT=8080 \
    BRIDGE_METRICS_PORT=9090

# Start the application
CMD ["./start.sh"] 