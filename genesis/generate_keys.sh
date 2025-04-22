#!/bin/bash
# Script to generate validator keys and faucet keys

# Set up colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Store the original directory
GENESIS_DIR=$(pwd)

echo -e "${GREEN}MySocial Genesis Key Generator${NC}"
echo "-----------------------------------"

# Step 1: Generate validator keys
echo -e "${YELLOW}Generating validator keys...${NC}"

for i in {1..3}; do
  echo -e "Creating keys for validator $i..."
  # Create directory if it doesn't exist
  mkdir -p "${GENESIS_DIR}/validator_keys/validator$i"
  
  # Change to the validator directory for key generation
  pushd "${GENESIS_DIR}/validator_keys/validator$i" > /dev/null
  
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
  echo "  Keys saved to validator_keys/validator$i/"
  echo
done

# Step 2: Generate faucet key
echo -e "${YELLOW}Generating faucet key...${NC}"

# Create the key in its own directory for cleaner organization
mkdir -p "${GENESIS_DIR}/faucet"
pushd "${GENESIS_DIR}/faucet" > /dev/null

cargo run --bin myso --manifest-path="${GENESIS_DIR}/../Cargo.toml" -- keytool generate ed25519 > faucet_output.txt

# Update token distribution CSV
FAUCET_ADDRESS=$(grep "mysAddress" faucet_output.txt | sed 's/│//g' | awk '{print $2}' | xargs)
FAUCET_MNEMONIC=$(grep "mnemonic" faucet_output.txt | sed 's/│//g' | awk '{$1=""; print $0}' | xargs)

echo "Faucet Address: $FAUCET_ADDRESS"
echo "Faucet Mnemonic: $FAUCET_MNEMONIC"

# Save faucet info for later use in Railway.app
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

# Copy faucet info to root
cp "${GENESIS_DIR}/faucet/faucet_info.txt" "${GENESIS_DIR}/"
cp "${GENESIS_DIR}/faucet/faucet.json" "${GENESIS_DIR}/"

# Update the token distribution CSV
echo "Updating token_distribution.csv with faucet address..."
sed -i.bak "s/0xFAUCET_ADDRESS_TO_BE_REPLACED/$FAUCET_ADDRESS/g" "${GENESIS_DIR}/token_distribution.csv"
rm -f "${GENESIS_DIR}/token_distribution.csv.bak"

# Clean up any remaining .key files in the root directory
echo -e "${YELLOW}Checking for any remaining key files...${NC}"
remaining_keys=$(find "${GENESIS_DIR}" -maxdepth 1 -name "0x*.key" | wc -l)
if [ $remaining_keys -gt 0 ]; then
  echo "Moving $remaining_keys remaining key files to the key_backup directory"
  mkdir -p "${GENESIS_DIR}/key_backup"
  mv "${GENESIS_DIR}"/0x*.key "${GENESIS_DIR}/key_backup/"
fi

echo -e "${GREEN}All keys generated successfully!${NC}"
echo
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Initialize genesis ceremony:"
echo "   ./init_genesis.sh"
echo "2. Add validators to the ceremony:"
echo "   ./add_validators.sh"
echo "3. Finalize genesis ceremony:"
echo "   ./finalize_genesis.sh"
echo
echo "Note: If you want custom token distribution, you will need to manually edit the"
echo "token-distribution-schedule file after running init_genesis.sh"
echo
echo "Remember to securely back up all generated keys!" 