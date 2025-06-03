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

# Copy migration files
COPY --from=builder /app/crates/mys-bridge/migrations ./migrations

# Create a simple startup script
COPY <<EOF start.sh
#!/bin/bash
set -e

echo "Starting MySo Bridge..."

# Create bridge authority key file from env var
if [ -n "\$BRIDGE_AUTHORITY_KEY" ]; then
    echo "\$BRIDGE_AUTHORITY_KEY" > /tmp/bridge_key
else
    echo "Error: BRIDGE_AUTHORITY_KEY required"
    exit 1
fi

# Create basic config file
cat > /tmp/config.yaml <<CONFIG
server-listen-port: \${BRIDGE_NODE_PORT:-8080}
metrics-port: \${BRIDGE_METRICS_PORT:-9090}
bridge-authority-key-path: /tmp/bridge_key
run-client: false
approved-governance-actions: []

mys:
  mys-rpc-url: "\${MYS_RPC_URL:-https://fullnode.mainnet.mysocial.network:443}"
  mys-bridge-chain-id: \${TARGET_CHAIN_ID:-1}

eth:
  eth-rpc-url: "\${ETH_RPC_URL:-https://sepolia.base.org}"
  eth-bridge-proxy-address: "\${BRIDGE_CONTRACT_ADDRESS:-0x196D0E3f7b50F29531d1bd006AB3D4c2CB731DC6}"
  eth-bridge-chain-id: \${SOURCE_CHAIN_ID:-84532}
  eth-contracts-start-block-fallback: 0

metrics:
  push-url: "https://bridge-metrics.testnet.mysocial.network:8443/publish/metrics"
CONFIG

echo "Starting bridge node..."
exec ./mys-bridge --config-path /tmp/config.yaml
EOF

# Make script executable
RUN chmod +x start.sh mys-bridge

# Change ownership to app user
RUN chown -R app:app /app

# Switch to app user
USER app

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose ports
EXPOSE 8080 9090

# Set environment variables
ENV RUST_LOG=info

# Start the application
CMD ["./start.sh"] 