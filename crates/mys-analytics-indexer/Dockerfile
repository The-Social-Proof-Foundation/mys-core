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

# Create entrypoint script with comprehensive environment validation
RUN echo '#!/bin/sh\n\
echo "=== MySocial Analytics Indexer ==="\n\
echo "Starting analytics indexer..."\n\
echo ""\n\
echo "=== Environment Configuration ==="\n\
echo "REST_URL: ${REST_URL:-NOT_SET}"\n\
echo "FILE_TYPE: ${FILE_TYPE:-NOT_SET}"\n\
echo "FILE_FORMAT: ${FILE_FORMAT:-csv}"\n\
echo "CHECKPOINT_INTERVAL: ${CHECKPOINT_INTERVAL:-10000}"\n\
echo "MAX_FILE_SIZE_MB: ${MAX_FILE_SIZE_MB:-100}"\n\
echo "TIME_INTERVAL_S: ${TIME_INTERVAL_S:-600}"\n\

echo "CHECKPOINT_DIR: ${CHECKPOINT_DIR:-/tmp/analytics}"\n\
echo "PACKAGE_CACHE_PATH: ${PACKAGE_CACHE_PATH:-/opt/mys/db/package_cache}"\n\
echo "REMOTE_STORE_URL: ${REMOTE_STORE_URL:-https://checkpoints.mainnet.mysocial.network}"\n\
echo ""\n\
\n\
# Validate required environment variables\n\
if [ -z "$REST_URL" ]; then\n\
  echo "ERROR: REST_URL is not set"\n\
  echo "Example: REST_URL=https://fullnode.mainnet.mysocial.network:9000"\n\
  exit 1\n\
fi\n\
\n\
if [ -z "$FILE_TYPE" ]; then\n\
  echo "ERROR: FILE_TYPE is not set"\n\
  echo "Must be one of: checkpoint, transaction, object, event, transaction_objects, move_call, move_package, dynamic_field, wrapped_object"\n\
  exit 1\n\
fi\n\
\n\
# Validate GCS configuration\n\
if [ -z "$REMOTE_STORE_BUCKET" ]; then\n\
  echo "ERROR: REMOTE_STORE_BUCKET is not set"\n\
  echo "GCS bucket name is required for storing analytics data"\n\
  exit 1\n\
fi\n\
\n\
# Create checkpoint directory if it doesnt exist\n\
mkdir -p "${CHECKPOINT_DIR:-/tmp/analytics}"\n\
mkdir -p "${PACKAGE_CACHE_PATH:-/opt/mys/db/package_cache}"\n\
\n\
# Build command arguments\n\
ARGS="--rest-url $REST_URL"\n\
ARGS="$ARGS --file-type $FILE_TYPE"\n\
ARGS="$ARGS --file-format ${FILE_FORMAT:-csv}"\n\
ARGS="$ARGS --checkpoint-interval ${CHECKPOINT_INTERVAL:-10000}"\n\
ARGS="$ARGS --max-file-size-mb ${MAX_FILE_SIZE_MB:-100}"\n\
ARGS="$ARGS --time-interval-s ${TIME_INTERVAL_S:-600}"\n\

ARGS="$ARGS --checkpoint-dir ${CHECKPOINT_DIR:-/tmp/analytics}"\n\
ARGS="$ARGS --package-cache-path ${PACKAGE_CACHE_PATH:-/opt/mys/db/package_cache}"\n\
ARGS="$ARGS --remote-store-url ${REMOTE_STORE_URL:-https://checkpoints.mainnet.mysocial.network}"\n\
\n\
# GCS Configuration\n\
if [ -n "$REMOTE_STORE_BUCKET" ]; then\n\
  ARGS="$ARGS --remote-store-bucket $REMOTE_STORE_BUCKET"\n\
fi\n\
if [ -n "$REMOTE_STORE_REGION" ]; then\n\
  ARGS="$ARGS --remote-store-region $REMOTE_STORE_REGION"\n\
