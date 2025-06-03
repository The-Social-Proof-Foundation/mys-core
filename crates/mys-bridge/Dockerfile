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

# Copy config file
COPY crates/mys-bridge/bridge-config.yaml /app/bridge-config.yaml

# Build the project
RUN cargo build --release --bin mys-bridge

# Expose ports
EXPOSE 8080 9090

# Default command - Railway can override this
CMD ["cargo", "run", "--release", "--bin", "mys-bridge", "--", "--config-path", "bridge-config.yaml"] 