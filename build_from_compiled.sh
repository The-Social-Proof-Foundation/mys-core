#!/bin/bash
# Script to build Docker images using pre-compiled binaries

set -e  # Exit on error

echo "Building Docker images for MySocial services using pre-compiled binaries..."

# Get the absolute path to the project root
PROJECT_ROOT="$(pwd)"

# Services and the paths to their pre-compiled binaries
SERVICES=("mys-node" "mys-indexer" "mys-edge-proxy" "mys-faucet" "mys-security-watchdog")
BINARY_PATHS=(
    "${PROJECT_ROOT}/target/release/mys-node"
    "${PROJECT_ROOT}/target/release/mys-indexer"
    "${PROJECT_ROOT}/target/release/mys-edge-proxy"
    "${PROJECT_ROOT}/target/release/mys-faucet"
    "${PROJECT_ROOT}/target/release/mys-security-watchdog"
)

# Docker registry
REGISTRY="socialproof"

# Loop through the services
for i in "${!SERVICES[@]}"; do
    service=${SERVICES[$i]}
    binary_path=${BINARY_PATHS[$i]}
    binary_name=${SERVICES[$i]}
    
    # Check if binary exists
    if [ ! -f "$binary_path" ]; then
        echo "ERROR: Binary not found at $binary_path"
        echo "Please make sure the binary is compiled and available at this location."
        exit 1
    fi
    
    echo "===== Processing $service ====="
    echo "Using binary from $binary_path"
    
    echo "Building $service image..."
    docker build -t "$REGISTRY/$service:latest" \
        --build-arg BINARY_PATH="$binary_path" \
        --build-arg BINARY_NAME="$binary_name" \
        -f Dockerfile.simple .
    
    echo "Pushing $service image to registry..."
    docker push "$REGISTRY/$service:latest"
    
    echo "$service completed successfully"
    echo ""
done

echo "All images built and pushed successfully!" 