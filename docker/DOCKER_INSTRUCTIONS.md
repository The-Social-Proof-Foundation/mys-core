# Docker Image Build Instructions

## Issue with macOS Binaries

The binaries in your `target/release` directory are compiled for macOS (ARM64), which won't work in Linux Docker containers. You need to compile these binaries on a Linux AMD64 system.

## Two Options

### Option 1: Compile on Linux System (Preferred)

1. SSH into your Linux AMD64 system
2. Clone the repository:
   ```bash
   git clone https://github.com/The-Social-Proof-Foundation/mys-core.git
   cd mys-core
   ```

3. Install dependencies:
   ```bash
   sudo apt-get update
   sudo apt-get install -y \
     build-essential \
     curl \
     git \
     pkg-config \
     libssl-dev \
     cmake \
     clang \
     llvm \
     libclang-dev
   ```

4. Build the required binaries:
   ```bash
   cargo build --release --bin mys-node
   cargo build --release --bin mys-indexer
   cargo build --release --bin mys-edge-proxy
   cargo build --release --bin mys-faucet
   cargo build --release --bin mys-security-watchdog
   ```

5. Copy the `build_from_compiled.sh` and `Dockerfile.simple` from your macOS system to the Linux system.

6. Run the build script:
   ```bash
   ./build_from_compiled.sh
   ```

### Option 2: Use Multi-Stage Docker Builds

If you don't have access to a Linux system, you can use Docker's multi-stage builds to compile the binaries inside a Linux container:

1. Use the Dockerfiles we've already created (Dockerfile.node, etc.)
2. Build the images:
   ```bash
   docker build -t socialproof/mys-node:latest -f Dockerfile.node .
   docker build -t socialproof/mys-indexer:latest -f Dockerfile.indexer .
   docker build -t socialproof/mys-edge-proxy:latest -f Dockerfile.edge-proxy .
   docker build -t socialproof/mys-faucet:latest -f Dockerfile.faucet .
   docker build -t socialproof/mys-security-watchdog:latest -f Dockerfile.security-watchdog .
   ```

This will take a long time (30+ minutes per image) as it needs to:
1. Download a Rust development environment
2. Clone the repository
3. Build the entire project for each binary

## Recommendation

For the fastest results, use Option 1 if you have access to a Linux AMD64 system. For the most reliable results (but slower build times), use Option 2. 