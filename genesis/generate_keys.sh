#!/bin/bash
# Script to generate account addresses and network configuration for genesis

# Set up colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Help function
show_help() {
    echo -e "${GREEN}MySocial Genesis Account Generator${NC}"
    echo "-----------------------------------"
    echo "This script generates account addresses for validators, fullnode, faucet, and foundation accounts."
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

echo -e "${GREEN}MySocial Genesis Account Generator${NC}"
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

# Step 1: Generate faucet account address
echo -e "${YELLOW}Generating faucet account address...${NC}"

# Create faucet directory
mkdir -p "${GENESIS_DIR}/faucet"

# Change to faucet directory so the .key file is generated there
cd "${GENESIS_DIR}/faucet"

FAUCET_OUTPUT=$(cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519)
FAUCET_ADDRESS=$(echo "$FAUCET_OUTPUT" | grep "mysAddress" | sed 's/│//g' | awk '{print $2}' | xargs)

# Extract key information
FAUCET_PUBLIC_KEY=$(echo "$FAUCET_OUTPUT" | grep "publicBase64Key" | sed 's/│//g' | awk '{print $2}' | xargs)
FAUCET_KEY_SCHEME=$(echo "$FAUCET_OUTPUT" | grep "keyScheme" | sed 's/│//g' | awk '{print $2}' | xargs)
FAUCET_FLAG=$(echo "$FAUCET_OUTPUT" | grep "flag" | sed 's/│//g' | awk '{print $2}' | xargs)
FAUCET_MNEMONIC=$(echo "$FAUCET_OUTPUT" | grep "mnemonic" | sed 's/.*mnemonic[[:space:]]*//g' | xargs)
FAUCET_PEER_ID=$(echo "$FAUCET_OUTPUT" | grep "peerId" | sed 's/│//g' | awk '{print $2}' | xargs)

# Rename the generated .key file to faucet.key
mv "${FAUCET_ADDRESS}.key" "faucet.key" 2>/dev/null || true

# Return to genesis directory
cd "${GENESIS_DIR}"

# Calculate the maximum length for proper formatting
MAX_LEN=$(printf "%s\n%s\n%s\n%s\n%s\n%s\n%s" "faucet" "$FAUCET_ADDRESS" "$FAUCET_PUBLIC_KEY" "$FAUCET_KEY_SCHEME" "$FAUCET_FLAG" "$FAUCET_MNEMONIC" "$FAUCET_PEER_ID" | awk '{print length}' | sort -rn | head -1)
# Ensure minimum width of 77 characters for the value column
if [ "$MAX_LEN" -lt 77 ]; then
    MAX_LEN=77
fi
# Create the border line
BORDER_LINE=$(printf '─%.0s' $(seq 1 $MAX_LEN))

# Save in table format with proper padding
cat > "${GENESIS_DIR}/faucet/faucet_wallet_info.json" << EOL
╭─────────────────┬─${BORDER_LINE}─╮
│ alias           │ $(printf "%-${MAX_LEN}s" "faucet") │
│ mysAddress      │ $(printf "%-${MAX_LEN}s" "$FAUCET_ADDRESS") │
│ publicBase64Key │ $(printf "%-${MAX_LEN}s" "$FAUCET_PUBLIC_KEY") │
│ keyScheme       │ $(printf "%-${MAX_LEN}s" "$FAUCET_KEY_SCHEME") │
│ flag            │ $(printf "%-${MAX_LEN}s" "$FAUCET_FLAG") │
│ mnemonic        │ $(printf "%-${MAX_LEN}s" "$FAUCET_MNEMONIC") │
│ peerId          │ $(printf "%-${MAX_LEN}s" "$FAUCET_PEER_ID") │
╰─────────────────┴─${BORDER_LINE}─╯
EOL

echo -e "${GREEN}Faucet account generated!${NC}"
echo "Address: $FAUCET_ADDRESS"
echo "Key saved to: ${GENESIS_DIR}/faucet/faucet.key"
echo "Wallet information saved to: ${GENESIS_DIR}/faucet/faucet_wallet_info.json"
echo

# Step 2: Generate social-proof-foundation account address
echo -e "${YELLOW}Generating social-proof-foundation account address...${NC}"

# Create social-proof-foundation directory
mkdir -p "${GENESIS_DIR}/social-proof-foundation"

# Change to social-proof-foundation directory so the .key file is generated there
cd "${GENESIS_DIR}/social-proof-foundation"

SOCIAL_PROOF_FOUNDATION_OUTPUT=$(cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519)
SOCIAL_PROOF_FOUNDATION_ADDRESS=$(echo "$SOCIAL_PROOF_FOUNDATION_OUTPUT" | grep "mysAddress" | sed 's/│//g' | awk '{print $2}' | xargs)

# Extract key information
SPF_PUBLIC_KEY=$(echo "$SOCIAL_PROOF_FOUNDATION_OUTPUT" | grep "publicBase64Key" | sed 's/│//g' | awk '{print $2}' | xargs)
SPF_KEY_SCHEME=$(echo "$SOCIAL_PROOF_FOUNDATION_OUTPUT" | grep "keyScheme" | sed 's/│//g' | awk '{print $2}' | xargs)
SPF_FLAG=$(echo "$SOCIAL_PROOF_FOUNDATION_OUTPUT" | grep "flag" | sed 's/│//g' | awk '{print $2}' | xargs)
SPF_MNEMONIC=$(echo "$SOCIAL_PROOF_FOUNDATION_OUTPUT" | grep "mnemonic" | sed 's/.*mnemonic[[:space:]]*//g' | xargs)
SPF_PEER_ID=$(echo "$SOCIAL_PROOF_FOUNDATION_OUTPUT" | grep "peerId" | sed 's/│//g' | awk '{print $2}' | xargs)

# Rename the generated .key file to social-proof-foundation.key
mv "${SOCIAL_PROOF_FOUNDATION_ADDRESS}.key" "social-proof-foundation.key" 2>/dev/null || true

# Return to genesis directory
cd "${GENESIS_DIR}"

# Calculate the maximum length for proper formatting
MAX_LEN=$(printf "%s\n%s\n%s\n%s\n%s\n%s\n%s" "social-proof-foundation" "$SOCIAL_PROOF_FOUNDATION_ADDRESS" "$SPF_PUBLIC_KEY" "$SPF_KEY_SCHEME" "$SPF_FLAG" "$SPF_MNEMONIC" "$SPF_PEER_ID" | awk '{print length}' | sort -rn | head -1)
# Ensure minimum width of 77 characters for the value column
if [ "$MAX_LEN" -lt 77 ]; then
    MAX_LEN=77
fi
# Create the border line
BORDER_LINE=$(printf '─%.0s' $(seq 1 $MAX_LEN))

# Save in table format with proper padding
cat > "${GENESIS_DIR}/social-proof-foundation/social-proof-foundation_wallet_info.json" << EOL
╭─────────────────┬─${BORDER_LINE}─╮
│ alias           │ $(printf "%-${MAX_LEN}s" "social-proof-foundation") │
│ mysAddress      │ $(printf "%-${MAX_LEN}s" "$SOCIAL_PROOF_FOUNDATION_ADDRESS") │
│ publicBase64Key │ $(printf "%-${MAX_LEN}s" "$SPF_PUBLIC_KEY") │
│ keyScheme       │ $(printf "%-${MAX_LEN}s" "$SPF_KEY_SCHEME") │
│ flag            │ $(printf "%-${MAX_LEN}s" "$SPF_FLAG") │
│ mnemonic        │ $(printf "%-${MAX_LEN}s" "$SPF_MNEMONIC") │
│ peerId          │ $(printf "%-${MAX_LEN}s" "$SPF_PEER_ID") │
╰─────────────────┴─${BORDER_LINE}─╯
EOL

echo -e "${GREEN}Social Proof Foundation account generated!${NC}"
echo "Address: $SOCIAL_PROOF_FOUNDATION_ADDRESS"
echo "Key saved to: ${GENESIS_DIR}/social-proof-foundation/social-proof-foundation.key"
echo "Wallet information saved to: ${GENESIS_DIR}/social-proof-foundation/social-proof-foundation_wallet_info.json"
echo

# Step 3: Generate core-team account address
echo -e "${YELLOW}Generating core-team account address...${NC}"

# Create core-team directory
mkdir -p "${GENESIS_DIR}/core-team"

# Change to core-team directory so the .key file is generated there
cd "${GENESIS_DIR}/core-team"

CORE_TEAM_OUTPUT=$(cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519)
CORE_TEAM_ADDRESS=$(echo "$CORE_TEAM_OUTPUT" | grep "mysAddress" | sed 's/│//g' | awk '{print $2}' | xargs)

# Extract key information
CORE_PUBLIC_KEY=$(echo "$CORE_TEAM_OUTPUT" | grep "publicBase64Key" | sed 's/│//g' | awk '{print $2}' | xargs)
CORE_KEY_SCHEME=$(echo "$CORE_TEAM_OUTPUT" | grep "keyScheme" | sed 's/│//g' | awk '{print $2}' | xargs)
CORE_FLAG=$(echo "$CORE_TEAM_OUTPUT" | grep "flag" | sed 's/│//g' | awk '{print $2}' | xargs)
CORE_MNEMONIC=$(echo "$CORE_TEAM_OUTPUT" | grep "mnemonic" | sed 's/.*mnemonic[[:space:]]*//g' | xargs)
CORE_PEER_ID=$(echo "$CORE_TEAM_OUTPUT" | grep "peerId" | sed 's/│//g' | awk '{print $2}' | xargs)

# Rename the generated .key file to core-team.key
mv "${CORE_TEAM_ADDRESS}.key" "core-team.key" 2>/dev/null || true

# Return to genesis directory
cd "${GENESIS_DIR}"

# Calculate the maximum length for proper formatting
MAX_LEN=$(printf "%s\n%s\n%s\n%s\n%s\n%s\n%s" "core-team" "$CORE_TEAM_ADDRESS" "$CORE_PUBLIC_KEY" "$CORE_KEY_SCHEME" "$CORE_FLAG" "$CORE_MNEMONIC" "$CORE_PEER_ID" | awk '{print length}' | sort -rn | head -1)
# Ensure minimum width of 77 characters for the value column
if [ "$MAX_LEN" -lt 77 ]; then
    MAX_LEN=77
fi
# Create the border line
BORDER_LINE=$(printf '─%.0s' $(seq 1 $MAX_LEN))

# Save in table format with proper padding
cat > "${GENESIS_DIR}/core-team/core-team_wallet_info.json" << EOL
╭─────────────────┬─${BORDER_LINE}─╮
│ alias           │ $(printf "%-${MAX_LEN}s" "core-team") │
│ mysAddress      │ $(printf "%-${MAX_LEN}s" "$CORE_TEAM_ADDRESS") │
│ publicBase64Key │ $(printf "%-${MAX_LEN}s" "$CORE_PUBLIC_KEY") │
│ keyScheme       │ $(printf "%-${MAX_LEN}s" "$CORE_KEY_SCHEME") │
│ flag            │ $(printf "%-${MAX_LEN}s" "$CORE_FLAG") │
│ mnemonic        │ $(printf "%-${MAX_LEN}s" "$CORE_MNEMONIC") │
│ peerId          │ $(printf "%-${MAX_LEN}s" "$CORE_PEER_ID") │
╰─────────────────┴─${BORDER_LINE}─╯
EOL

echo -e "${GREEN}Core Team account generated!${NC}"
echo "Address: $CORE_TEAM_ADDRESS"
echo "Key saved to: ${GENESIS_DIR}/core-team/core-team.key"
echo "Wallet information saved to: ${GENESIS_DIR}/core-team/core-team_wallet_info.json"
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
  # Chain start timestamp current time + 0 hour (in milliseconds since epoch)
  chain_start_timestamp_ms: 1768557600000 # $(( $(date +%s) * 1000 ))

  # Protocol version
  protocol_version: 75  # Latest version

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
  # Default: 31,537 MySo = 31,537,208,455,986 MIST
  # stake_subsidy_initial_distribution_amount: 31537208455986
  stake_subsidy_initial_apy_bps: 2000

  # Maximum APY cap (in basis points). Effective APY will never exceed this.
  stake_subsidy_max_apy_bps: 2500

  # Minimum APY floor (in basis points). Effective APY will never go below this.
  stake_subsidy_min_apy_bps: 100

  # Target duration for subsidy pool in years (e.g., 10).
  # Used to calculate stake-aware APY reduction to ensure pool sustainability.
  stake_subsidy_intended_duration_years: 10

  # Number of epochs before decreasing the subsidy amount
  # Default: 15 epochs
  stake_subsidy_period_length: 12

  # Rate at which subsidy decreases at end of each period (in basis points)
  # 40 basis points = 0.40%
  stake_subsidy_decrease_rate: 100

# Accounts to add tokens to
accounts:
  # Faucet
  - address: "$FAUCET_ADDRESS"
    gas_amounts:
      - 1000000000000000 # 1,000,000 MySo
  # Social Proof Foundation
  - address: "$SOCIAL_PROOF_FOUNDATION_ADDRESS"
    gas_amounts:
      - 646200000000000000 # 646,200,000 MySo (24% from the Social Proof Foundation + 51% from the Community - 3 mill from Validators - 150 mill from staking subsidy)
  # Core Team
  - address: "$CORE_TEAM_ADDRESS"
    gas_amounts:
      - 249000000000000000 # 249,000,000 MySo (25% = 12.5% core + 12.5% marketing - 1 mill faucet)

validator_config_info:
EOL

# Generate dynamic validator configuration
for i in {0..2}; do
  # Parse the ports for this validator
  IFS=',' read -r NETWORK_PORT P2P_PORT NARWHAL_PRIMARY_PORT NARWHAL_WORKER_PORT CONSENSUS_PORT <<< "${VALIDATOR_PORTS[$i]}"
  
  cat >> "${GENESIS_DIR}/genesis_config.new.yaml" << EOL
  # Validator $(($i + 1)) (dynamic ports)
  - name: "MySo Validator $(($i + 1))"
    network_address: "/ip4/$BASE_IP/tcp/$NETWORK_PORT/http"
    p2p_address: "/ip4/$BASE_IP/udp/$P2P_PORT"
    narwhal_primary_address: "/ip4/$BASE_IP/udp/$NARWHAL_PRIMARY_PORT"
    narwhal_worker_address: "/ip4/$BASE_IP/udp/$NARWHAL_WORKER_PORT"
    consensus_address: "/ip4/$BASE_IP/tcp/$CONSENSUS_PORT/http"
    gas_price: 1
    commission_rate: 250
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
Validator $(($i + 1)):
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

echo -e "${GREEN}All account addresses generated and configuration updated!${NC}"
echo
echo "Generated accounts:"
echo "  Faucet: $FAUCET_ADDRESS"
echo "  Social Proof Foundation: $SOCIAL_PROOF_FOUNDATION_ADDRESS"
echo "  Core Team: $CORE_TEAM_ADDRESS"
echo
echo "Port assignments:"
for i in {0..2}; do
  IFS=',' read -r NETWORK_PORT P2P_PORT NARWHAL_PRIMARY_PORT NARWHAL_WORKER_PORT CONSENSUS_PORT <<< "${VALIDATOR_PORTS[$i]}"
  echo "  Validator $(($i + 1)): Network=$NETWORK_PORT, P2P=$P2P_PORT, Primary=$NARWHAL_PRIMARY_PORT, Worker=$NARWHAL_WORKER_PORT, Consensus=$CONSENSUS_PORT"
done
echo
echo "Base IP: $BASE_IP (set BASE_IP environment variable to override)"
echo
echo "Next step: Run myso genesis -f --with-faucet --committee-size 3 --from-config genesis_config.yaml"
echo "Note: Protocol, network, and worker keys will be automatically generated during genesis creation." 
