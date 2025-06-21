# Build application
FROM rust:1.81-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR /app

# Install build dependencies
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
RUN cargo build --release --bin mys-social-indexer

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
COPY --from=builder /app/target/release/mys-social-indexer /usr/local/bin
COPY --from=builder /app/crates/mys-social-indexer/migrations /app/migrations

# Create entrypoint script
RUN echo '#!/bin/sh\n\
echo "=== ENTRYPOINT DEBUG ==="\n\
echo "All environment variables:"\n\
env | grep -E "(DATABASE_URL|PORT)" || echo "No matching env vars found"\n\
echo ""\n\
echo "=== Key Environment Variables ==="\n\
echo "DATABASE_URL: ${DATABASE_URL:-NOT_SET}"\n\
echo "PORT: ${PORT:-8080}"\n\
echo "=============================="\n\
\n\
# Validate required environment variables\n\
if [ -z "$DATABASE_URL" ]; then\n\
  echo "ERROR: DATABASE_URL is not set"\n\
  exit 1\n\
fi\n\
\n\
# Use actual environment variable values\n\
echo "Starting mys-social-indexer with:"\n\
echo "  DATABASE_URL: $DATABASE_URL"\n\
echo "  PORT: ${PORT:-8080}"\n\
echo ""\n\
\n\
exec mys-social-indexer' > /usr/local/bin/entrypoint.sh && \
    chmod +x /usr/local/bin/entrypoint.sh

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Expose API port
EXPOSE 8080

# Use the entrypoint script
ENTRYPOINT /usr/local/bin/entrypoint.sh