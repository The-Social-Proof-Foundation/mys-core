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
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build the application
RUN cargo build --release --bin mys-graphql-rpc

# Production Image
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/mys-graphql-rpc /app/

# Set default environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Expose port
EXPOSE 8080 