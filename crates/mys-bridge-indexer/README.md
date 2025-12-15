## Overview

MySocial Bridge Indexer is a comprehensive system that indexes and provides access to cross-chain bridge transactions between MySocial and Ethereum networks. The system consists of:

1. **Bridge Indexer**: Scans and indexes bridge transactions from both MySocial and Ethereum blockchains
2. **Bridge Node API**: Provides HTTP endpoints for bridge operations and governance actions
3. **Database Storage**: Stores indexed bridge data in PostgreSQL for fast querying

## Bridge Transaction Flow

The bridge enables seamless token transfers between MySocial and Ethereum chains through a secure validator-based system:

### From Ethereum to MySocial
1. **Deposit**: User calls `bridgeERC20()` or `bridgeETH()` on Ethereum bridge contract
2. **Approval**: Bridge validators approve the transaction with signatures
3. **Claim**: User calls `claim_token()` or `claim_mys_token()` on MySocial to receive tokens

### From MySocial to Ethereum
1. **Deposit**: User calls `send_token()` or `send_mys_token()` on MySocial bridge contract
2. **Approval**: Bridge validators approve the transaction with signatures
3. **Claim**: User calls `transferBridgedTokensWithSignatures()` on Ethereum to receive tokens

## Get Binary

```bash
cargo build --bin bridge-indexer --release
```

The pre-built Docker image for Bridge Indexer can be found in `socialproof/mys-tools:{SHA}`

## Run Binary

```
bridge-indexer --config-path config.yaml
```

## Config

```yaml
---
remote_store_url: https://checkpoints.mainnet.mysocial.network
eth_rpc_url: {eth rpc url}
mys_rpc_url: {mys rpc url}

concurrency: 500
checkpoints_path: {path-for-checkpoints}

eth_mys_bridge_contract_address: 0xb6868bE0717482c435cf8F2C20c425Cbae09a9cE # <-- mainnet, 0xAE68F87938439afEEDd6552B0E83D2CbC2473623 for testnet
metric_port: {port to export metrics}

mys_bridge_genesis_checkpoint: 55455583 # <-- mainnet, 43917829 for testnet
# genesis block number for eth
eth_bridge_genesis_block: 20811249 # <-- mainnet, 5997013 for testnet

eth_ws_url: {eth websocket url}
```

## Bridge Node HTTP API Endpoints

The Bridge Node exposes HTTP endpoints for bridge operations and governance actions. These endpoints are primarily used by validators for signing bridge transactions and governance actions.

> **Note**: The bridge node currently provides basic health check endpoints. Comprehensive monitoring and analytics endpoints for public consumption are not yet implemented.

### Base URL
```
http://<bridge-node-host>:<port>
```

### Health & Metadata
- **GET /ping** - Health check endpoint returning bridge node metadata
- **GET /metrics_pub_key** - Get the bridge node's metrics public key

### Bridge Status & Monitoring (GET Endpoints)

#### Health & Metadata
- **GET /ping** - Health check endpoint returning bridge node metadata
  ```json
  {
    "version": "1.2.3",
    "metrics_pubkey": {
      "scheme": "Ed25519",
      "pubkey_bytes": "base64_encoded_key"
    }
  }
  ```

- **GET /metrics_pub_key** - Get the bridge node's metrics public key
  ```json
  {
    "scheme": "Ed25519",
    "pubkey_bytes": "base64_encoded_key"
  }
  ```

### Integration Examples

#### Bridge Node Health Monitoring
```javascript
// Check bridge node health and connectivity
const health = await fetch('/ping');
if (health.version && health.metrics_pubkey) {
  console.log('Bridge node is healthy');
}
```

#### Validator Authentication
```javascript
// Get metrics public key for validator authentication
const pubKey = await fetch('/metrics_pub_key');
// Use for cryptographic verification of metrics data
```

## API Considerations

### Current API Characteristics

#### Available Endpoints
- **GET /ping** - Basic health check, returns bridge node metadata
- **GET /metrics_pub_key** - Returns the node's metrics public key for monitoring authentication

#### Request Patterns
Most bridge node endpoints are designed for validator operations:
- Signature requests for bridge transactions
- Governance action approvals
- Emergency operation handling

#### Error Responses
```json
{
  "error": "Internal server error",
  "details": "Something went wrong: BridgeError::Generic(\"Database connection failed\")"
}
```

#### Monitoring Integration
- Prometheus metrics available at standard `/metrics` endpoint (when enabled)
- Health checks via `/ping` endpoint
- Metrics authentication via `/metrics_pub_key` endpoint

### Future API Evolution

As the bridge ecosystem grows, additional monitoring and analytics endpoints may be added for:
- Public bridge status information
- Transfer analytics and statistics
- Vault balance monitoring
- Validator health status
- Transaction history and tracking

### Bridge Transaction Signing (Validator Operations)

