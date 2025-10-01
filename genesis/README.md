# Genesis Ceremony Instructions

This folder contains the files and instructions needed to perform the genesis ceremony for your MySocial network.

## Directory Structure

```
/genesis/
├── README.md                   # This file
├── token_distribution.csv      # Token distribution at genesis
├── validator_keys/             # Validator keys
│   ├── validator1/             # Keys for validator 1
│   ├── validator2/             # Keys for validator 2
│   └── validator3/             # Keys for validator 3
└── genesis.blob                # Generated genesis blob
```

## Step 1: Generate Validator Keys

Use the provided script to generate validator keys:

```bash
# Run the key generation script
./generate_keys.sh
```

This script will:
- Generate four keypairs (account, protocol, network, worker) for each validator
- Save them with standardized names (e.g., `protocol.key`, `network.key`) in validator-specific directories
- Generate a faucet key and update the token distribution CSV
- Save mnemonics for all keys for recovery purposes

The directory structure will be:
```
/genesis/
├── validator_keys/
│   ├── validator1/
│   │   ├── account.key    # Main account key
│   │   ├── protocol.key   # Consensus protocol key
│   │   ├── network.key    # Network identity key
│   │   ├── worker.key     # Worker key
│   │   ├── *.pub          # Public keys
│   │   └── *.mnemonic     # Mnemonics for recovery
│   ├── validator2/
│   │   └── ...
│   └── validator3/
│       └── ...
├── faucet/
│   ├── faucet.key         # Faucet private key
│   └── faucet_info.txt    # Faucet address and mnemonic
└── ...
```

## Step 2: Generate Faucet Key

Generate a keypair for the faucet and update the token distribution CSV:

```bash
# Generate faucet key
cargo run --bin myso -- generate-keypair > faucet.json

# Update the faucet address in token_distribution.csv
FAUCET_ADDRESS=$(cat faucet.json | jq -r '.address')
sed -i "s/0xFAUCET_ADDRESS_TO_BE_REPLACED/$FAUCET_ADDRESS/g" token_distribution.csv
```

## Step 3: Initialize Genesis Ceremony

The genesis ceremony allows you to customize parameters like token distribution, epoch duration, and stake subsidy settings. Edit the `genesis_config.yaml` file if needed for custom parameters.

Next, initialize the genesis ceremony with your token distribution CSV:

```bash
# Run the init script (recommended)
./init_genesis.sh
```

Or manually initialize:

```bash
# Initialize genesis ceremony with token distribution
# IMPORTANT: These parameters go BEFORE the 'init' subcommand
cargo run --bin myso -- genesis-ceremony \
  --path . \
  --token-distribution token_distribution.csv \
  --token-symbol MySo \
  --token-name MySocial \
  --token-description "MySocial Native Token" \
  --token-supply 1000000000000000000 \
  init
```

> **Important:** Token distribution must be specified during initialization as a parameter to the main `genesis-ceremony` command. You cannot add it later in the process.

## Step 4: Add Validators

You can add validators manually using the commands below, or use the provided script for automation:

```bash
# Run the add validators script (recommended)
./add_validators.sh
```

Or manually add each validator:

```bash
# Add validators one by one
cargo run --bin myso -- genesis-ceremony --path . add-validator \
  --name "Validator-1" \
  --validator-key-file validator_keys/validator1/protocol.json \
  --network-key-file validator_keys/validator1/network.json \
  --worker-key-file validator_keys/validator1/worker.json \
  --account-key-file validator_keys/validator1/account.json \
  --description "Validator 1" \
  --image-url "https://mysocial.io/logo.png" \
  --project-url "https://mysocial.io" \
  --network-address "/dns/validator1.mysocial.io/tcp/8080/http" \
  --p2p-address "/dns/validator1.mysocial.io/udp/8084" \
  --narwhal-primary-address "/dns/validator1.mysocial.io/udp/8081" \
  --narwhal-worker-address "/dns/validator1.mysocial.io/udp/8082" \
  --gas-price 100 \
  --commission-rate 200

# Add remaining validators (similar command for each)
```

## Step 5: Finalize Genesis

You can finalize the genesis ceremony using the provided script:

```bash
# Run the finalize genesis script (recommended)
./finalize_genesis.sh
```

Or manually run the finalization steps:

```bash
# Build unsigned checkpoint and finalize
cargo run --bin myso -- genesis-ceremony --path . build-unsigned-checkpoint
cargo run --bin myso -- genesis-ceremony --path . finalize
```

After completing these steps, you will have a `genesis.blob` file that can be used to start your network.

## Step 6: Deploy

Copy the genesis.blob to both of your servers in the appropriate locations:

```bash
# Copy to fullnode server
scp genesis.blob user@fullnode-server:/path/to/mys-core/docker/myso-fullnode/genesis.blob

# Copy to validator server
scp genesis.blob user@validator-server:/path/to/mys-core/docker/myso-validators/genesis.blob
```

Then start your nodes using Docker:

```bash
# On validator server
cd /path/to/mys-core/docker/myso-validators
docker-compose up -d

# On fullnode server
cd /path/to/mys-core/docker/myso-fullnode
docker-compose up -d
```

## Notes

- Keep your validator keys secure and backed up
- Save the faucet key for use in the Railway.app deployment
- Document all addresses and their purposes
- The gas-price and commission-rate are specified in basis points (1 basis point = 0.01%)
- Token amounts in the distribution CSV are in MIST (1 MySo = 10^9 MIST)
