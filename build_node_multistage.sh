#!/bin/bash

set -e

# Set project root
PROJECT_ROOT=$(pwd)
echo "Project root: $PROJECT_ROOT"

# Build node Docker image
SERVICE="mys-node"
echo "Building Docker image for $SERVICE..."

docker build \
  -t socialproof/$SERVICE:latest \
  -f Dockerfile.node.multistage \
  .

echo "Done building image for $SERVICE"

# Push to registry (optional)
read -p "Do you want to push the image to the registry? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
  echo "Pushing $SERVICE image to registry..."
  docker push socialproof/$SERVICE:latest
fi

echo "Multi-stage build process completed." 