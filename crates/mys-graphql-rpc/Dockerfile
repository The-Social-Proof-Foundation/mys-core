# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.81-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR /mys

# graphql-rpc needs postgres libpq5 and ca-certificates
RUN apt update && apt install -y libpq5 ca-certificates libpq-dev postgresql

RUN apt-get update && apt-get install -y cmake clang

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY mys-execution mys-execution
COPY external-crates external-crates

RUN cargo build --release --features "pg_backend" --package mys-graphql-rpc --bin mys-graphql-rpc

# Production Image
FROM debian:bullseye-slim AS runtime
# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y libjemalloc-dev ca-certificates curl
ENV LD_PRELOAD /usr/lib/x86_64-linux-gnu/libjemalloc.so
WORKDIR /mys
COPY --from=builder /mys/target/release/mys-graphql-rpc /usr/local/bin
RUN apt update && apt install -y libpq5 ca-certificates libpq-dev postgresql

# Create default config directory
RUN mkdir -p /mys/config

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Default command
CMD ["mys-graphql-rpc", "start-server", "--host", "0.0.0.0"] 