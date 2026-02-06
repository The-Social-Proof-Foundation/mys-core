# MySocial Orderbook Indexer API

The MySocial Orderbook Indexer provides comprehensive REST API endpoints for accessing real-time and historical orderbook data from the MySocial DEX.

## Base URL
```
http://your-indexer-host:port
```

## Table of Contents
- [Pool Management](#pool-management)
- [Market Data](#market-data)
- [Trading Data](#trading-data)
- [Volume Analytics](#volume-analytics)
- [Account Data](#account-data)
- [Utility Endpoints](#utility-endpoints)
- [Data Types](#data-types)
- [Error Handling](#error-handling)

## Pool Management

### GET /get_pools

Get all trading pools stored in the indexer database.

**Response:**
```json
[
  {
    "pool_id": "0x123...",
    "pool_name": "MYS_USD",
    "base_asset_id": "0x75d6::myusd::MYUSD",
    "base_asset_decimals": 9,
    "base_asset_symbol": "MYS",
    "base_asset_name": "MySocial",
    "quote_asset_id": "0x75d6::myusd::MYUSD",
    "quote_asset_decimals": 9,
    "quote_asset_symbol": "MYUSD",
    "quote_asset_name": "MyUSD",
    "min_size": 1000,
    "lot_size": 1000,
    "tick_size": 1000
  }
]
```

### GET /assets

Get all supported assets with their metadata.

**Response:**
```json
{
  "MYS": {
    "name": "MySocial",
    "can_withdraw": "true",
    "can_deposit": "true",
    "unified_cryptoasset_id": "1234",
    "contractAddressUrl": "https://explorer.mysocial.network/object/0x2::mys::MYS",
    "contractAddress": "0x2::mys::MYS"
  },
  "MYUSD": {
    "name": "MyUSD",
    "can_withdraw": "true",
    "can_deposit": "true",
    "contractAddress": "0x75d6::myusd::MYUSD"
  }
}
```

## Market Data

### GET /ticker

Get ticker data for all pools including last price, volume, and price change.

**Query Parameters:**
- `start_time` (optional): Start time in seconds (defaults to 24 hours ago)
- `end_time` (optional): End time in seconds (defaults to now)

**Response:**
```json
{
  "MYS_MYUSD": {
    "last_price": 1.23,
    "base_volume": 1000.50,
    "quote_volume": 1230.00,
    "isFrozen": 0
  }
}
```

### GET /summary

Get comprehensive summary data for all pools including price changes, highs/lows, and best bids/asks.

**Response:**
```json
[
  {
    "trading_pairs": "MYS_MYUSD",
    "base_currency": "MYS",
    "quote_currency": "MYUSD",
    "last_price": 1.23,
    "base_volume": 1000.50,
    "quote_volume": 1230.00,
    "price_change_percent_24h": 2.5,
    "highest_price_24h": 1.25,
    "lowest_price_24h": 1.20,
    "highest_bid": 1.22,
    "lowest_ask": 1.24
  }
]
```

### GET /orderbook/{pool_name}

Get Level 2 orderbook data (bids and asks) for a specific pool.

**Path Parameters:**
- `pool_name`: Pool name (e.g., "MYS_MYUSD")

**Query Parameters:**
- `depth` (optional): Number of price levels to return (default: 200, max: 200)
- `level` (optional): Orderbook level (1 for best bid/ask only, 2 for full orderbook)

**Examples:**
- `/orderbook/MYS_MYUSD?depth=10&level=2` - Get top 10 levels
- `/orderbook/MYS_MYUSD?level=1` - Get only best bid and ask

**Response:**
```json
{
  "timestamp": "1703123456789",
  "bids": [
    ["1.22", "100.5"],
    ["1.21", "250.0"]
  ],
  "asks": [
    ["1.24", "75.25"],
    ["1.25", "300.0"]
  ]
}
```

**Notes:**
- Prices and quantities are formatted as strings to preserve precision
- Data is fetched in real-time from the blockchain using dev_inspect

## Trading Data

### GET /trades/{pool_name}

Get recent trades for a specific pool.

**Path Parameters:**
- `pool_name`: Pool name (e.g., "MYS_MYUSD")

**Query Parameters:**
- `start_time` (optional): Start time in seconds (defaults to 24 hours ago)
- `end_time` (optional): End time in seconds (defaults to now)
- `limit` (optional): Maximum number of trades to return (default: 1)
- `maker_balance_manager_id` (optional): Filter by maker balance manager ID
- `taker_balance_manager_id` (optional): Filter by taker balance manager ID

**Examples:**
- `/trades/MYS_MYUSD?limit=50`
- `/trades/MYS_MYUSD?maker_balance_manager_id=0x123&start_time=1703123456`

**Response:**
```json
[
  {
    "trade_id": "123456789",
    "maker_order_id": "1234567890123456789",
    "taker_order_id": "9876543210987654321",
    "maker_balance_manager_id": "0x123...",
    "taker_balance_manager_id": "0x456...",
    "price": 1.23,
    "base_volume": 100.0,
    "quote_volume": 123.0,
    "timestamp": 1703123456789,
    "type": "buy"
  }
]
```

### GET /order_updates/{pool_name}

Get order updates (placements, fills, cancellations) for a specific pool.

**Path Parameters:**
- `pool_name`: Pool name (e.g., "MYS_MYUSD")

**Query Parameters:**
- `start_time` (optional): Start time in seconds (defaults to 24 hours ago)
- `end_time` (optional): End time in seconds (defaults to now)
- `limit` (optional): Maximum number of updates to return (default: 1)
- `balance_manager_id` (optional): Filter by balance manager ID
- `status` (optional): Filter by order status

**Response:**
```json
[
  {
    "order_id": "1234567890123456789",
    "price": 1.23,
    "original_quantity": 100.0,
    "remaining_quantity": 50.0,
    "filled_quantity": 50.0,
    "timestamp": 1703123456789,
    "type": "buy",
    "balance_manager_id": "0x123...",
    "status": "partial_filled"
  }
]
```

### GET /trade_count

Get total trade count across all pools.

**Query Parameters:**
- `start_time` (optional): Start time in seconds (defaults to 24 hours ago)
- `end_time` (optional): End time in seconds (defaults to now)

**Response:**
```json
42
```

## Volume Analytics

### GET /historical_volume/{pool_names}

Get historical volume data for specified pools.

**Path Parameters:**
- `pool_names`: Comma-separated pool names (e.g., "MYS_MYUSD,BTC_MYUSD")

**Query Parameters:**
- `start_time` (optional): Start time in seconds (defaults to 24 hours ago)
- `end_time` (optional): End time in seconds (defaults to now)
- `volume_in_base` (optional): Return volume in base currency (default: false, returns quote volume)

**Examples:**
- `/historical_volume/MYS_MYUSD`
- `/historical_volume/MYS_MYUSD,BTC_MYUSD?volume_in_base=true&start_time=1703123456`

**Response:**
```json
{
  "MYS_MYUSD": 12345.67,
  "BTC_MYUSD": 8901.23
}
```

### GET /all_historical_volume

Get historical volume data for all pools.

**Query Parameters:** Same as `/historical_volume/{pool_names}`

**Response:** Same format as `/historical_volume/{pool_names}` but includes all pools.

### GET /historical_volume_by_balance_manager_id/{pool_names}/{balance_manager_id}

Get historical volume data for a specific user across specified pools.

**Path Parameters:**
- `pool_names`: Comma-separated pool names
- `balance_manager_id`: User's balance manager ID

**Query Parameters:** Same as `/historical_volume/{pool_names}`

**Response:**
```json
{
  "MYS_MYUSD": [1500.0, 750.0],
  "BTC_MYUSD": [0.0, 2000.0]
}
```

**Notes:**
- Returns array `[maker_volume, taker_volume]` for each pool
- Volumes are in the same currency as specified by `volume_in_base`

### GET /historical_volume_by_balance_manager_id_with_interval/{pool_names}/{balance_manager_id}

Get historical volume data for a specific user with time interval breakdown.

**Path Parameters:** Same as above

**Query Parameters:**
- All parameters from the basic endpoint plus:
- `interval` (optional): Time interval in seconds (default: 3600 = 1 hour)

**Response:**
```json
{
  "[1703123456, 1703127056]": {
    "MYS_MYUSD": [100.0, 50.0],
    "BTC_MYUSD": [0.0, 75.0]
  },
  "[1703127056, 1703130656]": {
    "MYS_MYUSD": [200.0, 100.0],
    "BTC_MYUSD": [0.0, 150.0]
  }
}
```

## Account Data

### GET /get_net_deposits/{asset_ids}/{timestamp}

Get net deposits (deposits - withdrawals) for specified assets before a timestamp.

**Path Parameters:**
- `asset_ids`: Comma-separated asset IDs (with or without 0x prefix)
- `timestamp`: Timestamp in milliseconds

**Example:**
- `/get_net_deposits/0x75d6::myusd::MYUSD,0x2::mys::MYS/1703123456789`

**Response:**
```json
{
  "0x75d6::myusd::MYUSD": 1500000000,
  "0x2::mys::MYS": -500000000
}
```

## Utility Endpoints

### GET /

Health check endpoint.

**Response:**
```json
Status: 200 OK
```

## Data Types

### Order Status Values
- `placed`
- `partial_filled`
- `filled`
- `cancelled`
- `expired`

### Order Types
- `buy` (taker bid order)
- `sell` (taker ask order)

### Time Formats
- All timestamps are in milliseconds since Unix epoch
- Query parameters accept seconds, but are converted to milliseconds internally
- Response timestamps are always in milliseconds

### Numeric Precision
- Prices and quantities are returned as floats with appropriate decimal precision
- Raw blockchain data uses integers with scaling factors (typically 10^9)
- The API handles decimal conversion automatically

## Error Handling

All endpoints return standard HTTP status codes:

- `200`: Success
- `400`: Bad Request (invalid parameters)
- `404`: Not Found (pool or asset not found)
- `500`: Internal Server Error

Error responses include a descriptive error message:

```json
{
  "error": "Pool 'INVALID_POOL' not found"
}
```

## Configuration

The indexer is configured via `config.yaml`. Key settings include:

- Database connection parameters
- RPC endpoint URLs
- Checkpoint synchronization settings
- Worker thread counts

## Rate Limiting

Currently no rate limiting is implemented. Consider implementing rate limiting for production deployments.

## WebSocket Support

Currently only REST endpoints are available. WebSocket support for real-time updates may be added in future versions.

## Authentication

Currently no authentication is required. Consider implementing API key authentication for production deployments.

## Examples

### Get Orderbook for MYS/MYUSD Pair
```bash
curl "http://localhost:8080/orderbook/MYS_MYUSD?depth=10"
```

### Get Recent Trades
```bash
curl "http://localhost:8080/trades/MYS_MYUSD?limit=20"
```

### Get 24h Volume for Multiple Pools
```bash
curl "http://localhost:8080/historical_volume/MYS_MYUSD,BTC_MYUSD"
```

### Get User Trading Volume
```bash
curl "http://localhost:8080/historical_volume_by_balance_manager_id/MYS_MYUSD/0x123...456"
```

## Notes

- All endpoints are GET requests
- Most endpoints support time range filtering
- Data is sourced from the MySocial blockchain via RPC calls
- Real-time orderbook data uses `dev_inspect` to query the orderbook smart contract directly
- Historical data is served from the PostgreSQL database
- The indexer uses TimescaleDB for optimized time-series queries