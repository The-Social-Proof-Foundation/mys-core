# Railway Deployment Guide

This guide walks you through deploying the mys-price-oracle to Railway using environment variables for configuration.

## Prerequisites

1. [Railway CLI](https://docs.railway.app/develop/cli) installed
2. Railway account
3. Bridge server URL and authentication credentials

## Quick Deploy

### 1. Clone and Navigate

```bash
git clone <your-repo>
cd mys-core/crates/mys-price-oracle
```

### 2. Initialize Railway Project

```bash
railway login
railway init
```

### 3. Set Required Environment Variables

**Required Variables (must be set as Railway secrets):**

```bash
# Bridge server configuration
railway variables set MYS_ORACLE_SERVER_URL="https://your-bridge-server.com"

# Authentication (choose one)
railway variables set MYS_ORACLE_AUTH_API_KEY="your-secure-api-key"
# OR
railway variables set MYS_ORACLE_AUTH_HMAC_SECRET="your-hmac-secret"
```

**Optional Variables (defaults provided in railway.toml):**

```bash
# Override defaults if needed
railway variables set MYS_ORACLE_CHAIN_ID="8453"  # Base network
railway variables set MYS_ORACLE_TOKEN_ID="1"
railway variables set MYS_ORACLE_UPDATE_INTERVAL="30s"
railway variables set MYS_ORACLE_PRICE_CHANGE_THRESHOLD="0.02"

# Custom token configuration
railway variables set MYS_ORACLE_SOURCE_TOKEN_ADDRESS="0xYourTokenAddress"
```

### 4. Deploy

```bash
railway up
```

## Environment Variables Reference

### Core Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `MYS_ORACLE_SERVER_URL` | Bridge server URL | ✅ | - |
| `MYS_ORACLE_CHAIN_ID` | Blockchain network ID | ❌ | `8453` (Base) |
| `MYS_ORACLE_TOKEN_ID` | Token ID on bridge | ❌ | `1` |
| `MYS_ORACLE_UPDATE_INTERVAL` | Update frequency | ❌ | `30s` |
| `MYS_ORACLE_PRICE_CHANGE_THRESHOLD` | Min price change % | ❌ | `0.02` (2%) |

### Data Source Configuration

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `MYS_ORACLE_SOURCE_TYPE` | `graphql` or `rest_api` | ❌ | `graphql` |
| `MYS_ORACLE_SOURCE_URL` | Data source URL | ❌ | Uniswap V3 Base |
| `MYS_ORACLE_SOURCE_TOKEN_ADDRESS` | Token contract address | ❌ | Pre-configured |
| `MYS_ORACLE_SOURCE_POOL_FEE_TIER` | Uniswap pool fee tier | ❌ | `3000` |

### Authentication (Required)

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `MYS_ORACLE_AUTH_API_KEY` | API key for authentication | ❌* | - |
| `MYS_ORACLE_AUTH_HMAC_SECRET` | HMAC secret for authentication | ❌* | - |

*One of these must be set for production deployment*

### Advanced Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `MYS_ORACLE_RETRY_MAX_ATTEMPTS` | Max retry attempts | `3` |
| `MYS_ORACLE_RETRY_INITIAL_DELAY` | Initial retry delay | `100ms` |
| `MYS_ORACLE_RETRY_MAX_DELAY` | Max retry delay | `30s` |
| `MYS_ORACLE_RETRY_MULTIPLIER` | Retry backoff multiplier | `2.0` |
| `MYS_ORACLE_VALIDATION_MIN_PRICE_USD` | Minimum valid price | `0.000001` |
| `MYS_ORACLE_VALIDATION_MAX_PRICE_USD` | Maximum valid price | `1000000` |
| `MYS_ORACLE_VALIDATION_MAX_PRICE_DEVIATION_PERCENT` | Max price change % | `50` |
| `MYS_ORACLE_MONITORING_METRICS_PORT` | Prometheus metrics port | `9090` |
| `MYS_ORACLE_PERSISTENCE_DATABASE_PATH` | Database file path | `/app/data/oracle_state.db` |

## Configuration Examples

### Basic Setup (Recommended)

```bash
# Core configuration
railway variables set MYS_ORACLE_SERVER_URL="https://your-bridge.com"
railway variables set MYS_ORACLE_AUTH_API_KEY="$(openssl rand -hex 32)"

# Deploy with defaults
railway up
```

### Custom Token

```bash
# Configure for different token
railway variables set MYS_ORACLE_SOURCE_TOKEN_ADDRESS="0x1234567890123456789012345678901234567890"
railway variables set MYS_ORACLE_CHAIN_ID="1"  # Ethereum mainnet
railway variables set MYS_ORACLE_SOURCE_URL="https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3"

railway up
```

### High-Security Setup

```bash
# Use HMAC authentication
railway variables set MYS_ORACLE_AUTH_HMAC_SECRET="$(openssl rand -hex 64)"

# Tighter price validation
railway variables set MYS_ORACLE_VALIDATION_MAX_PRICE_DEVIATION_PERCENT="10"

# More frequent updates
railway variables set MYS_ORACLE_UPDATE_INTERVAL="15s"
railway variables set MYS_ORACLE_PRICE_CHANGE_THRESHOLD="0.01"

railway up
```

## Monitoring

### Health Checks

Railway automatically monitors the health endpoint:
- **URL**: `https://your-app.railway.app/health`
- **Timeout**: 300 seconds
- **Restart Policy**: On failure (max 10 retries)

### Logs

View logs in real-time:

```bash
railway logs
```

### Metrics

Access Prometheus metrics:
- **URL**: `https://your-app.railway.app:9090/metrics`

## Troubleshooting

### Deployment Issues

1. **Build Failures**
   ```bash
   # Check build logs
   railway logs --deployment
   ```

2. **Missing Environment Variables**
   ```bash
   # List current variables
   railway variables
   
   # Set missing variables
   railway variables set MYS_ORACLE_SERVER_URL="https://your-bridge.com"
   ```

3. **Health Check Failures**
   ```bash
   # Check application logs
   railway logs
   
   # Validate configuration
   railway run mys-price-oracle --env --validate-config
   ```

### Runtime Issues

1. **Authentication Failures**
   - Verify `MYS_ORACLE_AUTH_API_KEY` or `MYS_ORACLE_AUTH_HMAC_SECRET` is set
   - Check bridge server logs for authentication errors

2. **Price Fetch Failures**
   - Verify token address format and network
   - Check GraphQL endpoint accessibility

3. **Database Issues**
   - Ensure `/app/data` directory has write permissions
   - Check Railway volume persistence settings

### Debug Mode

Enable debug logging:

```bash
railway variables set RUST_LOG="debug,mys_price_oracle=trace"
railway up
```

## Production Checklist

- [ ] **Authentication**: API key or HMAC secret configured
- [ ] **Monitoring**: Health checks passing
- [ ] **Validation**: Price bounds and deviation limits set
- [ ] **Persistence**: Database path configured for Railway volumes
- [ ] **Security**: Using HTTPS for all external connections
- [ ] **Observability**: Metrics collection enabled
- [ ] **Backup**: Bridge nonce and price state backup strategy

## Support

For Railway-specific issues:
- [Railway Documentation](https://docs.railway.app/)
- [Railway Community](https://discord.gg/railway)

For oracle configuration issues:
- Check the main README.md
- Review application logs with correlation IDs
- Monitor health check endpoints

## Advanced: Custom Dockerfile

If you need to customize the build process, create a custom Dockerfile:

```dockerfile
# Custom build with additional dependencies
FROM rust:1.75-slim as builder
RUN apt-get update && apt-get install -y your-dependencies
# ... rest of build process

FROM debian:bookworm-slim
# ... custom runtime setup
CMD ["mys-price-oracle", "--env"]
```

Update `railway.toml`:

```toml
[build]
builder = "DOCKERFILE"
dockerfilePath = "Dockerfile"
``` 