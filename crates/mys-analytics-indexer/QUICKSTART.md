# MySocial Analytics Indexer - Quick Start Guide

## 🚀 Local Development Setup

### Prerequisites

1. **Docker & Docker Compose** installed
2. **Google Cloud Storage bucket** (for production data storage)
3. **GCS Service Account Key** (optional for local testing)

### 1. Quick Local Testing (No GCS Required)

```bash
# Clone the repository (if not already done)
cd crates/mys-analytics-indexer

# Start with docker-compose
docker-compose up --build

# Access the services:
# - Analytics Indexer Metrics: http://localhost:9184/metrics, http://localhost:9185/metrics
```

This will start:
- ✅ **Analytics Indexer (Checkpoints)** with metrics on port 9184
- ✅ **Analytics Indexer (Transactions)** with metrics on port 9185

### 2. Production Railway Deployment

#### Step 1: Prepare Your Environment

1. **Create GCS Bucket**:
   ```bash
   gsutil mb gs://your-analytics-bucket
   ```

2. **Create Service Account**:
   ```bash
   gcloud iam service-accounts create analytics-indexer
   gcloud projects add-iam-policy-binding YOUR_PROJECT \
     --member="serviceAccount:analytics-indexer@YOUR_PROJECT.iam.gserviceaccount.com" \
     --role="roles/storage.admin"
   gcloud iam service-accounts keys create gcs-key.json \
     --iam-account=analytics-indexer@YOUR_PROJECT.iam.gserviceaccount.com
   ```

#### Step 2: Deploy to Railway

1. **Create Railway Projects** (one per data type):
   ```bash
   # For each data type you want to process
   railway login
   railway init analytics-indexer-checkpoints
   railway init analytics-indexer-transactions
   railway init analytics-indexer-events
   ```

2. **Configure Environment Variables**:
   - Copy `env-templates/checkpoint.env` for reference
   - Set variables in Railway dashboard or CLI:
   ```bash
   railway variables set REST_URL=https://fullnode.mainnet.mysocial.network:9000
   railway variables set FILE_TYPE=checkpoint
   railway variables set REMOTE_STORE_BUCKET=your-analytics-bucket
   railway variables set GOOGLE_APPLICATION_CREDENTIALS="$(cat gcs-key.json)"
   ```

3. **Deploy**:
   ```bash
   railway up
   ```

#### Step 3: Setup Monitoring

1. **MySocial Monitoring Integration**:
   Analytics indexers automatically expose metrics on port 9184 using the standard MySocial monitoring infrastructure. Add them to your existing monitoring setup.

### 3. Environment Variables Reference

#### Required Variables:
```bash
REST_URL=https://fullnode.mainnet.mysocial.network:9000
FILE_TYPE=checkpoint  # or transaction, event, object, etc.
REMOTE_STORE_BUCKET=your-analytics-bucket
```

#### Optional Variables:
```bash
FILE_FORMAT=parquet                    # or csv
CHECKPOINT_INTERVAL=10000              # checkpoints per batch
MAX_FILE_SIZE_MB=100                   # MB before upload
TIME_INTERVAL_S=600                    # seconds before upload
# Metrics automatically on port 9184
STARTING_CHECKPOINT_SEQ_NUM=1000000    # starting point
```

### 4. Data Types Available

Deploy separate services for each data type you need:

| Data Type | Description | Use Case |
|-----------|-------------|----------|
| `checkpoint` | Core checkpoint data | Foundation analytics |
| `transaction` | Transaction details | User activity analysis |
| `event` | Blockchain events | Application-specific metrics |
| `object` | Object state changes | Asset tracking |
| `move_call` | Function calls | Contract interaction analysis |
| `move_package` | Package deployments | Development metrics |

### 5. Monitoring & Alerting

#### Key Metrics to Watch:
- **Processing Rate**: `rate(total_received[5m])`
- **Upload Lag**: `max_checkpoint_on_store - last_uploaded_checkpoint`
- **Service Health**: `up{job=~"analytics-indexer.*"}`

#### Recommended Alerts:
```yaml
# High upload lag
- alert: HighUploadLag
  expr: max_checkpoint_on_store - last_uploaded_checkpoint > 10000
  
# Service down
- alert: IndexerDown
  expr: up{job=~"analytics-indexer.*"} == 0
```

### 6. Data Output

Analytics data is stored in GCS with this structure:

```
your-bucket/
├── checkpoints/epoch_0/0_10000.parquet
├── checkpoints/epoch_0/10000_20000.parquet
├── transactions/epoch_0/0_10000.parquet
└── events/epoch_0/0_10000.parquet
```

Files can be analyzed with:
- **BigQuery** (native Parquet support)
- **Google Cloud DataFlow**
- **Apache Spark**
- **pandas/polars** (for smaller datasets)

### 7. Troubleshooting

#### Common Issues:

1. **"REST_URL connection failed"**
   - Check MySocial RPC endpoint is accessible
   - Verify network connectivity

2. **"GCS upload failed"**
   - Check service account permissions
   - Verify bucket exists and is accessible

3. **"High memory usage"**
   - Reduce `CHECKPOINT_INTERVAL`
   - Reduce `MAX_FILE_SIZE_MB`

#### Debug Commands:
```bash
# Check metrics
curl http://localhost:9184/metrics

# View logs
docker-compose logs analytics-indexer-checkpoints

# Test GCS connectivity
gsutil ls gs://your-analytics-bucket
```

### 8. Next Steps

1. **Start with checkpoint indexer** - provides foundation data
2. **Add transaction indexer** - for user activity analysis
3. **Add event indexer** - for application-specific metrics
4. **Scale based on processing lag** - monitor upload lag metrics
5. **Setup BigQuery/Snowflake** - for advanced analytics (optional)

---

**Need Help?** Check the full deployment guide in `README_DEPLOYMENT.md` or create an issue in the repository. 