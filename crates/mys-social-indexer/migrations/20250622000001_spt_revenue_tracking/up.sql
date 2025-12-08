-- Migration: Create SPT Revenue Tracking and Unified Revenue System
-- Version: 20250622000001
-- Purpose: Production-ready revenue aggregation for MySocial ecosystem

-- ============================================================================
-- 1. SOCIAL PROOF TOKEN (SPT) REVENUE TRACKING
-- ============================================================================

-- SPT Revenue Table (TimescaleDB hypertable for high-volume swap fee tracking)
CREATE TABLE IF NOT EXISTS spt_revenue (
    pool_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL CHECK (transaction_type IN ('buy', 'sell')), 
    trader TEXT NOT NULL,
    creator_address TEXT NOT NULL,
    platform_address TEXT NOT NULL,
    treasury_address TEXT NOT NULL,
    creator_fee BIGINT NOT NULL,
    platform_fee BIGINT NOT NULL,
    treasury_fee BIGINT NOT NULL,
    total_fee BIGINT NOT NULL,
    token_amount BIGINT NOT NULL,
    mys_amount BIGINT NOT NULL,
    token_price BIGINT NOT NULL,
    revenue_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Convert to TimescaleDB hypertable with 1-hour chunks for real-time SPT analytics
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'spt_revenue'
    ) THEN
        PERFORM create_hypertable('spt_revenue'::regclass, 'time'::name, chunk_time_interval => INTERVAL '1 hour');
    END IF;
END $$;

-- Optimized indexes for SPT revenue queries
CREATE INDEX IF NOT EXISTS idx_spt_revenue_time_pool ON spt_revenue (time DESC, pool_id);
CREATE INDEX IF NOT EXISTS idx_spt_revenue_creator_time ON spt_revenue (creator_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_spt_revenue_platform_time ON spt_revenue (platform_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_spt_revenue_type_time ON spt_revenue (transaction_type, time DESC);
CREATE INDEX IF NOT EXISTS idx_spt_revenue_trader_time ON spt_revenue (trader, time DESC);

-- ============================================================================
-- 2. UNIFIED REVENUE AGGREGATION TABLES
-- ============================================================================

-- Unified Revenue Summary (TimescaleDB hypertable for cross-platform analytics)
CREATE TABLE IF NOT EXISTS unified_revenue (
    revenue_source TEXT NOT NULL CHECK (revenue_source IN ('subscription', 'my_ip', 'spt', 'tips', 'posts')),
    revenue_type TEXT NOT NULL, 
    creator_address TEXT NOT NULL,
    platform_address TEXT,
    amount BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'MYSO',
    content_id TEXT, -- post_id, ip_id, service_id, pool_id
    content_type TEXT, -- post, profile, service, data, token
    payer_address TEXT NOT NULL,
    recipient_address TEXT NOT NULL,
    revenue_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Convert to TimescaleDB hypertable with 1-hour chunks for unified analytics
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.hypertables 
        WHERE hypertable_schema = 'public' AND hypertable_name = 'unified_revenue'
    ) THEN
        PERFORM create_hypertable('unified_revenue'::regclass, 'time'::name, chunk_time_interval => INTERVAL '1 hour');
    END IF;
END $$;

-- Comprehensive indexes for unified revenue queries
CREATE INDEX IF NOT EXISTS idx_unified_revenue_time_source ON unified_revenue (time DESC, revenue_source);
CREATE INDEX IF NOT EXISTS idx_unified_revenue_creator_time ON unified_revenue (creator_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_unified_revenue_platform_time ON unified_revenue (platform_address, time DESC) WHERE platform_address IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_unified_revenue_source_type ON unified_revenue (revenue_source, revenue_type, time DESC);
CREATE INDEX IF NOT EXISTS idx_unified_revenue_content ON unified_revenue (content_id, content_type, time DESC) WHERE content_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_unified_revenue_payer_time ON unified_revenue (payer_address, time DESC);

-- ============================================================================
-- 3. CONTINUOUS AGGREGATES REMOVED DUE TO CREATION ISSUES  
-- ============================================================================
-- Note: All continuous aggregates removed due to TimescaleDB creation failures
-- Direct queries on unified_revenue and spt_revenue tables can be used instead

-- ============================================================================
-- 4. PERFORMANCE OPTIMIZATIONS
-- ============================================================================

-- Enable compression on hypertables first, then add policies
DO $$
BEGIN
    -- Set compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_revenue'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_revenue SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'unified_revenue'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE unified_revenue SET (timescaledb.compress = true);
    END IF;
    
    -- Add compression policies if they don't exist
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_revenue'
    ) THEN
        PERFORM add_compression_policy('spt_revenue', INTERVAL '24 hours');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'unified_revenue'
    ) THEN
        PERFORM add_compression_policy('unified_revenue', INTERVAL '24 hours');
    END IF;
    
    -- Add retention policies if they don't exist
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_retention' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_revenue'
    ) THEN
        PERFORM add_retention_policy('spt_revenue', INTERVAL '2 years');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_retention' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'unified_revenue'
    ) THEN
        PERFORM add_retention_policy('unified_revenue', INTERVAL '3 years');
    END IF;
