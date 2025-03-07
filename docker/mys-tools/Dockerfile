# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.81-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/mys"
RUN apt-get update && apt-get install -y cmake clang libpq5 libpq-dev

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY mys-execution mys-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} \
    --bin stress \
    --bin mys-faucet \
    --bin mys-bridge \
    --bin bridge-indexer \
    --bin deepbook-indexer \
    --bin mys-bridge-cli \
    --bin mys \
    --bin mys-cluster-test \
    --bin mys-tool

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/mys"

# mys-tool needs libpq at runtime and uses git when fetching move dependencies during build
RUN apt-get update && apt-get install -y libpq5 libpq-dev ca-certificates git

COPY --from=builder /mys/target/release/stress /usr/local/bin
COPY --from=builder /mys/target/release/mys-faucet /usr/local/bin
COPY --from=builder /mys/target/release/mys-bridge /usr/local/bin
COPY --from=builder /mys/target/release/bridge-indexer /usr/local/bin
COPY --from=builder /mys/target/release/deepbook-indexer /usr/local/bin
COPY --from=builder /mys/target/release/mys-bridge-cli /usr/local/bin
COPY --from=builder /mys/target/release/mys /usr/local/bin
COPY --from=builder /mys/target/release/mys-cluster-test /usr/local/bin
COPY --from=builder /mys/target/release/mys-tool /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
