#!/bin/bash
set -e

# Validate required environment variables
if [ -z "$DATABASE_URL" ]; then
    echo "ERROR: DATABASE_URL environment variable is not set"
    exit 1
fi

if [ -z "$RPC_URL" ]; then
    echo "ERROR: RPC_URL environment variable is not set"
    exit 1
fi

if [ -z "$PORT" ]; then
    echo "ERROR: PORT environment variable is not set"
    exit 1
fi

echo "Starting mys-graphql-rpc with:"
echo "  Database URL: $DATABASE_URL"
echo "  RPC URL: $RPC_URL"
echo "  Port: $PORT"

# Start the GraphQL RPC server
exec /app/mys-graphql-rpc start-server \
    --host 0.0.0.0 \
    --port "$PORT" \
    --db-url "$DATABASE_URL" \
    --node-rpc-url "$RPC_URL" 