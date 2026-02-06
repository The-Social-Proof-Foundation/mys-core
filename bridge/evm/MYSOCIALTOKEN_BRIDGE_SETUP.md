# MySocialToken Bridge Integration Guide

This document outlines the steps to integrate the official MySocialToken (deployed at `0xFdD6013Bf2757018D8c087244f03e5a521B2d3B7` on Base) with the MYS-EVM bridge.

## Architecture Overview

We're using an adapter pattern to integrate without modifying the MySocialToken:

1. Deploy a `MySocialTokenBridgeAdapter` contract owned by the token owner
2. The adapter is granted permission to call mint/burn on MySocialToken (via owner privileges)
3. The adapter authorizes the bridge to call its functions
4. The bridge is configured with the adapter address during initialization (Token ID 0)
5. The bridge interacts with the adapter for mint/burn operations instead of vault transfers

This design preserves existing token functionality while enabling secure bidirectional bridging.

**Key Design Decision**: Token ID 0 is reserved for the native MYS token, which uses the adapter for mint/burn instead of vault transfers.

## Setup Steps

### 1. Deploy the Adapter Contract (Token Owner)

**IMPORTANT**: This must be done BEFORE deploying the bridge, as the adapter address is required during bridge initialization.

```bash
# Export environment variables
export PRIVATE_KEY="your_token_owner_private_key"
export MYSOCIAL_TOKEN_ADDRESS="0xFdD6013Bf2757018D8c087244f03e5a521B2d3B7"

# Deploy adapter (without bridge address since bridge isn't deployed yet)
cd bridge/evm
forge script script/DeployMySocialTokenAdapter.s.sol \
  --rpc-url $BASE_RPC_URL \
  --broadcast \
  --verify
```

**Save the deployed adapter address** - you'll need it for the next steps.

### 2. Update Bridge Configuration (Before Bridge Deployment)

Update `bridge/evm/deploy_configs/84532.json`:

```json
{
  "supportedTokens": [
    "YOUR_ADAPTER_ADDRESS_HERE",  // Token ID 0 = MYS adapter
    "0x0000000000000000000000000000000000000001",
    // ... other tokens
  ],
  "tokenIds": [0, 1, 2, 3, 4],
  "tokenPrices": [100000000, ...],  // Set appropriate price for MYS
  "mysDecimals": [9, 8, 8, 6, 6],   // MYS uses 9 decimals on Mys chain
  "mySocialTokenAdapter": "YOUR_ADAPTER_ADDRESS_HERE"
}
```

### 3. Deploy the Bridge

```bash
# Deploy bridge with adapter address in configuration
cd bridge/evm
forge script script/deploy_bridge.s.sol \
  --rpc-url $BASE_RPC_URL \
  --broadcast \
  --verify
```

The bridge will automatically be configured with the adapter address during initialization.

### 4. Authorize the Bridge in the Adapter (Token Owner)

After the bridge is deployed, authorize it in the adapter:

```bash
# Authorize the bridge to use the adapter
export BRIDGE_ADDRESS="your_deployed_bridge_address"

cast send \
  --rpc-url $BASE_RPC_URL \
  --private-key $TOKEN_OWNER_PRIVATE_KEY \
  $ADAPTER_ADDRESS \
  "setAuthorizedBridge(address,bool)" \
  $BRIDGE_ADDRESS \
  true
```

## Verification Steps

### 1. Verify Adapter Ownership and Configuration

```bash
# Check that the adapter is owned by the token owner
cast call --rpc-url $BASE_RPC_URL $ADAPTER_ADDRESS "owner()"

# Check that the adapter points to the correct token
cast call --rpc-url $BASE_RPC_URL $ADAPTER_ADDRESS "mySocialToken()"

# Check that the bridge is authorized in the adapter
cast call --rpc-url $BASE_RPC_URL $ADAPTER_ADDRESS "authorizedBridges(address)" $BRIDGE_ADDRESS

# Get list of all authorized bridges
cast call --rpc-url $BASE_RPC_URL $ADAPTER_ADDRESS "getAuthorizedBridges()"
```

### 2. Verify Bridge Configuration

```bash
# Check that the bridge config has the adapter address
cast call --rpc-url $BASE_RPC_URL $BRIDGE_CONFIG_ADDRESS "mySocialTokenAdapter()"

# Check that token ID 0 (MYS) returns the adapter address
cast call --rpc-url $BASE_RPC_URL $BRIDGE_CONFIG_ADDRESS "tokenAddressOf(uint8)" 0

# Check MYS token price (should match config)
cast call --rpc-url $BASE_RPC_URL $BRIDGE_CONFIG_ADDRESS "tokenPriceOf(uint8)" 0

# Check MYS decimal configuration (should be 9)
cast call --rpc-url $BASE_RPC_URL $BRIDGE_CONFIG_ADDRESS "tokenMysDecimalOf(uint8)" 0
```

## Testing the Integration

Perform small test transactions in both directions:

### MYS → Base:
1. Send native MYS tokens from MYS chain to Base using the bridge
2. Monitor events: `TokensDeposited` on MYS chain and `TokensMinted` on the adapter
3. Verify recipient balance of MySocialToken increased on Base

### Base → MYS:
1. Approve the bridge contract to spend your MySocialToken
2. Call `bridgeMYS(amount, recipientAddress)` on the bridge
3. Monitor events: `TokensBurned` on the adapter and bridge events on MYS chain
4. Verify native MYS tokens are received on the recipient address

## Security Features

1. **Emergency Controls**:
   - Token owner can call `revokeAllAuthorizations()` to revoke all bridge permissions instantly
   - Bridge configuration can be updated by authorized governance actions

2. **Access Control**:
   - Only the token owner can authorize/deauthorize bridges
   - Only authorized bridges can call mint/burn functions
   - Contract uses ReentrancyGuard to prevent reentrancy attacks

3. **Transparency**:
   - All authorizations are tracked and enumerable via `getAuthorizedBridges()`
   - All operations emit detailed events for off-chain monitoring

## Troubleshooting

| Issue | Possible Causes | Solutions |
|-------|----------------|-----------|
| Token minting fails | 1. Bridge not authorized<br>2. Token supply cap reached | 1. Check authorizations<br>2. Verify token supply |
| Token burning fails | 1. Insufficient allowance<br>2. Insufficient balance | 1. Approve tokens for bridge<br>2. Check user balance |
| Bridge transaction fails | Integration issue | Check bridge logs and events |

For support, contact the bridge team or open an issue in the repository.
