# MySo Price Oracle

Automated price oracle service that monitors token prices and updates the MySo Bridge with real-time pricing data.

## Features

- 🔄 Automatic price fetching from multiple sources (GraphQL, REST API)
- 🎯 Configurable price change thresholds
- 🔒 Price validation and deviation limits
- 📊 Prometheus metrics export
- 🏥 Health check endpoints
- 💾 Persistent state management
- 🔁 Automatic retry with exponential backoff

## API Endpoints

The price oracle exposes the following HTTP endpoints:

### Health & Status Endpoints (Default Port: 8080)

#### GET `/health`
Basic liveness check - returns if service is running

**Response:**
```json
{
  "status": "ok",
  "timestamp": 1696435200
}
```

#### GET `/ready`
Readiness check - verifies database connectivity and service state

**Response (Healthy):**
```json
{
  "status": "ready",
  "timestamp": 1696435200,
  "database_accessible": true,
  "current_nonce": 42,
  "last_price": "0.0045"
}
```

**Response (Not Ready):**
```json
{
  "status": "not_ready",
  "timestamp": 1696435200,
  "database_accessible": false,
  "error": "Cannot access state database"
}
```

#### GET `/status`
Detailed service status with oracle state and metrics

**Response:**
```json
{
  "status": "ok",
  "timestamp": 1696435200,
  "uptime_seconds": 86400,
  "oracle_state": {
    "nonce": 42,
    "last_price": "0.0045",
    "last_update_ago_seconds": 300,
    "last_bridge_update_ago_seconds": 600
  },
  "system_info": {
    "rust_version": "0.1.0",
    "build_info": "production_build"
  }
}
```

### Metrics Endpoint (Default Port: 9090)

#### GET `/metrics`
Prometheus-compatible metrics in text exposition format

**Metrics Exposed:**
- `oracle_price_fetch_total` - Total number of price fetches
- `oracle_price_fetch_success_total` - Successful price fetches
- `oracle_price_fetch_errors_total` - Failed price fetches
- `oracle_bridge_update_total` - Total bridge price updates
- `oracle_bridge_update_success_total` - Successful bridge updates
- `oracle_bridge_update_errors_total` - Failed bridge updates
- `oracle_current_price` - Current token price (USD)
- `oracle_last_update_timestamp` - Unix timestamp of last update
- `oracle_price_deviation_percent` - Price change percentage

**Example Response:**
```
# HELP oracle_price_fetch_total Total number of price fetch attempts
# TYPE oracle_price_fetch_total counter
oracle_price_fetch_total 1234

# HELP oracle_current_price Current token price in USD
# TYPE oracle_current_price gauge
oracle_current_price{token_id="0"} 0.0045
```

## Usage Examples

### Check Oracle Health
```bash
curl http://localhost:8080/health
```

### Get Detailed Status
```bash
curl http://localhost:8080/status | jq
```

### Check Readiness
```bash
curl http://localhost:8080/ready
```

### Get Prometheus Metrics
```bash
curl http://localhost:9090/metrics
```

### Monitor with curl (continuous)
```bash
watch -n 10 'curl -s http://localhost:8080/status | jq .oracle_state'
```

## Configuration

### Using Config File
```bash
mys-price-oracle --config-path config.yaml
```

### Using Environment Variables (Railway)
```bash
mys-price-oracle --env
```

### Validate Configuration
```bash
mys-price-oracle --config-path config.yaml --validate-config
```

### Dry Run Mode
```bash
mys-price-oracle --config-path config.yaml --dry-run
```

## Monitoring with Prometheus

Add to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'myso-price-oracle'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 30s
```

## Grafana Dashboard

Recommended queries for monitoring:

- **Price Updates Per Hour:** `rate(oracle_bridge_update_success_total[1h]) * 3600`
- **Error Rate:** `rate(oracle_price_fetch_errors_total[5m]) / rate(oracle_price_fetch_total[5m])`
- **Current Price:** `oracle_current_price{token_id="0"}`
- **Price Volatility:** `delta(oracle_current_price[1h])`

## Development

### Running Tests
```bash
cargo test
```

### Code Coverage
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

## Support

For production deployment support or questions:

1. Check the troubleshooting section above
2. Review logs with correlation IDs for specific errors
3. Monitor Prometheus metrics for performance insights
4. Use health check endpoints for operational status

## License

Apache-2.0