END $$;

-- ============================================================================
-- 5. HELPER VIEWS FOR API OPTIMIZATION
-- ============================================================================

-- SPT Creator Revenue Summary (for leaderboards and profile pages)
-- Uses direct query on unified_revenue since revenue_daily_creators was removed
CREATE OR REPLACE VIEW spt_creator_revenue_summary AS
SELECT 
    creator_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'my_ip' THEN amount ELSE 0 END) AS total_myip_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    SUM(CASE WHEN revenue_source = 'tips' THEN amount ELSE 0 END) AS total_tips_revenue,
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT payer_address) AS total_unique_payers,
    MAX(amount) AS largest_single_transaction,
    COUNT(DISTINCT DATE(time)) AS active_days,
    MAX(time) AS last_revenue_date
FROM unified_revenue
WHERE time >= NOW() - INTERVAL '30 days'
GROUP BY creator_address
ORDER BY total_revenue DESC;

-- Platform Revenue Summary (for platform analytics) 
-- Uses direct query on unified_revenue since revenue_monthly_platforms was removed
CREATE OR REPLACE VIEW platform_revenue_summary AS
SELECT 
    platform_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'my_ip' THEN amount ELSE 0 END) AS total_myip_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT creator_address) AS total_creators,
    COUNT(DISTINCT payer_address) AS total_payers,
    AVG(amount) AS avg_transaction_amount,
    COUNT(DISTINCT DATE_TRUNC('month', time)) AS active_months,
    MAX(time) AS last_active_time
FROM unified_revenue
WHERE platform_address IS NOT NULL
    AND time >= DATE_TRUNC('month', NOW() - INTERVAL '12 months')
GROUP BY platform_address
ORDER BY total_revenue DESC;

-- Real-time Revenue Dashboard (last 24 hours)
-- Uses direct query on unified_revenue since revenue_realtime_metrics was removed
CREATE OR REPLACE VIEW revenue_dashboard_24h AS
SELECT 
    revenue_source,
    SUM(amount) AS total_revenue_24h,
    COUNT(*) AS total_transactions_24h,
    COUNT(DISTINCT creator_address) AS unique_creators_24h,
    COUNT(DISTINCT payer_address) AS unique_payers_24h,
    MAX(amount) AS largest_transaction_24h,
    AVG(amount) AS avg_transaction_amount
FROM unified_revenue
WHERE time >= NOW() - INTERVAL '24 hours'
GROUP BY revenue_source
ORDER BY total_revenue_24h DESC;

-- ============================================================================
-- 6. INDEXES FOR VIEWS AND COMPLEX QUERIES
-- ============================================================================

-- Composite indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_unified_revenue_creator_source_time ON unified_revenue (creator_address, revenue_source, time DESC);
CREATE INDEX IF NOT EXISTS idx_unified_revenue_time_amount ON unified_revenue (time DESC, amount DESC);
CREATE INDEX IF NOT EXISTS idx_spt_revenue_pool_time_fees ON spt_revenue (pool_id, time DESC, total_fee DESC);

-- ============================================================================
-- 7. TABLE COMMENTS FOR DOCUMENTATION
-- ============================================================================

COMMENT ON TABLE spt_revenue IS 'SPT swap fee revenue tracking with real-time analytics (TimescaleDB)';
COMMENT ON TABLE unified_revenue IS 'Unified revenue tracking across all MySocial revenue sources (TimescaleDB)';
COMMENT ON VIEW spt_creator_revenue_summary IS 'Creator revenue leaderboard using direct unified_revenue queries (30-day summary)';
COMMENT ON VIEW platform_revenue_summary IS 'Platform revenue analytics using direct unified_revenue queries (12-month summary)';
COMMENT ON VIEW revenue_dashboard_24h IS 'Real-time dashboard metrics using direct unified_revenue queries (24-hour summary)'; 