-- Migration: Create SPT Revenue Tracking and Unified Revenue System
-- Version: 20250622000001
-- Purpose: Production-ready revenue aggregation for MySocial ecosystem

-- ============================================================================
-- 1. SOCIAL PROOF TOKEN (SPT) REVENUE TRACKING
-- ============================================================================

-- SPT Revenue Table (TimescaleDB hypertable for high-volume swap fee tracking)
CREATE TABLE spt_revenue (
    pool_id VARCHAR NOT NULL,
    transaction_type VARCHAR NOT NULL CHECK (transaction_type IN ('buy', 'sell')), 
    trader VARCHAR NOT NULL,
    creator_address VARCHAR NOT NULL,
    platform_address VARCHAR NOT NULL,
    treasury_address VARCHAR NOT NULL,
    creator_fee BIGINT NOT NULL,
    platform_fee BIGINT NOT NULL,
    treasury_fee BIGINT NOT NULL,
    total_fee BIGINT NOT NULL,
    token_amount BIGINT NOT NULL,
    mys_amount BIGINT NOT NULL,
    token_price BIGINT NOT NULL,
    revenue_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL
);

-- Convert to TimescaleDB hypertable with 1-hour chunks for real-time SPT analytics
SELECT create_hypertable('spt_revenue', 'time', chunk_time_interval => INTERVAL '1 hour');

-- Optimized indexes for SPT revenue queries
CREATE INDEX idx_spt_revenue_time_pool ON spt_revenue (time DESC, pool_id);
CREATE INDEX idx_spt_revenue_creator_time ON spt_revenue (creator_address, time DESC);
CREATE INDEX idx_spt_revenue_platform_time ON spt_revenue (platform_address, time DESC);
CREATE INDEX idx_spt_revenue_type_time ON spt_revenue (transaction_type, time DESC);
CREATE INDEX idx_spt_revenue_trader_time ON spt_revenue (trader, time DESC);

-- ============================================================================
-- 2. UNIFIED REVENUE AGGREGATION TABLES
-- ============================================================================

-- Unified Revenue Summary (TimescaleDB hypertable for cross-platform analytics)
CREATE TABLE unified_revenue (
    revenue_source VARCHAR NOT NULL CHECK (revenue_source IN ('subscription', 'my_ip', 'spt', 'tips', 'posts')),
    revenue_type VARCHAR NOT NULL, 
    creator_address VARCHAR NOT NULL,
    platform_address VARCHAR,
    amount BIGINT NOT NULL,
    currency VARCHAR NOT NULL DEFAULT 'MYSO',
    content_id VARCHAR, -- post_id, ip_id, service_id, pool_id
    content_type VARCHAR, -- post, profile, service, data, token
    payer_address VARCHAR NOT NULL,
    recipient_address VARCHAR NOT NULL,
    revenue_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL
);

-- Convert to TimescaleDB hypertable with 1-hour chunks for unified analytics
SELECT create_hypertable('unified_revenue', 'time', chunk_time_interval => INTERVAL '1 hour');

-- Comprehensive indexes for unified revenue queries
CREATE INDEX idx_unified_revenue_time_source ON unified_revenue (time DESC, revenue_source);
CREATE INDEX idx_unified_revenue_creator_time ON unified_revenue (creator_address, time DESC);
CREATE INDEX idx_unified_revenue_platform_time ON unified_revenue (platform_address, time DESC) WHERE platform_address IS NOT NULL;
CREATE INDEX idx_unified_revenue_source_type ON unified_revenue (revenue_source, revenue_type, time DESC);
CREATE INDEX idx_unified_revenue_content ON unified_revenue (content_id, content_type, time DESC) WHERE content_id IS NOT NULL;
CREATE INDEX idx_unified_revenue_payer_time ON unified_revenue (payer_address, time DESC);

-- ============================================================================
-- 3. TIMESCALEDB CONTINUOUS AGGREGATES FOR REAL-TIME ANALYTICS
-- ============================================================================

-- Hourly Revenue Summary by Source (Real-time aggregate)
CREATE MATERIALIZED VIEW revenue_hourly_summary
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 hour', time) AS hour,
    revenue_source,
    revenue_type,
    creator_address,
    platform_address,
    SUM(amount) AS total_revenue,
    COUNT(*) AS transaction_count,
    COUNT(DISTINCT payer_address) AS unique_payers,
    AVG(amount) AS avg_transaction_amount
FROM unified_revenue
GROUP BY time_bucket('1 hour', time), revenue_source, revenue_type, creator_address, platform_address
WITH NO DATA;

-- Enable real-time refresh (window must be > 1 hour for unified_revenue chunk interval, using 3h for safety)
SELECT add_continuous_aggregate_policy('revenue_hourly_summary',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '5 minutes',
    schedule_interval => INTERVAL '5 minutes');

