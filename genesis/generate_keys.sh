#!/bin/bash
# Script to generate fullnode and faucet keys with dynamic port configuration

# Set up colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Help function
show_help() {
    echo -e "${GREEN}MySocial Genesis Key Generator${NC}"
    echo "-----------------------------------"
    echo "This script generates validator, fullnode, and faucet keys with dynamic port configuration."
    echo
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Options:"
    echo "  -h, --help          Show this help message"
    echo "  -i, --ip IP_ADDR    Set the base IP address (default: 69.10.63.78)"
    echo
    echo "Environment Variables:"
    echo "  BASE_IP             Base IP address for all validators (same as -i flag)"
    echo
    echo "Examples:"
    echo "  $0                           # Use default IP (69.10.63.78)"
    echo "  $0 -i 192.168.1.100         # Use custom IP"
    echo "  BASE_IP=10.0.0.1 $0         # Use environment variable"
    echo
    exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            ;;
        -i|--ip)
            BASE_IP="$2"
            shift 2
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use -h or --help for usage information."
            exit 1
            ;;
    esac
done

# Store the original directory
GENESIS_DIR=$(pwd)

echo -e "${GREEN}MySocial Genesis Key Generator${NC}"
echo "-----------------------------------"

# Array to store validator addresses and port information
declare -a VALIDATOR_ADDRESSES
declare -a VALIDATOR_PORTS

# Base IP address (configurable)
BASE_IP="${BASE_IP:-69.10.63.78}"

# Validate IP address format
validate_ip() {
    local ip=$1
    if [[ $ip =~ ^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}$ ]]; then
        local IFS='.'
        local -a ip_parts=($ip)
        for part in "${ip_parts[@]}"; do
            if [[ $part -gt 255 ]]; then
                return 1
            fi
        done
        return 0
    else
        return 1
    fi
}

# Validate the BASE_IP
if ! validate_ip "$BASE_IP"; then
    echo -e "${RED}Error: Invalid IP address format: $BASE_IP${NC}"
    echo "Please provide a valid IPv4 address."
    exit 1
fi

echo "Using base IP address: $BASE_IP"

# Function to generate a unique port number
# Takes a base port and validator index as parameters
generate_port() {
  local base_port=$1
  local validator_index=$2
  echo $((base_port + (validator_index * 20)))
}

# Function to check if port is available (optional enhancement)
is_port_available() {
  local port=$1
  if command -v ss &> /dev/null; then
    ! ss -tuln | grep -q ":$port "
  elif command -v netstat &> /dev/null; then
    ! netstat -tuln | grep -q ":$port "
  else
    # If no port checking tools available, assume available
    true
  fi
}

# Generate port configurations for all validators
echo -e "${YELLOW}Generating port configurations...${NC}"
for i in {0..2}; do
  # Each validator gets a 100-port range to avoid any conflicts
  # Validator 0: 59000-59099, Validator 1: 59100-59199, Validator 2: 59200-59299
  BASE_VALIDATOR_PORT=$((59000 + (i * 100)))
  
  NETWORK_PORT=$((BASE_VALIDATOR_PORT + 10))      # 59010, 59110, 59210
  P2P_PORT=$((BASE_VALIDATOR_PORT + 20))          # 59020, 59120, 59220
  NARWHAL_PRIMARY_PORT=$((BASE_VALIDATOR_PORT + 30))  # 59030, 59130, 59230
  NARWHAL_WORKER_PORT=$((BASE_VALIDATOR_PORT + 40))   # 59040, 59140, 59240
  CONSENSUS_PORT=$((BASE_VALIDATOR_PORT + 50))    # 59050, 59150, 59250
  
  VALIDATOR_PORTS[$i]="$NETWORK_PORT,$P2P_PORT,$NARWHAL_PRIMARY_PORT,$NARWHAL_WORKER_PORT,$CONSENSUS_PORT"
  
  echo "Validator $i ports: Network=$NETWORK_PORT, P2P=$P2P_PORT, Primary=$NARWHAL_PRIMARY_PORT, Worker=$NARWHAL_WORKER_PORT, Consensus=$CONSENSUS_PORT"
done
echo

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

# Step 4: Generate social-proof-foundation key
echo -e "${YELLOW}Generating social-proof-foundation key...${NC}"

