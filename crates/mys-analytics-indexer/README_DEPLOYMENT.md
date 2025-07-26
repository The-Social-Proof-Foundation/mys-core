# MySocial Analytics Indexer - Production Deployment Guide

## Overview

The MySocial Analytics Indexer processes blockchain checkpoint data and exports it to various analytics destinations including Google Cloud Storage, BigQuery, and Snowflake. This guide covers complete production deployment to Railway with monitoring.

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Checkpoint     │────▶│  Analytics       │────▶│  Data Outputs   │
│  Source         │    │  Indexer         │    │  (GCS/BQ/SF)    │
│  (MySocial)     │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌──────────────────┐
                       │  MySocial        │
                       │  Prometheus      │
                       │  Infrastructure  │
                       │  :9184           │
                       └──────────────────┘
```

## Data Types Supported

The indexer can process different data types independently. Each type should be deployed as a separate service:

- **Checkpoint**: Core checkpoint data
- **Transaction**: Transaction details
- **Object**: Blockchain objects
- **Event**: Blockchain events
- **TransactionObjects**: Transaction-object relationships
- **MoveCall**: Move function calls
- **MovePackage**: Move package deployments
- **DynamicField**: Dynamic field data
- **WrappedObject**: Wrapped object data

## Step-by-Step Production Setup

### 1. Environment Variables Configuration

#### **Core Required Variables**

```bash
# === CORE INDEXER SETTINGS ===
# The MySocial RPC endpoint for checkpoint data
REST_URL="https://fullnode.mainnet.mysocial.network:9000"

# Data type to process (see FileType enum above)
FILE_TYPE="checkpoint"  # Change per deployment

# File format for output
FILE_FORMAT="parquet"   # "csv" or "parquet"

# === PROCESSING CONFIGURATION ===
# Number of checkpoints to process before uploading
CHECKPOINT_INTERVAL="10000"

# Maximum file size in MB before uploading
MAX_FILE_SIZE_MB="100"

# Time interval in seconds before uploading
TIME_INTERVAL_S="600"

# Starting checkpoint (optional - will auto-detect if not set)
# STARTING_CHECKPOINT_SEQ_NUM="1000000"

# === PROMETHEUS METRICS ===
# Metrics automatically exposed on standard MySocial port 9184

# === FILE STORAGE ===
# Local temporary directory for processing
CHECKPOINT_DIR="/tmp/analytics"

# Package cache directory
PACKAGE_CACHE_PATH="/opt/mys/db/package_cache"

# Package filtering (optional)
# PACKAGE_ID_FILTER="0x123456789..."
```

#### **Google Cloud Storage (Required)**

```bash
# === GOOGLE CLOUD STORAGE ===
# GCS bucket configuration
REMOTE_STORE_TYPE="gcs"
REMOTE_STORE_BUCKET="your-analytics-bucket"
REMOTE_STORE_REGION="us-central1"

# Optional: Path prefix in bucket
# REMOTE_STORE_PATH_PREFIX="analytics/v1"

# GCS Authentication
GOOGLE_SERVICE_ACCOUNT_PATH="/app/gcs-key.json"
# OR set GOOGLE_APPLICATION_CREDENTIALS env var

# Checkpoint source URL  
REMOTE_STORE_URL="https://checkpoints.mainnet.mysocial.network"
```

#### **Optional: BigQuery Integration**

```bash
# === BIGQUERY (OPTIONAL) ===
# Enable BigQuery max checkpoint reporting
REPORT_BQ_MAX_TABLE_CHECKPOINT="true"

# BigQuery configuration
BQ_SERVICE_ACCOUNT_KEY_FILE="/app/bq-key.json"
BQ_PROJECT_ID="your-gcp-project"
BQ_DATASET_ID="mysocial_analytics"
BQ_TABLE_ID="checkpoints"
BQ_CHECKPOINT_COL_ID="checkpoint_sequence_number"
```

#### **Optional: Snowflake Integration**

```bash
# === SNOWFLAKE (OPTIONAL) ===
# Enable Snowflake max checkpoint reporting
REPORT_SF_MAX_TABLE_CHECKPOINT="true"