-- Daily Revenue Summary by Creator (For leaderboards)
CREATE MATERIALIZED VIEW revenue_daily_creators
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 day', time) AS day,
    creator_address,
    revenue_source,
    SUM(amount) AS daily_revenue,
    COUNT(*) AS transaction_count,
    COUNT(DISTINCT payer_address) AS unique_payers,
    MAX(amount) AS largest_transaction,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS subscription_revenue,
    SUM(CASE WHEN revenue_source = 'my_ip' THEN amount ELSE 0 END) AS myip_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS spt_revenue,
    SUM(CASE WHEN revenue_source = 'tips' THEN amount ELSE 0 END) AS tips_revenue
FROM unified_revenue
GROUP BY time_bucket('1 day', time), creator_address, revenue_source
WITH NO DATA;

-- Enable hourly refresh for daily creator leaderboards
SELECT add_continuous_aggregate_policy('revenue_daily_creators',
    start_offset => INTERVAL '7 days',
    end_offset => INTERVAL '30 minutes',
    schedule_interval => INTERVAL '30 minutes');

-- Monthly Revenue Summary by Platform (For platform analytics)
CREATE MATERIALIZED VIEW revenue_monthly_platforms
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 month', time) AS month,
    platform_address,
    revenue_source,
    SUM(amount) AS monthly_revenue,
    COUNT(*) AS transaction_count,
    COUNT(DISTINCT creator_address) AS unique_creators,
    COUNT(DISTINCT payer_address) AS unique_payers,
    AVG(amount) AS avg_transaction_amount,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS subscription_revenue,
    SUM(CASE WHEN revenue_source = 'my_ip' THEN amount ELSE 0 END) AS myip_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS spt_revenue
FROM unified_revenue
WHERE platform_address IS NOT NULL
GROUP BY time_bucket('1 month', time), platform_address, revenue_source
WITH NO DATA;

-- Enable daily refresh for monthly platform analytics
SELECT add_continuous_aggregate_policy('revenue_monthly_platforms',
    start_offset => INTERVAL '3 months',
    end_offset => INTERVAL '1 day',
    schedule_interval => INTERVAL '1 day');

-- Real-time Revenue Metrics (5-minute buckets for dashboards)
CREATE MATERIALIZED VIEW revenue_realtime_metrics
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('5 minutes', time) AS bucket,
    revenue_source,
    SUM(amount) AS revenue_5min,
    COUNT(*) AS transactions_5min,
    COUNT(DISTINCT creator_address) AS active_creators,
    COUNT(DISTINCT payer_address) AS active_payers,
    MAX(amount) AS max_transaction,
    MIN(amount) AS min_transaction
FROM unified_revenue
GROUP BY time_bucket('5 minutes', time), revenue_source
WITH NO DATA;