#### Ethereum to MySocial Transactions
- **GET /sign/bridge_tx/eth/mys/{tx_hash}/{event_index}** - Request signatures for ETH→MYS bridge transaction
  - `tx_hash`: Ethereum transaction hash
  - `event_index`: Event index in the transaction

#### MySocial to Ethereum Transactions
- **GET /sign/bridge_tx/mys/eth/{tx_digest}/{event_index}** - Request signatures for MYS→ETH bridge transaction
  - `tx_digest`: MySocial transaction digest
  - `event_index`: Event index in the transaction

### Governance Actions (Validator Operations)

#### Emergency Operations
- **GET /sign/emergency_button/{chain_id}/{nonce}/{type}** - Request signatures for emergency pause/unpause
  - `chain_id`: Target chain ID (1=Ethereum, 12=MySocial)
  - `nonce`: Governance nonce
  - `type`: 0=pause, 1=unpause

#### Committee Management
- **GET /sign/update_committee_blocklist/{chain_id}/{nonce}/{type}/{keys}** - Request signatures for committee blocklist updates
  - `chain_id`: Target chain ID
  - `nonce`: Governance nonce
  - `type`: Blocklist action type
  - `keys`: Comma-separated list of public keys

#### Bridge Limits
- **GET /sign/update_limit/{chain_id}/{nonce}/{sending_chain_id}/{new_usd_limit}** - Request signatures for bridge limit updates
  - `chain_id`: Target chain ID
  - `nonce`: Governance nonce
  - `sending_chain_id`: Chain ID that sends tokens
  - `new_usd_limit`: New USD limit for transfers

#### Asset Prices
- **GET /sign/update_asset_price/{chain_id}/{nonce}/{token_id}/{new_usd_price}** - Request signatures for asset price updates
  - `chain_id`: Target chain ID
  - `nonce`: Governance nonce
  - `token_id`: Token ID (0=MYS, 1=USDT, 2=ETH, etc.)
  - `new_usd_price`: New USD price per token

#### Contract Upgrades
- **GET /sign/upgrade_evm_contract/{chain_id}/{nonce}/{proxy_address}/{new_impl_address}** - Request signatures for EVM contract upgrades
- **GET /sign/upgrade_evm_contract/{chain_id}/{nonce}/{proxy_address}/{new_impl_address}/{calldata}** - Contract upgrade with call data

#### Token Management
- **GET /sign/add_tokens_on_mys/{chain_id}/{nonce}/{native}/{token_ids}/{token_type_names}/{token_prices}** - Add tokens on MySocial chain
- **GET /sign/add_tokens_on_evm/{chain_id}/{nonce}/{native}/{token_ids}/{token_addresses}/{token_mys_decimals}/{token_prices}** - Add tokens on Ethereum chain

### Example API Usage

#### Requesting Bridge Transaction Approval
```bash
# Get signatures for an Ethereum deposit (ETH → MYS)
curl "http://localhost:8080/sign/bridge_tx/eth/mys/0x1234567890abcdef.../0"

# Get signatures for a MySocial deposit (MYS → ETH)
curl "http://localhost:8080/sign/bridge_tx/mys/eth/ABC123def456.../1"
```

#### Emergency Bridge Pause
```bash
# Pause bridge operations on Ethereum (chain_id=1)
curl "http://localhost:8080/sign/emergency_button/1/42/0"

# Unpause bridge operations
curl "http://localhost:8080/sign/emergency_button/1/43/1"
```

#### Update Bridge Limits
```bash
# Set $1M daily limit for MySocial → Ethereum transfers
curl "http://localhost:8080/sign/update_limit/1/44/12/1000000000"
```

## User Journey: Bridging Tokens

### Scenario: Alice wants to bridge MYS tokens from MySocial to Ethereum

**Step 1: Smart Wallet Connection**
Alice visits your bridge website and sees multiple connection options:

```
🌟 Recommended: Connect MySocial Wallet
   └─ Auto-detects tokens and bridge routes

🔗 Additional Wallets (Optional)
   ├─ Ethereum: For seamless claiming
   ├─ Solana: Future cross-chain support
   └─ Connect later when needed
```

Alice connects her MySocial wallet. The interface automatically detects:
- Available tokens (MYS, USDT, ETH, etc.)
- Balance information
- Supported bridge routes

**Pro Tip:** The bridge remembers connected wallets and only prompts for missing ones when needed.

**Step 2: Smart Bridge Selection**
Alice selects MYS token and enters amount (1000 MYS). The interface automatically:
- Detects bridge direction: MySocial → Ethereum
- Shows destination address field
- Calculates fees and estimated completion time

**Step 3: Enter Destination & Preferences**
- **Destination Address**: Alice enters her Ethereum address (0x742d...)
- **Claim Method**: Choose how to handle destination claiming:
  - 🔄 **Auto-Claim**: "Connect my Ethereum wallet when ready" (recommended)
  - 📧 **Email Notification**: "Email me when ready to claim"
  - 🛠️ **Manual Claim**: "I'll handle claiming myself"

