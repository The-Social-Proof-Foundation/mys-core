# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.81-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    libpq5 \
    ca-certificates \
    libpq-dev \
    postgresql

# Copy source code
COPY . .

# Build
RUN cargo build --release --bin mys-graphql-rpc

# Production Image
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libjemalloc-dev \
    ca-certificates \
    curl \
    libpq5 \
    libpq-dev \
    postgresql \
    gettext-base

# Use jemalloc as memory allocator
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so

WORKDIR /app
COPY --from=builder /app/target/release/mys-graphql-rpc /usr/local/bin

# Create entrypoint script
RUN echo '#!/bin/sh\n\
echo "=== ENTRYPOINT DEBUG ==="\n\
echo "All environment variables:"\n\
env | sort\n\
echo ""\n\
echo "=== Key Environment Variables ==="\n\
echo "DATABASE_URL: ${DATABASE_URL:-NOT_SET}"\n\
echo "RPC_URL: ${RPC_URL:-NOT_SET}"\n\
echo "PORT: ${PORT:-NOT_SET}"\n\
echo "=============================="\n\
\n\
# Use default values if environment variables are not set\n\
DATABASE_URL_VAL="${DATABASE_URL:-postgres://user:pass@localhost/db}"\n\
RPC_URL_VAL="${RPC_URL:-http://localhost:8080}"\n\
PORT_VAL="${PORT:-8080}"\n\
\n\
echo "Using values:"\n\
echo "DATABASE_URL_VAL: $DATABASE_URL_VAL"\n\
echo "RPC_URL_VAL: $RPC_URL_VAL"\n\
echo "PORT_VAL: $PORT_VAL"\n\
echo ""\n\
\n\
echo "Starting mys-graphql-rpc..."\n\
exec mys-graphql-rpc start-server \\\n\
    --host 0.0.0.0 \\\n\
    --port "$PORT_VAL" \\\n\
    --db-url "$DATABASE_URL_VAL" \\\n\
    --node-rpc-url "$RPC_URL_VAL"' > /usr/local/bin/entrypoint.sh && \
    chmod +x /usr/local/bin/entrypoint.sh

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Use the entrypoint script
ENTRYPOINT /usr/local/bin/entrypoint.sh 