-- Enable real-time refresh (window must be > 1 hour for unified_revenue chunk interval, using 3h for safety)
SELECT add_continuous_aggregate_policy('revenue_realtime_metrics',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute');

-- ============================================================================
-- 4. SPT-SPECIFIC CONTINUOUS AGGREGATES
-- ============================================================================

-- SPT Trading Volume and Fee Analytics (Hourly)
CREATE MATERIALIZED VIEW spt_hourly_analytics
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 hour', time) AS hour,
    pool_id,
    creator_address,
    transaction_type,
    SUM(total_fee) AS total_fees,
    SUM(creator_fee) AS total_creator_fees,
    SUM(platform_fee) AS total_platform_fees,
    SUM(treasury_fee) AS total_treasury_fees,
    SUM(mys_amount) AS total_volume,
    SUM(token_amount) AS total_tokens,
    COUNT(*) AS transaction_count,
    COUNT(DISTINCT trader) AS unique_traders,
    AVG(token_price) AS avg_price,
    MAX(token_price) AS max_price,
    MIN(token_price) AS min_price
FROM spt_revenue
GROUP BY time_bucket('1 hour', time), pool_id, creator_address, transaction_type
WITH NO DATA;

-- Enable real-time refresh for SPT analytics (window must be > 1 hour for spt_revenue chunk interval, using 3h for safety)
SELECT add_continuous_aggregate_policy('spt_hourly_analytics',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '5 minutes',
    schedule_interval => INTERVAL '5 minutes');

-- ============================================================================
-- CONTINUOUS AGGREGATE REFRESH POLICY NOTES
-- ============================================================================

-- Note: All continuous aggregates will be populated by their refresh policies automatically
-- Initial data will be available after the first scheduled refresh:
-- - revenue_hourly_summary: 5-minute refresh
-- - revenue_daily_creators: 30-minute refresh  
-- - revenue_monthly_platforms: daily refresh
-- - revenue_realtime_metrics: 1-minute refresh
-- - spt_hourly_analytics: 5-minute refresh

-- ============================================================================
-- 5. PERFORMANCE OPTIMIZATIONS
-- ============================================================================

-- Enable compression on hypertables first, then add policies
ALTER TABLE spt_revenue SET (timescaledb.compress = true);
ALTER TABLE unified_revenue SET (timescaledb.compress = true);

-- Compression policies for high-volume revenue data
SELECT add_compression_policy('spt_revenue', INTERVAL '24 hours');
SELECT add_compression_policy('unified_revenue', INTERVAL '24 hours');

-- Retention policies for high-volume data
SELECT add_retention_policy('spt_revenue', INTERVAL '2 years');
SELECT add_retention_policy('unified_revenue', INTERVAL '3 years');

-- ============================================================================
-- 6. HELPER VIEWS FOR API OPTIMIZATION
-- ============================================================================

-- SPT Creator Revenue Summary (for leaderboards and profile pages)
CREATE OR REPLACE VIEW spt_creator_revenue_summary AS
SELECT 
    creator_address,
    SUM(daily_revenue) AS total_revenue,
    SUM(subscription_revenue) AS total_subscription_revenue,
    SUM(myip_revenue) AS total_myip_revenue,
    SUM(spt_revenue) AS total_spt_revenue,
    SUM(tips_revenue) AS total_tips_revenue,
    SUM(transaction_count) AS total_transactions,
    SUM(unique_payers) AS total_unique_payers,
    MAX(largest_transaction) AS largest_single_transaction,
    COUNT(DISTINCT day) AS active_days,
    MAX(day) AS last_revenue_date
FROM revenue_daily_creators
WHERE day >= NOW() - INTERVAL '30 days'
GROUP BY creator_address
ORDER BY total_revenue DESC;

-- Platform Revenue Summary (for platform analytics)
CREATE OR REPLACE VIEW platform_revenue_summary AS
SELECT 
    platform_address,
    SUM(monthly_revenue) AS total_revenue,
    SUM(subscription_revenue) AS total_subscription_revenue,
    SUM(myip_revenue) AS total_myip_revenue,
    SUM(spt_revenue) AS total_spt_revenue,
    SUM(transaction_count) AS total_transactions,
    SUM(unique_creators) AS total_creators,
    SUM(unique_payers) AS total_payers,
    AVG(avg_transaction_amount) AS avg_transaction_amount,
    COUNT(DISTINCT month) AS active_months,
    MAX(month) AS last_active_month
FROM revenue_monthly_platforms
WHERE month >= DATE_TRUNC('month', NOW() - INTERVAL '12 months')
GROUP BY platform_address
ORDER BY total_revenue DESC;

-- Real-time Revenue Dashboard (last 24 hours)
CREATE OR REPLACE VIEW revenue_dashboard_24h AS
SELECT 
    revenue_source,
    SUM(revenue_5min) AS total_revenue_24h,
    SUM(transactions_5min) AS total_transactions_24h,
    COUNT(DISTINCT active_creators) AS unique_creators_24h,
    COUNT(DISTINCT active_payers) AS unique_payers_24h,
    MAX(max_transaction) AS largest_transaction_24h,
    AVG(revenue_5min) AS avg_revenue_per_5min
FROM revenue_realtime_metrics
WHERE bucket >= NOW() - INTERVAL '24 hours'
GROUP BY revenue_source
ORDER BY total_revenue_24h DESC;

-- ============================================================================
-- 7. INDEXES FOR VIEWS AND COMPLEX QUERIES
-- ============================================================================

-- Composite indexes for common query patterns
CREATE INDEX idx_unified_revenue_creator_source_time ON unified_revenue (creator_address, revenue_source, time DESC);
CREATE INDEX idx_unified_revenue_time_amount ON unified_revenue (time DESC, amount DESC);
CREATE INDEX idx_spt_revenue_pool_time_fees ON spt_revenue (pool_id, time DESC, total_fee DESC);

-- ============================================================================
-- 8. TABLE COMMENTS FOR DOCUMENTATION
-- ============================================================================

COMMENT ON TABLE spt_revenue IS 'SPT swap fee revenue tracking with real-time analytics (TimescaleDB)';
COMMENT ON TABLE unified_revenue IS 'Unified revenue tracking across all MySocial revenue sources (TimescaleDB)';
COMMENT ON MATERIALIZED VIEW revenue_hourly_summary IS 'Real-time hourly revenue aggregates (5-minute refresh)';
COMMENT ON MATERIALIZED VIEW revenue_daily_creators IS 'Daily creator revenue for leaderboards (30-minute refresh)';
COMMENT ON MATERIALIZED VIEW revenue_monthly_platforms IS 'Monthly platform revenue analytics (daily refresh)';
COMMENT ON MATERIALIZED VIEW revenue_realtime_metrics IS 'Real-time 5-minute revenue metrics (1-minute refresh)';
COMMENT ON MATERIALIZED VIEW spt_hourly_analytics IS 'SPT trading analytics with fee breakdowns (5-minute refresh)';
COMMENT ON VIEW spt_creator_revenue_summary IS 'SPT creator revenue leaderboard (30-day summary)';
COMMENT ON VIEW platform_revenue_summary IS 'Platform revenue analytics (12-month summary)';
COMMENT ON VIEW revenue_dashboard_24h IS 'Real-time dashboard metrics (24-hour summary)'; 