# Create the key in its own directory
mkdir -p "${GENESIS_DIR}/social-proof-foundation"
pushd "${GENESIS_DIR}/social-proof-foundation" > /dev/null

cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519 > social_proof_foundation_output.txt

# Extract key information
SOCIAL_PROOF_FOUNDATION_ADDRESS=$(grep "mysAddress" social_proof_foundation_output.txt | sed 's/│//g' | awk '{print $2}' | xargs)
SOCIAL_PROOF_FOUNDATION_MNEMONIC=$(grep "mnemonic" social_proof_foundation_output.txt | sed 's/│//g' | awk '{$1=""; print $0}' | xargs)

echo "Social Proof Foundation Address: $SOCIAL_PROOF_FOUNDATION_ADDRESS"
echo "Social Proof Foundation Mnemonic: $SOCIAL_PROOF_FOUNDATION_MNEMONIC"

# Save social-proof-foundation info for reference
echo "Social Proof Foundation Address: $SOCIAL_PROOF_FOUNDATION_ADDRESS" > social_proof_foundation_info.txt
echo "Social Proof Foundation Mnemonic: $SOCIAL_PROOF_FOUNDATION_MNEMONIC" >> social_proof_foundation_info.txt
cp social_proof_foundation_output.txt social_proof_foundation.json

# Move social-proof-foundation key file if it exists
if [ -f "${SOCIAL_PROOF_FOUNDATION_ADDRESS}.key" ]; then
    mv "${SOCIAL_PROOF_FOUNDATION_ADDRESS}.key" social_proof_foundation.key
    echo "Created social_proof_foundation.key"
fi

# Return to genesis directory
popd > /dev/null

echo -e "${GREEN}Social Proof Foundation key generated successfully!${NC}"
echo "Address: $SOCIAL_PROOF_FOUNDATION_ADDRESS"
echo "Key saved to social-proof-foundation/"
echo

# Step 5: Generate core-team key
echo -e "${YELLOW}Generating core-team key...${NC}"

# Create the key in its own directory
mkdir -p "${GENESIS_DIR}/core-team"
pushd "${GENESIS_DIR}/core-team" > /dev/null

cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519 > core_team_output.txt

# Extract key information
CORE_TEAM_ADDRESS=$(grep "mysAddress" core_team_output.txt | sed 's/│//g' | awk '{print $2}' | xargs)
CORE_TEAM_MNEMONIC=$(grep "mnemonic" core_team_output.txt | sed 's/│//g' | awk '{$1=""; print $0}' | xargs)

echo "Core Team Address: $CORE_TEAM_ADDRESS"
echo "Core Team Mnemonic: $CORE_TEAM_MNEMONIC"

# Save core-team info for later use
echo "Core Team Address: $CORE_TEAM_ADDRESS" > core_team_info.txt
echo "Core Team Mnemonic: $CORE_TEAM_MNEMONIC" >> core_team_info.txt
cp core_team_output.txt core_team.json

# Move core-team key file if it exists
if [ -f "${CORE_TEAM_ADDRESS}.key" ]; then
    mv "${CORE_TEAM_ADDRESS}.key" core_team.key
    echo "Created core_team.key"
fi

# Return to genesis directory
popd > /dev/null

echo -e "${GREEN}Core Team key generated successfully!${NC}"
echo "Address: $CORE_TEAM_ADDRESS"
echo "Key saved to core-team/"
echo

# Step 6: Update genesis_config.yaml with generated addresses
echo -e "${YELLOW}Updating genesis_config.yaml with generated addresses...${NC}"

# Make a backup of the original file
cp "${GENESIS_DIR}/genesis_config.yaml" "${GENESIS_DIR}/genesis_config.yaml.backup"

# Create a new temporary file with the updated accounts section
cat > "${GENESIS_DIR}/genesis_config.new.yaml" << EOL
---
# Genesis configuration parameters
# This file controls the epoch duration and stake subsidy parameters

parameters:
  # Chain start timestamp current time + 0 hour (in milliseconds since epoch)
  chain_start_timestamp_ms: $(( $(date +%s) * 1000 ))

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
  stake_subsidy_start_epoch: 2

  # Initial stake subsidy distribution amount per epoch (in MIST)
  # Default: 538,626 MySo = 538,626,000,000,000 MIST
  stake_subsidy_initial_distribution_amount: 538626000000000

  # Number of epochs before decreasing the subsidy amount
  # Default: 15 epochs
  stake_subsidy_period_length: 10

  # Rate at which subsidy decreases at end of each period (in basis points)
  # 100 basis points = 1%
  stake_subsidy_decrease_rate: 100

