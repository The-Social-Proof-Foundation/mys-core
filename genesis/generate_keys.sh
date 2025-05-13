#!/bin/bash
# Script to generate fullnode and faucet keys

# Set up colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Store the original directory
GENESIS_DIR=$(pwd)

echo -e "${GREEN}MySocial Genesis Key Generator${NC}"
echo "-----------------------------------"

# Array to store validator addresses
declare -a VALIDATOR_ADDRESSES

# Step 1: Generate validator keys
echo -e "${YELLOW}Generating validator keys...${NC}"

for i in {1..3}; do
  echo -e "Creating keys for validator $i..."
  # Create directory if it doesn't exist
  mkdir -p "${GENESIS_DIR}/validators/validator$i"
  
  # Change to the validator directory for key generation
  pushd "${GENESIS_DIR}/validators/validator$i" > /dev/null
  
  echo "  • Generating account keys..."
  cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519 > account_output.txt
  
  echo "  • Generating protocol keys..."
  cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate bls12381 > protocol_output.txt
  
  echo "  • Generating network keys..."
  cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519 > network_output.txt
  
  echo "  • Generating worker keys..."
  cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519 > worker_output.txt
  
  # Extract keys and addresses from the output
  echo "  • Extracting keys and addresses..."
  
  # Extract addresses - properly handle table formatting
  ACCOUNT_ADDRESS=$(grep "mysAddress" account_output.txt | sed 's/│//g' | awk '{print $2}' | xargs)
  PROTOCOL_ADDRESS=$(grep "mysAddress" protocol_output.txt | sed 's/│//g' | awk '{print $2}' | xargs)
  NETWORK_ADDRESS=$(grep "mysAddress" network_output.txt | sed 's/│//g' | awk '{print $2}' | xargs)
  WORKER_ADDRESS=$(grep "mysAddress" worker_output.txt | sed 's/│//g' | awk '{print $2}' | xargs)
  
  # Add to validator addresses array
  VALIDATOR_ADDRESSES[$i-1]=$ACCOUNT_ADDRESS
  
  echo "$ACCOUNT_ADDRESS" > address.txt
  
  # Save mnemonics for recovery
  grep "mnemonic" account_output.txt | sed 's/│//g' | awk '{$1=""; print $0}' | xargs > account.mnemonic
  grep "mnemonic" protocol_output.txt | sed 's/│//g' | awk '{$1=""; print $0}' | xargs > protocol.mnemonic
  grep "mnemonic" network_output.txt | sed 's/│//g' | awk '{$1=""; print $0}' | xargs > network.mnemonic
  grep "mnemonic" worker_output.txt | sed 's/│//g' | awk '{$1=""; print $0}' | xargs > worker.mnemonic
  
  # Extract public keys - properly handle table formatting
  grep "publicBase64Key" protocol_output.txt | sed 's/│//g' | awk '{print $2}' | xargs > protocol.pub
  grep "publicBase64Key" network_output.txt | sed 's/│//g' | awk '{print $2}' | xargs > network.pub
  grep "publicBase64Key" worker_output.txt | sed 's/│//g' | awk '{print $2}' | xargs > worker.pub
  grep "publicBase64Key" account_output.txt | sed 's/│//g' | awk '{print $2}' | xargs > account.pub
  
  # Move automatically generated .key files to standardized names
  if [ -f "${ACCOUNT_ADDRESS}.key" ]; then
    mv "${ACCOUNT_ADDRESS}.key" account.key
    echo "  • Created account.key"
  fi
  
  if [ -f "${PROTOCOL_ADDRESS}.key" ]; then
    mv "${PROTOCOL_ADDRESS}.key" protocol.key
    echo "  • Created protocol.key"
  fi
  
  if [ -f "${NETWORK_ADDRESS}.key" ]; then
    mv "${NETWORK_ADDRESS}.key" network.key
    echo "  • Created network.key"
  fi
  
  if [ -f "${WORKER_ADDRESS}.key" ]; then
    mv "${WORKER_ADDRESS}.key" worker.key
    echo "  • Created worker.key"
  fi
  
  # Save the entire output files as well for reference
  cp protocol_output.txt protocol.json
  cp network_output.txt network.json
  cp worker_output.txt worker.json
  cp account_output.txt account.json
  
  # Return to the genesis directory
  popd > /dev/null
  
  echo -e "${GREEN}Validator $i keys generated successfully!${NC}"
  echo "  Account Address: $ACCOUNT_ADDRESS"
  echo "  Keys saved to validators/validator$i/"
  echo
done

# Step 2: Generate fullnode key
echo -e "${YELLOW}Generating fullnode key...${NC}"

# Create the key in its own directory
mkdir -p "${GENESIS_DIR}/fullnode"
pushd "${GENESIS_DIR}/fullnode" > /dev/null

cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519 > fullnode_output.txt

# Extract key information
FULLNODE_ADDRESS=$(grep "mysAddress" fullnode_output.txt | sed 's/│//g' | awk '{print $2}' | xargs)
FULLNODE_MNEMONIC=$(grep "mnemonic" fullnode_output.txt | sed 's/│//g' | awk '{$1=""; print $0}' | xargs)

echo "Fullnode Address: $FULLNODE_ADDRESS"
echo "Fullnode Mnemonic: $FULLNODE_MNEMONIC"