# Snowflake configuration
SF_ACCOUNT_IDENTIFIER="your-account.region"
SF_WAREHOUSE="ANALYTICS_WH"
SF_DATABASE="MYSOCIAL"
SF_SCHEMA="ANALYTICS"
SF_USERNAME="analytics_user"
SF_ROLE="ANALYTICS_ROLE"
SF_PASSWORD="your-password"
SF_TABLE_ID="checkpoints"
SF_CHECKPOINT_COL_ID="checkpoint_sequence_number"
```

### 2. Railway Deployment

#### **Service Configuration Per Data Type**

Deploy separate Railway services for each data type you need:

1. **analytics-indexer-checkpoints** (`FILE_TYPE=checkpoint`)
2. **analytics-indexer-transactions** (`FILE_TYPE=transaction`)
3. **analytics-indexer-objects** (`FILE_TYPE=object`)
4. **analytics-indexer-events** (`FILE_TYPE=event`)
5. **analytics-indexer-move-calls** (`FILE_TYPE=move_call`)

Each service will:
- Process its specific data type
- Export Prometheus metrics on standard port 9184
- Upload processed data to GCS

### 3. Monitoring Integration

Analytics indexers automatically integrate with your existing MySocial monitoring infrastructure.

#### **Prometheus Metrics**

All services expose metrics on the standard MySocial port 9184 using `mysten-service::metrics`.

#### **Key Metrics Available**

- `total_received{data_type="checkpoint"}`: Checkpoints processed
- `last_uploaded_checkpoint{data_type="checkpoint"}`: Last checkpoint uploaded
- `max_checkpoint_on_store{data_type="checkpoint"}`: Max checkpoint in destination

Add these to your existing MySocial monitoring setup.

### 4. Production Checklist

#### **Before Deployment**
- [ ] GCS bucket created and accessible
- [ ] Service account keys configured
- [ ] BigQuery/Snowflake setup (if using)
- [ ] Package cache directory mounted
- [ ] Resource limits configured

#### **After Deployment**
- [ ] Metrics endpoints accessible on port 9184
- [ ] Data flowing to GCS
- [ ] Monitoring integrated with existing MySocial infrastructure
- [ ] Alerts configured
- [ ] Log aggregation setup

### 6. Scaling Considerations

#### **Resource Requirements**
- **CPU**: 1-2 cores per service
- **Memory**: 2-4GB per service  
- **Storage**: 10GB+ for temporary files
- **Network**: High bandwidth for checkpoint downloads

#### **Scaling Strategy**
- Deploy multiple services for different data types
- Scale individual services based on processing lag
- Monitor file upload frequency and size

### 7. Troubleshooting

#### **Common Issues**

1. **High Memory Usage**
   - Reduce `CHECKPOINT_INTERVAL`
   - Reduce `MAX_FILE_SIZE_MB`
   - Increase `TIME_INTERVAL_S`

2. **Slow Processing**
   - Check checkpoint source URL latency
   - Verify GCS upload performance
   - Monitor network bandwidth

3. **Missing Data**
   - Check `STARTING_CHECKPOINT_SEQ_NUM` setting
   - Verify checkpoint source availability
   - Check error logs for processing failures

#### **Log Analysis**
```bash
# Check processing status
kubectl logs -f deployment/analytics-indexer-checkpoints | grep "Processing checkpoint"

# Check upload status  
kubectl logs -f deployment/analytics-indexer-checkpoints | grep "Uploaded file"

# Check errors
kubectl logs -f deployment/analytics-indexer-checkpoints | grep "ERROR"
```

### 8. Data Output Structure

#### **GCS File Structure**
```
your-bucket/
├── checkpoints/
│   ├── epoch_0/
│   │   ├── 0_10000.parquet
│   │   ├── 10000_20000.parquet
│   │   └── ...
│   ├── epoch_1/
│   └── ...
├── transactions/
│   ├── epoch_0/
│   └── ...
└── events/
    ├── epoch_0/
    └── ...
```

This structure allows for efficient querying and analysis in downstream analytics tools.

## Next Steps

1. **Start with Checkpoint indexer** - This provides the foundation data
2. **Add Transaction indexer** - For transaction-level analytics  
3. **Add Event indexer** - For application-specific events
4. **Scale based on needs** - Add other data types as required

For questions or issues, refer to the MySocial documentation or create an issue in the repository. 