fi\n\
if [ -n "$REMOTE_STORE_PATH_PREFIX" ]; then\n\
  ARGS="$ARGS --remote-store-path-prefix $REMOTE_STORE_PATH_PREFIX"\n\
fi\n\
\n\
# Optional starting checkpoint\n\
if [ -n "$STARTING_CHECKPOINT_SEQ_NUM" ]; then\n\
  ARGS="$ARGS --starting-checkpoint-seq-num $STARTING_CHECKPOINT_SEQ_NUM"\n\
fi\n\
\n\
# Optional package filtering\n\
if [ -n "$PACKAGE_ID_FILTER" ]; then\n\
  ARGS="$ARGS --package-id-filter $PACKAGE_ID_FILTER"\n\
fi\n\
\n\
# BigQuery configuration\n\
if [ "$REPORT_BQ_MAX_TABLE_CHECKPOINT" = "true" ]; then\n\
  ARGS="$ARGS --report-bq-max-table-checkpoint"\n\
  if [ -n "$BQ_SERVICE_ACCOUNT_KEY_FILE" ]; then\n\
    ARGS="$ARGS --bq-service-account-key-file $BQ_SERVICE_ACCOUNT_KEY_FILE"\n\
  fi\n\
  if [ -n "$BQ_PROJECT_ID" ]; then\n\
    ARGS="$ARGS --bq-project-id $BQ_PROJECT_ID"\n\
  fi\n\
  if [ -n "$BQ_DATASET_ID" ]; then\n\
    ARGS="$ARGS --bq-dataset-id $BQ_DATASET_ID"\n\
  fi\n\
  if [ -n "$BQ_TABLE_ID" ]; then\n\
    ARGS="$ARGS --bq-table-id $BQ_TABLE_ID"\n\
  fi\n\
  if [ -n "$BQ_CHECKPOINT_COL_ID" ]; then\n\
    ARGS="$ARGS --bq-checkpoint-col-id $BQ_CHECKPOINT_COL_ID"\n\
  fi\n\
fi\n\
\n\
# Snowflake configuration\n\
if [ "$REPORT_SF_MAX_TABLE_CHECKPOINT" = "true" ]; then\n\
  ARGS="$ARGS --report-sf-max-table-checkpoint"\n\
  if [ -n "$SF_ACCOUNT_IDENTIFIER" ]; then\n\
    ARGS="$ARGS --sf-account-identifier $SF_ACCOUNT_IDENTIFIER"\n\
  fi\n\
  if [ -n "$SF_WAREHOUSE" ]; then\n\
    ARGS="$ARGS --sf-warehouse $SF_WAREHOUSE"\n\
  fi\n\
  if [ -n "$SF_DATABASE" ]; then\n\
    ARGS="$ARGS --sf-database $SF_DATABASE"\n\
  fi\n\
  if [ -n "$SF_SCHEMA" ]; then\n\
    ARGS="$ARGS --sf-schema $SF_SCHEMA"\n\
  fi\n\
  if [ -n "$SF_USERNAME" ]; then\n\
    ARGS="$ARGS --sf-username $SF_USERNAME"\n\
  fi\n\
  if [ -n "$SF_ROLE" ]; then\n\
    ARGS="$ARGS --sf-role $SF_ROLE"\n\
  fi\n\
  if [ -n "$SF_PASSWORD" ]; then\n\
    ARGS="$ARGS --sf-password $SF_PASSWORD"\n\
  fi\n\
  if [ -n "$SF_TABLE_ID" ]; then\n\
    ARGS="$ARGS --sf-table-id $SF_TABLE_ID"\n\
  fi\n\
  if [ -n "$SF_CHECKPOINT_COL_ID" ]; then\n\
    ARGS="$ARGS --sf-checkpoint-col-id $SF_CHECKPOINT_COL_ID"\n\
  fi\n\
fi\n\
\n\
echo "=== Starting Analytics Indexer ==="\n\
echo "Command: mys-analytics-indexer $ARGS"\n\
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