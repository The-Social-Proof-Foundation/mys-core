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
    postgresql \
    pkg-config \
    libssl-dev

# Copy source code
COPY . .

# Build the analytics indexer
RUN cargo build --release --bin mys-analytics-indexer

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
    gettext-base \
    libssl1.1 \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Use jemalloc as memory allocator
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/mys-analytics-indexer /usr/local/bin/

# Create directories for storage
RUN mkdir -p /tmp/analytics \
    && mkdir -p /opt/mys/db/package_cache \
    && mkdir -p /app/credentials

# Create simplified entrypoint script
RUN echo '#!/bin/sh\n\
echo "=== MySocial Analytics Indexer ==="\n\
echo "Starting analytics indexer..."\n\
echo ""\n\
\n\
# Validate required environment variables\n\
if [ -z "$REST_URL" ]; then\n\
  echo "ERROR: REST_URL is required"\n\
  echo "Example: REST_URL=https://fullnode.mainnet.mysocial.network:9000"\n\
  exit 1\n\
fi\n\
\n\
if [ -z "$FILE_TYPE" ]; then\n\
  echo "ERROR: FILE_TYPE is required"\n\
  echo "Must be one of: checkpoint, transaction, object, event"\n\
  exit 1\n\
fi\n\
\n\
if [ -z "$REMOTE_STORE_BUCKET" ]; then\n\
  echo "ERROR: REMOTE_STORE_BUCKET is required"\n\
  exit 1\n\
fi\n\
\n\
# Create GCS service account key from environment variables\n\
if [ -n "$GCS_SERVICE_ACCOUNT_JSON" ]; then\n\
  echo "Creating GCS service account key file..."\n\
  echo "$GCS_SERVICE_ACCOUNT_JSON" > /app/credentials/gcs-key.json\n\
  export GOOGLE_APPLICATION_CREDENTIALS="/app/credentials/gcs-key.json"\n\
  echo "✓ GCS authentication configured"\n\
else\n\
  echo "WARNING: GCS_SERVICE_ACCOUNT_JSON not set - uploads may fail"\n\
fi\n\
\n\
# Create checkpoint and cache directories\n\
mkdir -p "${CHECKPOINT_DIR:-/tmp/analytics}"\n\
mkdir -p "${PACKAGE_CACHE_PATH:-/opt/mys/db/package_cache}"\n\
\n\
# Build command arguments\n\
ARGS="--rest-url $REST_URL"\n\
ARGS="$ARGS --file-type $FILE_TYPE"\n\
ARGS="$ARGS --file-format ${FILE_FORMAT:-parquet}"\n\
ARGS="$ARGS --checkpoint-interval ${CHECKPOINT_INTERVAL:-10000}"\n\
ARGS="$ARGS --max-file-size-mb ${MAX_FILE_SIZE_MB:-100}"\n\
ARGS="$ARGS --time-interval-s ${TIME_INTERVAL_S:-600}"\n\
ARGS="$ARGS --checkpoint-dir ${CHECKPOINT_DIR:-/tmp/analytics}"\n\
ARGS="$ARGS --package-cache-path ${PACKAGE_CACHE_PATH:-/opt/mys/db/package_cache}"\n\
ARGS="$ARGS --remote-store-url ${REMOTE_STORE_URL:-https://checkpoints.mainnet.mysocial.network}"\n\
ARGS="$ARGS --remote-store-bucket $REMOTE_STORE_BUCKET"\n\
\n\
if [ -n "$REMOTE_STORE_REGION" ]; then\n\
  ARGS="$ARGS --remote-store-region $REMOTE_STORE_REGION"\n\
fi\n\
\n\
if [ -n "$STARTING_CHECKPOINT_SEQ_NUM" ]; then\n\
  ARGS="$ARGS --starting-checkpoint-seq-num $STARTING_CHECKPOINT_SEQ_NUM"\n\
fi\n\
\n\
echo "=== Configuration ==="\n\
echo "REST_URL: $REST_URL"\n\
echo "FILE_TYPE: $FILE_TYPE"\n\
echo "BUCKET: $REMOTE_STORE_BUCKET"\n\
echo "CHECKPOINT_INTERVAL: ${CHECKPOINT_INTERVAL:-10000}"\n\
echo ""\n\
echo "Starting: mys-analytics-indexer $ARGS"\n\
echo ""\n\
\n\
exec mys-analytics-indexer $ARGS' > /usr/local/bin/entrypoint.sh && \
    chmod +x /usr/local/bin/entrypoint.sh

# Build metadata
ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Expose standard metrics port
EXPOSE 9184

# Health check endpoint for metrics
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:9184/metrics || exit 1

# Use the entrypoint script
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"] 