#!/bin/bash
set -e

echo "Setting up MySocial full node server..."

# Update system packages
apt-get update
apt-get upgrade -y

# Install required packages
apt-get install -y curl wget git build-essential pkg-config libssl-dev

# Install Docker
echo "Installing Docker..."
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
rm get-docker.sh

# Install Docker Compose
echo "Installing Docker Compose..."
curl -SL https://github.com/docker/compose/releases/download/v2.20.3/docker-compose-linux-x86_64 -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# Install Rust
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Create directories
mkdir -p /opt/mys/config
mkdir -p /opt/mys/db
mkdir -p /opt/mys/data/postgres

# Create docker-compose file
cat > docker-compose.yaml << 'EOF'
version: "3"

services:
  fullnode:
    container_name: fullnode
    image: socialproof/mys-node:${MYS_SHA:-latest}
    ports:
      - "8080:8080"
      - "8084:8084/udp"
      - "9000:9000"
      - "9184:9184"
    volumes:
      - ./fullnode.yaml:/opt/mys/config/fullnode.yaml:ro
      - ./genesis.blob:/opt/mys/config/genesis.blob:ro
      - ./data/fullnode:/opt/mys/db:rw
    command: ["/opt/mys/bin/mys-node", "--config-path", "/opt/mys/config/fullnode.yaml"]
    restart: on-failure
    logging:
      driver: "json-file"
      options:
        max-file: "10"
        max-size: "5g"

  postgres:
    container_name: postgres
    image: postgres:15-alpine
    ports:
      - "5432:5432"
    volumes:
      - ./data/postgres:/var/lib/postgresql/data
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=postgres
      - POSTGRES_DB=mysindexer
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: on-failure

  indexer:
    container_name: indexer
    image: mysten/mys-indexer:${MYS_SHA:-latest}
    depends_on:
      - postgres
      - fullnode
    ports:
      - "8081:8081"
    environment:
      - POSTGRES_URL=postgres://postgres:postgres@postgres:5432/mysindexer
      - JSON_RPC_URL=http://fullnode:9000
      - RUST_LOG=info
    restart: on-failure
    logging:
      driver: "json-file"
      options:
        max-file: "10"
        max-size: "5g"

  edge-proxy:
    container_name: edge-proxy
    image: mysten/mys-tools:${MYS_SHA:-latest}
    ports:
      - "8082:8082"
      - "9185:9184"
    volumes:
      - ./edge-proxy-config.yaml:/config/proxy.yaml:ro
    command: ["/opt/mys/bin/mys-edge-proxy", "--config", "/config/proxy.yaml"]
    restart: on-failure
    depends_on:
      - fullnode
      - indexer
EOF

# Create fullnode config
cat > fullnode.yaml << 'EOF'
db-path: /opt/mys/db/fullnode_db
network-address: /ip4/0.0.0.0/tcp/8080/http
metrics-address: 0.0.0.0:9184
admin-interface-port: 1337
json-rpc-address: 0.0.0.0:9000
websocket-address: 0.0.0.0:9001
p2p-config:
  listen-address: /ip4/0.0.0.0/udp/8084
genesis:
  genesis-file-location: /opt/mys/config/genesis.blob
authority-store-pruning-config:
  num-latest-epoch-dbs-to-retain: 3
  epoch-db-pruning-period-secs: 3600
  num-epochs-to-retain: 1
  max-checkpoints-in-archive: 10
  max-transactions-in-archive: 1000
  num-epochs-to-retain-for-checkpoints: 0
EOF

# Create edge proxy config
cat > edge-proxy-config.yaml << 'EOF'
---
listen-address: "0.0.0.0:8082"
metrics-address: "0.0.0.0:9184"

execution-peer:
  address: "http://fullnode:9000"

read-peer:
  address: "http://indexer:8081"

logging:
  read-request-sample-rate: 1.0
EOF

echo "Setup complete! You now need to:"
echo "1. Place your genesis.blob file in the current directory"
echo "2. Run 'docker-compose up -d' to start all services"
echo "3. Monitor logs with 'docker-compose logs -f'" 