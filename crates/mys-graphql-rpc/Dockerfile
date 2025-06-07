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
echo "=== Environment Variables ==="\n\
echo "DATABASE_URL: ${DATABASE_URL}"\n\
echo "RPC_URL: ${RPC_URL}"\n\
echo "PORT: ${PORT:-8080}"\n\
echo "=============================="\n\
\n\
if [ -z "$DATABASE_URL" ]; then\n\
  echo "ERROR: DATABASE_URL is not set"\n\
  exit 1\n\
fi\n\
\n\
if [ -z "$RPC_URL" ]; then\n\
  echo "ERROR: RPC_URL is not set"\n\
  exit 1\n\
fi\n\
\n\
exec mys-graphql-rpc start-server \\\n\
    --host 0.0.0.0 \\\n\
    --port ${PORT:-8080} \\\n\
    --db-url "$DATABASE_URL" \\\n\
    --node-rpc-url "$RPC_URL"' > /usr/local/bin/entrypoint.sh && \
    chmod +x /usr/local/bin/entrypoint.sh

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Use the entrypoint script
ENTRYPOINT /usr/local/bin/entrypoint.sh 