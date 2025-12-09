# MySo Bridge Dockerfile for Railway Deployment
FROM rust:1.81-bullseye

WORKDIR /work

# Install system dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace
COPY . .

# Copy config files
COPY crates/mys-bridge/bridge-config.yaml /app/bridge-config.yaml
COPY crates/mys-bridge/bridge-config-validator-1.yaml /app/bridge-config-validator-1.yaml
COPY crates/mys-bridge/bridge-config-validator-2.yaml /app/bridge-config-validator-2.yaml
COPY crates/mys-bridge/bridge-config-validator-3.yaml /app/bridge-config-validator-3.yaml

# Build the project
RUN cargo build --release --bin mys-bridge

# Expose ports
EXPOSE 8080 8081 8082 9090 9091 9092

# Default command - Railway will override this via railway.toml
CMD ["sleep", "infinity"] 

