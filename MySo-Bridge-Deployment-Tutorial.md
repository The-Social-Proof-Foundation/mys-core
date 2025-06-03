# MySo Bridge Deployment Tutorial

This tutorial will guide you through deploying the MySo Bridge system, including Solidity smart contracts to Base Sepolia testnet using Foundry/Forge, setting up a PostgreSQL database, and deploying the Bridge Node server to Railway.

## Prerequisites

- Foundry/Forge (already set up)
- Rust and Cargo (already set up)
- Git
- Railway CLI (already installed)
- Base Sepolia testnet ETH for gas fees
- PostgreSQL database access

## Part 1: Smart Contract Deployment to Base Sepolia using Forge

### Step 1: Environment Setup

1. **Navigate to the EVM contracts directory:**
```bash
cd bridge/evm
```

2. **The Foundry project is already configured with:**
   - ✅ `foundry.toml` with Base Sepolia network configuration
   - ✅ OpenZeppelin contracts and dependencies via Soldeer
   - ✅ Deployment script `script/deploy_bridge.s.sol`
   - ✅ Base Sepolia configuration in `deploy_configs/84532.json`

3. **Create your environment variables:**
```bash
# Create .env file in bridge/evm directory
touch .env
```

Add the following to your `.env` file:
```bash
# Base Sepolia Configuration
BASE_SEPOLIA_RPC_URL=https://sepolia.base.org
BASESCAN_API_KEY=your_basescan_api_key_here

# Deployment wallet (use a dedicated deployment wallet)
PRIVATE_KEY=your_private_key_here
```

### Step 2: Update Dependencies and Build

1. **Update Forge dependencies:**
```bash
forge soldeer update
```

2. **Build contracts:**
```bash
forge build
```

### Step 3: Deploy Bridge Contracts

1. **Deploy to Base Sepolia:**
```bash
./deploy.sh
```

