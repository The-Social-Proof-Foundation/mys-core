#!/bin/bash
# Script to build and push all required Docker images

set -e  # Exit on error

echo "Building and pushing Docker images for MySocial services..."
echo "NOTE: Building from source takes a significant amount of time (30+ minutes per image)"
echo "This is because we're building the Rust project from source inside the Docker container"

# Services and their Dockerfile names (using simple arrays instead of associative arrays)
SERVICES=("mys-node" "mys-indexer" "mys-edge-proxy" "mys-faucet" "mys-security-watchdog")
DOCKERFILE_SUFFIXES=("node" "indexer" "edge-proxy" "faucet" "security-watchdog")

# Docker registry
REGISTRY="socialproof"

# Loop through the services
for i in "${!SERVICES[@]}"; do
    service=${SERVICES[$i]}
    dockerfile_suffix=${DOCKERFILE_SUFFIXES[$i]}
    
    echo "===== Processing $service ====="
    echo "Using Dockerfile.$dockerfile_suffix"
    
    echo "Building $service image..."
    echo "This will take a long time as it builds the Rust project from source"
    docker build -t "$REGISTRY/$service:latest" -f "Dockerfile.$dockerfile_suffix" .
    
    echo "Pushing $service image to registry..."
    docker push "$REGISTRY/$service:latest"
    
    echo "$service completed successfully"
    echo ""
done

echo "All images built and pushed successfully!" 