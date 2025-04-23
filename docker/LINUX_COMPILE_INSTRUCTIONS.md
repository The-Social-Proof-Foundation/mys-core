# Compiling MySocial Binaries on Linux AMD

These instructions will guide you through compiling the MySocial binaries on your Linux AMD system.

## Prerequisites

Install the required dependencies:

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

## Clone the Repository

```bash
git clone https://github.com/The-Social-Proof-Foundation/mys-core.git
cd mys-core
```

## Build the Binaries

Build each of the required binaries:

```bash
# Build mys-node
cargo build --release --bin mys-node

# Build mys-indexer
cargo build --release --bin mys-indexer

# Build mys-edge-proxy
cargo build --release --bin mys-edge-proxy

# Build mys-faucet
cargo build --release --bin mys-faucet
```

The compiled binaries will be in the `target/release/` directory.

## Create Docker Images

Once the binaries are compiled, update the `BINARY_PATHS` in the `build_from_compiled.sh` script to point to the locations of your compiled binaries:

```bash
BINARY_PATHS=(
    "/path/to/mys-core/target/release/mys-node"
    "/path/to/mys-core/target/release/mys-indexer"
    "/path/to/mys-core/target/release/mys-edge-proxy"
    "/path/to/mys-core/target/release/mys-faucet"
)
```

Then run the script to build and push the Docker images:

```bash
./build_from_compiled.sh
```

This will create lightweight Docker images that use your pre-compiled binaries, which is much faster than building inside Docker containers. 