FROM ubuntu:latest AS chef
WORKDIR mys
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config curl cmake clang ca-certificates
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Build application
FROM chef AS builder

RUN curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY mys-execution mys-execution
COPY external-crates external-crates
RUN cargo build --release --bin mys --bin mys-rosetta

# Production Image
FROM ubuntu:latest AS runtime
WORKDIR mys
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /mys/target/release/mys /usr/local/bin
COPY --from=builder /mys/target/release/mys-rosetta /usr/local/bin
COPY --from=builder /mys/bin/rosetta-cli /usr/local/bin
COPY --from=builder /mys/crates/mys-config/data/fullnode-template.yaml /mys/devnet/fullnode.yaml
RUN /usr/local/bin/mys genesis

ARG BUILD_DATE
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
