FROM rust:1.81-bullseye  AS chef
WORKDIR mys
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y cmake clang

FROM chef AS builder 

# Build application
COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY mys-execution mys-execution
COPY external-crates external-crates
RUN cargo build --release \
    --bin mys-source-validation-service

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR mys
RUN apt update && apt install -y git
COPY --from=builder /mys/target/release/mys-source-validation-service /usr/local/bin
COPY crates/mys-source-validation-service/config.toml /var/mys/

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
