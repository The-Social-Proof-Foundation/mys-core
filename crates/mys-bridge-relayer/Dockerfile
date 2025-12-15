# Build application
#
# Build from workspace root so Cargo workspace deps resolve.

FROM rustlang/rust:nightly-bullseye AS builder
WORKDIR "/mys"

# Runtime deps for compiling Diesel/Postgres and some workspace crates
RUN apt-get update && apt-get install -y \
    ca-certificates \
    clang \
    cmake \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY mys-execution mys-execution
COPY external-crates external-crates

RUN cargo build --release --package mys-bridge-relayer --bin mys-bridge-relayer

# Runtime image
FROM debian:bullseye-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR "/mys"

COPY --from=builder /mys/target/release/mys-bridge-relayer /usr/local/bin/mys-bridge-relayer
COPY crates/mys-bridge-relayer/config.yaml /mys/config.yaml

# Railway will provide $PORT; we bind metrics there by default.
CMD ["mys-bridge-relayer", "--config-path", "/mys/config.yaml"]