# Save fullnode info for reference
echo "Fullnode Address: $FULLNODE_ADDRESS" > fullnode_info.txt
echo "Fullnode Mnemonic: $FULLNODE_MNEMONIC" >> fullnode_info.txt
cp fullnode_output.txt fullnode.json

# Move fullnode key file if it exists
if [ -f "${FULLNODE_ADDRESS}.key" ]; then
    mv "${FULLNODE_ADDRESS}.key" fullnode.key
    echo "Created fullnode.key"
fi

# Return to genesis directory
popd > /dev/null

echo -e "${GREEN}Fullnode key generated successfully!${NC}"
echo "Address: $FULLNODE_ADDRESS"
echo "Key saved to fullnode/"
echo

# Step 3: Generate faucet key
echo -e "${YELLOW}Generating faucet key...${NC}"

# Create the key in its own directory
mkdir -p "${GENESIS_DIR}/faucet"
pushd "${GENESIS_DIR}/faucet" > /dev/null

cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519 > faucet_output.txt

# Extract key information
FAUCET_ADDRESS=$(grep "mysAddress" faucet_output.txt | sed 's/│//g' | awk '{print $2}' | xargs)
FAUCET_MNEMONIC=$(grep "mnemonic" faucet_output.txt | sed 's/│//g' | awk '{$1=""; print $0}' | xargs)

echo "Faucet Address: $FAUCET_ADDRESS"
echo "Faucet Mnemonic: $FAUCET_MNEMONIC"

# Save faucet info for later use
echo "Faucet Address: $FAUCET_ADDRESS" > faucet_info.txt
echo "Faucet Mnemonic: $FAUCET_MNEMONIC" >> faucet_info.txt
cp faucet_output.txt faucet.json

# Move faucet key file if it exists
if [ -f "${FAUCET_ADDRESS}.key" ]; then
    mv "${FAUCET_ADDRESS}.key" faucet.key
    echo "Created faucet.key"
fi

# Return to genesis directory
popd > /dev/null

echo -e "${GREEN}Faucet key generated successfully!${NC}"
echo "Address: $FAUCET_ADDRESS"
echo "Key saved to faucet/"
echo

# Step 4: Update genesis_config.yaml with generated addresses
echo -e "${YELLOW}Updating genesis_config.yaml with generated addresses...${NC}"

# Make a backup of the original file
cp "${GENESIS_DIR}/genesis_config.yaml" "${GENESIS_DIR}/genesis_config.yaml.backup"

# Create a new temporary file with the updated accounts section
cat > "${GENESIS_DIR}/genesis_config.new.yaml" << EOL
---
# Genesis configuration parameters
# This file controls the epoch duration and stake subsidy parameters

parameters:
  # Chain start timestamp (in milliseconds since epoch)
  chain_start_timestamp_ms: $(date +%s)000

  # Protocol version
  protocol_version: 74  # Latest version

  # Whether to allow insertion of extra objects in genesis
  allow_insertion_of_extra_objects: true

  # Epoch duration in milliseconds (default: 24 hours)
  # 1 hour = 3,600,000 ms
  # 24 hours = 86,400,000 ms
  epoch_duration_ms: 7200000  # 2 hours for testnet (faster epochs)

  # Stake subsidy parameters
  #
  # When to start paying stake subsidies (0 = from beginning)
  stake_subsidy_start_epoch: 0

  # Initial stake subsidy distribution amount per epoch (in MIST)
  # Default: 35,000 MySo = 35,000,000,000,000 MIST
  stake_subsidy_initial_distribution_amount: 35000000000000

  # Number of epochs before decreasing the subsidy amount
  # Default: 15 epochs
  stake_subsidy_period_length: 15

  # Rate at which subsidy decreases at end of each period (in basis points)
  # 300 basis points = 3%
  stake_subsidy_decrease_rate: 300

# Accounts to add tokens to
accounts:
  # Treasury/Team allocation
  - address: "$FULLNODE_ADDRESS"
    gas_amounts:
      - 250000000000000 # 250,000 MySo
  # Validator 1
  - address: "${VALIDATOR_ADDRESSES[0]}"
    gas_amounts:
      - 500000000000000 # 500,000 MySo
  # Validator 2
  - address: "${VALIDATOR_ADDRESSES[1]}"
    gas_amounts:
      - 500000000000000 # 500,000 MySo
  # Validator 3
  - address: "${VALIDATOR_ADDRESSES[2]}"
    gas_amounts:
      - 500000000000000 # 500,000 MySo
  # Faucet
  - address: "$FAUCET_ADDRESS"
    gas_amounts:
      - 100000000000000 # 100,000 MySo
EOL

# Replace the original file with the new one
mv "${GENESIS_DIR}/genesis_config.new.yaml" "${GENESIS_DIR}/genesis_config.yaml"

echo -e "${GREEN}Genesis config updated successfully!${NC}"
echo "Backup saved as genesis_config.yaml.backup"
echo

echo -e "${GREEN}All keys generated and configurations updated!${NC}"
echo
echo "Remember to securely back up all generated keys!"
echo "You can now run: myso genesis -f --with-faucet --committee-size 3 --from-config genesis_config.yaml" 