# Accounts to add tokens to
accounts:
  # Fullnode allocation
  - address: "$FULLNODE_ADDRESS"
    gas_amounts:
      - 500000000000000 # 500,000 MySo
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
      - 1000000000000000 # 1,000,000 MySo
  # Social Proof Foundation
  - address: "$SOCIAL_PROOF_FOUNDATION_ADDRESS"
    gas_amounts:
      - 235000000000000000 # 240,000,000 MySo (24% - 2mill for owning fullnode & validator - 3mill for 3 validators init staking)
  # Core Team
  - address: "$CORE_TEAM_ADDRESS"
    gas_amounts:
      - 250000000000000000 # 250,000,000 MySo (25% 12.5% core & 12.5 marketing)

validator_config_info:
EOL

# Generate dynamic validator configuration
for i in {0..2}; do
  # Parse the ports for this validator
  IFS=',' read -r NETWORK_PORT P2P_PORT NARWHAL_PRIMARY_PORT NARWHAL_WORKER_PORT CONSENSUS_PORT <<< "${VALIDATOR_PORTS[$i]}"
  
  cat >> "${GENESIS_DIR}/genesis_config.new.yaml" << EOL
  # Validator $i (dynamic ports)
  - name: "MySo Validator $i"
    network_address: "/ip4/$BASE_IP/tcp/$NETWORK_PORT/http"
    p2p_address: "/ip4/$BASE_IP/udp/$P2P_PORT"
    narwhal_primary_address: "/ip4/$BASE_IP/udp/$NARWHAL_PRIMARY_PORT"
    narwhal_worker_address: "/ip4/$BASE_IP/udp/$NARWHAL_WORKER_PORT"
    consensus_address: "/ip4/$BASE_IP/tcp/$CONSENSUS_PORT/http"
    gas_price: 1
    commission_rate: 500
    stake: 1000000000000000  # 1 million MySo in MIST units
EOL
done

# Replace the original file with the new one
mv "${GENESIS_DIR}/genesis_config.new.yaml" "${GENESIS_DIR}/genesis_config.yaml"

# Save port configuration for reference
echo -e "${YELLOW}Saving port configuration...${NC}"
cat > "${GENESIS_DIR}/port_config.txt" << EOL
MySocial Validator Port Configuration
Generated on: $(date)
Base IP: $BASE_IP

EOL

for i in {0..2}; do
  IFS=',' read -r NETWORK_PORT P2P_PORT NARWHAL_PRIMARY_PORT NARWHAL_WORKER_PORT CONSENSUS_PORT <<< "${VALIDATOR_PORTS[$i]}"
  cat >> "${GENESIS_DIR}/port_config.txt" << EOL
Validator $i:
  - Network Address: $BASE_IP:$NETWORK_PORT
  - P2P Address: $BASE_IP:$P2P_PORT  
  - Narwhal Primary: $BASE_IP:$NARWHAL_PRIMARY_PORT
  - Narwhal Worker: $BASE_IP:$NARWHAL_WORKER_PORT
  - Consensus: $BASE_IP:$CONSENSUS_PORT

EOL
done

echo -e "${GREEN}Genesis config updated successfully!${NC}"
echo "Backup saved as genesis_config.yaml.backup"
echo "Port configuration saved to port_config.txt"
echo

echo -e "${GREEN}All keys generated and configurations updated!${NC}"
echo
echo "Generated port assignments:"
for i in {0..2}; do
  IFS=',' read -r NETWORK_PORT P2P_PORT NARWHAL_PRIMARY_PORT NARWHAL_WORKER_PORT CONSENSUS_PORT <<< "${VALIDATOR_PORTS[$i]}"
  echo "  Validator $i: Network=$NETWORK_PORT, P2P=$P2P_PORT, Primary=$NARWHAL_PRIMARY_PORT, Worker=$NARWHAL_WORKER_PORT, Consensus=$CONSENSUS_PORT"
done
echo
echo "Base IP: $BASE_IP (set BASE_IP environment variable to override)"
echo "Remember to securely back up all generated keys!"
echo "You can now run: myso genesis -f --with-faucet --committee-size 3 --from-config genesis_config.yaml" 