**Step 4: Initiate Bridge**
Alice clicks "Bridge 1000 MYS" and approves the transaction in her MySocial wallet. The bridge contract locks her tokens securely.

**Step 5: Real-time Progress Tracking**
```
🚀 Bridge Initiated
   ├─ Deposit: 1000 MYS on MySocial ✅
   ├─ Security: Validator approval in progress (2/3 signed)
   └─ Estimated completion: ~3-5 minutes

💡 Pro Tip: You can close this tab - we'll notify you when ready!
```

**Progress Monitoring:**
The bridge frontend can monitor transaction status through:
- Direct blockchain queries for transaction confirmations
- Email/webhook notifications when transfers are ready to claim
- Integration with wallet notification systems
- Periodic status checks via bridge node health endpoints

**Step 6: Seamless Claiming Experience**
When validators approve the transfer, Alice gets notified. Depending on her preference:

**Auto-Claim (Recommended):**
```
🎉 Your bridge is ready!

"One-Click Claim" button appears
↓ Click
↓ Automatic wallet connection
↓ Instant claim completion
↓ Tokens arrive in 30 seconds
```

**Email Notification:**
```
📧 Email: "Your 1000 MYS bridge from MySocial to Ethereum is ready to claim"

[Claim Now] button → Opens bridge interface
[View Details] → Shows claim instructions
```

**Manual Claim:**
```
🔧 Advanced Options
├─ Transaction Hash: ABC123...
├─ Validator Signatures: Available for download
├─ Claim Contract: 0xb6868bE0717482c435cf8F2C20c425Cbae09a9cE
└─ Claim Function: transferBridgedTokensWithSignatures()
```

**Step 7: Completion & History**
```
✅ Bridge Complete!
├─ Sent: 1000 MYS on MySocial
├─ Received: 999.99 MYS on Ethereum (0.01 MYS fee)
├─ Transaction time: 4 minutes 32 seconds
└─ View in explorer: [MySocial Tx] [Ethereum Tx]
```

**Bridge History Dashboard:**
- All past bridges with status
- Claim pending notifications
- Gas fee optimization tips
- Multi-chain portfolio view

### Error Scenarios & Recovery

**Wallet Connection Issues**
```
🔗 Wallet Connection Failed
   ├─ Try: Refresh page and reconnect
   ├─ Alternative: Use wallet mobile app
   └─ Help: Check wallet extension settings
```

**Insufficient Balance**
```
💰 Insufficient MYS balance
   ├─ Required: 1000 MYS + 0.01 MYS fee
   ├─ Available: 950 MYS
   └─ Solution: Add more MYS to your wallet
```

**Bridge Paused**
```
⚠️ Bridge temporarily paused for maintenance
   ├─ Your transaction will be processed when bridge resumes
   └─ Monitor bridge status via health endpoints
```

**Network Congestion**
```
🐌 High network activity detected
   ├─ Estimated wait: 5-10 minutes
   └─ Transaction will process automatically
```

**Claim Transaction Failed**
```
❌ Claim transaction failed
   ├─ Check wallet balance and gas fees
   └─ Retry the claim operation
```

## Supported Tokens

| Token ID | Symbol | MySocial | Ethereum | Decimals |
|----------|--------|----------|-----------|----------|
| 0 | MYS | Native | 0x8a9e9Ad05010aD980a1d24b61bC2a099B13D42a2 | 9 |
| 1 | USDT | Bridged | 0x0000000000000000000000000000000000000001 | 8 |
| 2 | ETH | Bridged | 0x4200000000000000000000000000000000000006 | 8 |
| 3 | USDC | Bridged | 0x0000000000000000000000000000000000000003 | 6 |
| 4 | WBTC | Bridged | 0x0000000000000000000000000000000000000004 | 6 |

## Bridge Limits & Security

- **Daily Limits**: $1M USD per route (ETH↔MYS)
- **Validator Threshold**: 2/3 validator signatures required
- **Emergency Pause**: Governance can pause bridge operations
- **Transfer Limits**: Per-user limits to prevent abuse

## Monitoring & Analytics

The bridge indexer currently provides basic health monitoring. Future versions may include comprehensive monitoring endpoints for:

### Potential Future Endpoints
- Bridge operational status and health metrics
- Vault balance monitoring across chains
- Transfer volume and success rate analytics
- Validator committee health and connectivity
- Transaction status tracking and history
- Network performance and latency statistics
- Governance action monitoring
- Capacity and limit utilization tracking

### Current Capabilities
The bridge node exposes Prometheus metrics that can be scraped for monitoring data, including:
- Transaction processing rates
- Validator signature aggregation success/failure rates
- RPC call latencies and error rates
- Bridge action execution metrics
- Network connectivity status

### Metrics Available
The bridge exposes detailed metrics via Prometheus for operational monitoring:
- Request rates and latencies by endpoint type
- Transaction success/failure rates
- Validator response times and reliability
- Network RPC performance metrics
- Bridge action processing statistics