2. **Save the deployed contract addresses** (you'll need these for the bridge node):
```
MysBridge: 0x196D0E3f7b50F29531d1bd006AB3D4c2CB731DC6
BridgeCommittee: 0x2e37B4ffCE8B1F00dFD7234f8D2637715A473628
BridgeConfig: 0x93e073F0bE555e3CAA1c9cE3e7d825496f2a5f02
BridgeLimiter: 0x5B5691521f695c0D5743Cc7D786821b5Fda2718f
BridgeVault: 0xBCD346797E5DEc586B0c4Fa19b82a4E2cAd5DD67
USDC Token: 0x0E958cE89b85Ec7662Bb34f4ff5A4E9B1E7F1A41
```

## Part 2: PostgreSQL Database Setup

### Step 1: Create Database Tables

The MySo Bridge requires several database tables to track bridge events, signatures, and state. We've created migration files that will be automatically applied when deploying to Railway.

**Migration files created:**
- `001_initial_bridge_schema.sql` - Core bridge tables
- `002_bridge_tokens_and_limits.sql` - Token configuration and limits

**Key tables:**
- `bridge_events` - Cross-chain bridge events
- `bridge_signatures` - Authority signatures for bridge actions  
- `bridge_actions` - Pending and completed bridge actions
- `bridge_tokens` - Supported token configurations
- `bridge_chain_limits` - Transfer limits per chain route
- `bridge_committee_config` - Committee and system configuration

### Step 2: Database Schema Features

- ✅ **Automatic migrations** on Railway deployment
- ✅ **USDC and ETH** token configuration pre-loaded
- ✅ **Base Sepolia ↔ Mys** chain limits configured
- ✅ **Event tracking** with duplicate prevention
- ✅ **Performance indexes** for fast queries
- ✅ **Updated timestamp triggers** for audit trails

## Part 3: Railway Deployment

### Step 1: Prepare Bridge Node Files

1. **Navigate to the bridge crate:**
```bash
cd crates/mys-bridge
```

2. **Verify deployment files are created:**
```
✅ Dockerfile - Multi-stage build with migrations
✅ railway.toml - Railway configuration
✅ docker-compose.yml - Local development setup
✅ prometheus.yml - Monitoring configuration
✅ migrations/ - Database migration files
✅ src/bin/migrate.rs - Migration runner binary
```

### Step 2: Create Railway Project

1. **Initialize Railway project:**
```bash
railway login
railway init
```

2. **Add PostgreSQL database:**
```bash
railway add postgresql
```

### Step 3: Configure Environment Variables

**Set these environment variables in Railway Dashboard:**

**Database Configuration:**
```bash
DATABASE_URL=${{Postgres.DATABASE_URL}}
DB_POOL_SIZE=10
DB_CONNECTION_TIMEOUT=30
```

**Bridge Node Configuration:**
```bash
MYS_RPC_URL=https://fullnode.mainnet.sui.io:443
ETH_RPC_URL=https://sepolia.base.org
BRIDGE_NODE_PORT=8080
BRIDGE_METRICS_PORT=9090
```

**Contract Addresses (Base Sepolia):**
```bash
BRIDGE_CONTRACT_ADDRESS=0x196D0E3f7b50F29531d1bd006AB3D4c2CB731DC6
BRIDGE_COMMITTEE_ADDRESS=0x2e37B4ffCE8B1F00dFD7234f8D2637715A473628
BRIDGE_CONFIG_ADDRESS=0x93e073F0bE555e3CAA1c9cE3e7d825496f2a5f02
BRIDGE_LIMITER_ADDRESS=0x5B5691521f695c0D5743Cc7D786821b5Fda2718f
BRIDGE_VAULT_ADDRESS=0xBCD346797E5DEc586B0c4Fa19b82a4E2cAd5DD67
```

**Bridge Authority Configuration:**
```bash
BRIDGE_AUTHORITY_KEY=your_committee_member_private_key
BRIDGE_COMMITTEE_MEMBER=true
```

**Network Configuration:**
```bash
SOURCE_CHAIN_ID=84532
TARGET_CHAIN_ID=1
```

**Monitoring and Performance:**
```bash
LOG_LEVEL=info
RUST_LOG=mys_bridge=info,tower_http=debug
METRICS_ENABLED=true
PROMETHEUS_PORT=9090
SYNC_BATCH_SIZE=100
SYNC_INTERVAL_MS=5000
MAX_CONCURRENT_REQUESTS=10
```

**Security and Features:**
```bash
ENABLE_CORS=true
TRUSTED_PROXY_COUNT=1
ENABLE_EMERGENCY_PAUSE=true
ENABLE_RATE_LIMITING=true
ENABLE_TRANSFER_LIMITS=true
```

### Step 4: Deploy to Railway

1. **Deploy the service:**
```bash
railway up
```

2. **Monitor deployment:**
```bash
railway logs
```

3. **Check service status:**
```bash
railway status
```

### Step 5: Verify Deployment

1. **Check health endpoint:**
```bash
curl https://your-app.railway.app/health
```

2. **Check metrics endpoint:**
```bash
curl https://your-app.railway.app/metrics
```

3. **View database tables:**
```bash
railway connect postgresql
\dt
```

## Part 4: Local Development Setup

### Step 1: Run Local Environment

1. **Start local services:**
```bash
cd crates/mys-bridge
docker-compose up -d
```

2. **Check services:**
```bash
docker-compose ps
```

**Available services:**
- **Bridge Node**: http://localhost:8080
- **PostgreSQL**: localhost:5432
- **Prometheus**: http://localhost:9091
- **Grafana**: http://localhost:3000 (admin/admin)
- **Redis**: localhost:6379

### Step 2: Test Bridge Locally

1. **Check bridge health:**
```bash
curl http://localhost:8080/health
```

2. **View bridge metrics:**
```bash
curl http://localhost:8080/metrics
```

3. **Check database:**
```bash
docker exec -it myso-bridge-postgres psql -U bridge_user -d myso_bridge -c "\dt"
```

## Part 5: Production Monitoring

### Step 1: Set Up Monitoring

**Grafana Dashboards:**
- Bridge transaction volumes
- Committee signature rates
- Cross-chain transfer latency
- Error rates and alerts

**Prometheus Metrics:**
- `bridge_events_processed_total`
- `bridge_signatures_collected_total`
- `bridge_transfer_volume_usd`
- `bridge_committee_health_score`

### Step 2: Health Checks

**Automated monitoring endpoints:**
- `/health` - Basic service health
- `/metrics` - Prometheus metrics
- `/status` - Bridge committee status
- `/version` - Service version info

### Step 3: Alerting Rules

Configure alerts for:
- ❌ Bridge node disconnection
- ❌ Database connection failures
- ❌ Committee member offline
- ❌ High transaction failure rate
- ⚠️ Transfer volume approaching limits
- ⚠️ Long processing delays

## Troubleshooting

### Common Issues

1. **Migration Failures:**
   - Check DATABASE_URL format
   - Verify PostgreSQL connection
   - Review migration logs in Railway

2. **Contract Connection Issues:**
   - Verify contract addresses are correct
   - Check Base Sepolia RPC connectivity
   - Validate private key has committee permissions

3. **Performance Issues:**
   - Increase DB_POOL_SIZE
   - Adjust SYNC_BATCH_SIZE
   - Monitor memory usage in Railway

### Logs and Debugging

```bash
# Railway logs
railway logs --tail

# Local logs
docker-compose logs bridge-node -f

# Database queries
railway connect postgresql
SELECT * FROM bridge_events ORDER BY created_at DESC LIMIT 10;
```

## Security Considerations

1. **Private Key Management:**
   - Use dedicated committee member keys
   - Store in Railway environment variables only
   - Rotate keys regularly
   - Monitor key usage

2. **Network Security:**
   - Enable CORS appropriately
   - Use HTTPS in production
   - Set trusted proxy count
   - Rate limit API endpoints

3. **Database Security:**
   - Use strong PostgreSQL passwords
   - Enable connection encryption
   - Regular backup strategy
   - Monitor for suspicious queries

## Next Steps

1. **Production Hardening:**
   - Set up backup strategies
   - Implement comprehensive monitoring
   - Configure proper alerting
   - Security audit

2. **Scaling:**
   - Add multiple bridge node instances
   - Implement load balancing
   - Database read replicas
   - Caching strategies

3. **Maintenance:**
   - Regular dependency updates
   - Security patches
   - Performance optimization
   - Committee management

---

🎉 **Congratulations!** Your MySo Bridge is now deployed and operational between Base Sepolia and MySo networks, supporting USDC transfers with full monitoring and database persistence. 