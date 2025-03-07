FROM rust:1.81-bullseye  AS chef
WORKDIR mys
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y cmake clang libpq5 ca-certificates libpq-dev postgresql

# Build application
FROM chef AS builder 

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY mys-execution mys-execution
COPY external-crates external-crates
RUN cargo build --release \
    --bin mys-oracle \
    --bin mysns-indexer
RUN mkdir /mys/bin/
RUN find /mys/target/release/ -maxdepth 1 -type f -executable -print 
RUN find /mys/target/release/ -maxdepth 1 -type f -executable -print | xargs cp -t /mys/bin/

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR mys
COPY --from=builder /mys/bin /tmp/mys-bin
RUN find /tmp/mys-bin -maxdepth 1 -type f -executable -print | xargs cp -t /usr/local/bin/
RUN rm -rf /tmp/mys-bin
RUN apt update && apt install -y libpq5 libpq-dev postgresql

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

