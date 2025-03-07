# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.81-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/mys"
RUN apt-get update && apt-get install -y cmake clang

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY mys-execution mys-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} --bin mys-node

# Production Image
FROM debian:bullseye-slim AS runtime
# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y libjemalloc-dev ca-certificates curl
ENV LD_PRELOAD /usr/lib/x86_64-linux-gnu/libjemalloc.so
ARG PROFILE=release
WORKDIR "$WORKDIR/mys"
# Both bench and release profiles copy from release dir
COPY --from=builder /mys/target/release/mys-node /opt/mys/bin/mys-node
# Support legacy usages of /usr/local/bin/mys-node
COPY --from=builder /mys/target/release/mys-node /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
