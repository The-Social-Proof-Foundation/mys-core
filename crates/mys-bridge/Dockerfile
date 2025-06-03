# MySo Bridge Dockerfile for Railway Deployment
FROM rust:1.81-bullseye as builder

ARG PROFILE=release
WORKDIR /work

RUN apt-get update && apt-get install -y cmake clang

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY mys-execution mys-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} --bin mys-bridge

FROM debian:bullseye-slim as runtime

RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /work/target/release/mys-bridge /opt/mys/bin/mys-bridge

# Copy config file
COPY crates/mys-bridge/bridge-config.yaml /app/bridge-config.yaml

# Create simple entrypoint script
RUN echo '#!/bin/bash\nif [ -n "$BRIDGE_AUTHORITY_KEY" ]; then\n  echo "$BRIDGE_AUTHORITY_KEY" > /tmp/bridge_key\nelse\n  echo "Error: BRIDGE_AUTHORITY_KEY environment variable required"\n  exit 1\nfi\nexec /opt/mys/bin/mys-bridge --config-path /app/bridge-config.yaml "$@"' > /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080 9090

ENTRYPOINT ["/entrypoint.sh"] 