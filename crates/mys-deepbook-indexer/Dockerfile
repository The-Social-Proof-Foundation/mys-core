# MySocial DeepBook Indexer Dockerfile for Railway Deployment
FROM rust:1.81-bullseye

WORKDIR /work

# Install system dependencies  
RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    ca-certificates \
    curl \
    libpq-dev \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace from the repository root
COPY Cargo.toml Cargo.lock ./
COPY consensus/ consensus/
COPY crates/ crates/
COPY mys-execution/ mys-execution/
COPY external-crates/ external-crates/

# Build the project
RUN cargo build --release --bin mys-deepbook-indexer

# Expose ports
EXPOSE 8080 9090

# Default command - will use environment variables for config
CMD ["cargo", "run", "--release", "--bin", "mys-deepbook-indexer"] 