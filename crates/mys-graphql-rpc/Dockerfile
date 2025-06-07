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

# Build with pg_backend feature
RUN cargo build --release --features "pg_backend" --bin mys-graphql-rpc

# Production Image
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libjemalloc-dev \
    ca-certificates \
    curl \
    libpq5 \
    libpq-dev \
    postgresql

# Use jemalloc as memory allocator
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so

WORKDIR /app
COPY --from=builder /app/target/release/mys-graphql-rpc /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Default command that uses environment variables
CMD ["mys-graphql-rpc", "start-server", \
     "--host", "0.0.0.0", \
     "--port", "$PORT", \
     "--db-url", "$DATABASE_URL", \
     "--node-rpc-url", "$RPC_URL"] 