-- RENAME MY_IP TO MYDATA - COMPLETE SYSTEM RENAME
-- Production-safe migration to rename all my_ip tables and columns to mydata
-- This maintains all TimescaleDB features, indexes, compression, and constraints

-- ============================================================================
-- 1. DROP DEPENDENT OBJECTS (Views, Materialized Views, Functions)
-- ============================================================================

-- Drop views that depend on the tables
DROP VIEW IF EXISTS active_marketplace_data CASCADE;
DROP VIEW IF EXISTS popular_data_30d CASCADE;
DROP VIEW IF EXISTS creator_revenue_summary CASCADE;

-- Drop continuous aggregates (TimescaleDB materialized views)
DROP MATERIALIZED VIEW IF EXISTS my_ip_daily_revenue CASCADE;
DROP MATERIALIZED VIEW IF EXISTS my_ip_daily_access CASCADE;
DROP MATERIALIZED VIEW IF EXISTS my_ip_popular_data CASCADE;

-- Drop functions that depend on the tables
DROP FUNCTION IF EXISTS user_has_access(TEXT, TEXT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_data_pricing(TEXT) CASCADE;

-- ============================================================================
-- 2. RENAME TABLES
-- ============================================================================

-- Rename main data table (idempotent - only rename if source exists and target doesn't)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'my_ip_data'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'mydata_data'
    ) THEN
        ALTER TABLE my_ip_data RENAME TO mydata_data;
    END IF;
END $$;

-- Rename hypertables (TimescaleDB will maintain hypertable properties)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'my_ip_purchases'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'mydata_purchases'
    ) THEN
        ALTER TABLE my_ip_purchases RENAME TO mydata_purchases;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'my_ip_subscriptions'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'mydata_subscriptions'
    ) THEN
        ALTER TABLE my_ip_subscriptions RENAME TO mydata_subscriptions;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'my_ip_revenue'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'mydata_revenue'
    ) THEN
        ALTER TABLE my_ip_revenue RENAME TO mydata_revenue;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'my_ip_access_logs'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.tables 
        WHERE table_name = 'mydata_access_logs'
    ) THEN
        ALTER TABLE my_ip_access_logs RENAME TO mydata_access_logs;
    END IF;
END $$;

-- ============================================================================
-- 3. RENAME COLUMNS (ip_id -> mydata_id)
-- ============================================================================

-- Rename primary key column in main table
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_data' 
        AND column_name = 'ip_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_data' 
        AND column_name = 'mydata_id'
    ) THEN
        ALTER TABLE mydata_data RENAME COLUMN ip_id TO mydata_id;
    END IF;
END $$;

-- Rename foreign key columns in related tables
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_purchases' 
        AND column_name = 'ip_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_purchases' 
        AND column_name = 'mydata_id'
    ) THEN
        ALTER TABLE mydata_purchases RENAME COLUMN ip_id TO mydata_id;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_subscriptions' 
        AND column_name = 'ip_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_subscriptions' 
        AND column_name = 'mydata_id'
    ) THEN
        ALTER TABLE mydata_subscriptions RENAME COLUMN ip_id TO mydata_id;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_revenue' 
        AND column_name = 'ip_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_revenue' 
        AND column_name = 'mydata_id'
    ) THEN
        ALTER TABLE mydata_revenue RENAME COLUMN ip_id TO mydata_id;
    END IF;
    
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_access_logs' 
        AND column_name = 'ip_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'mydata_access_logs' 
        AND column_name = 'mydata_id'
    ) THEN
        ALTER TABLE mydata_access_logs RENAME COLUMN ip_id TO mydata_id;
    END IF;
END $$;

-- ============================================================================
-- 4. RENAME INDEXES
-- ============================================================================

-- Rename indexes (idempotent - only rename if source exists and target doesn't)
DO $$
BEGIN
    -- mydata_data indexes
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_data_owner')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_data_owner') THEN
        ALTER INDEX idx_my_ip_data_owner RENAME TO idx_mydata_data_owner;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_data_media_type')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_data_media_type') THEN
        ALTER INDEX idx_my_ip_data_media_type RENAME TO idx_mydata_data_media_type;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_data_tags')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_data_tags') THEN
        ALTER INDEX idx_my_ip_data_tags RENAME TO idx_mydata_data_tags;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_data_platform_id')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_data_platform_id') THEN
        ALTER INDEX idx_my_ip_data_platform_id RENAME TO idx_mydata_data_platform_id;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_data_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_data_time') THEN
        ALTER INDEX idx_my_ip_data_time RENAME TO idx_mydata_data_time;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_data_pricing')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_data_pricing') THEN
        ALTER INDEX idx_my_ip_data_pricing RENAME TO idx_mydata_data_pricing;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_data_geographic')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_data_geographic') THEN
        ALTER INDEX idx_my_ip_data_geographic RENAME TO idx_mydata_data_geographic;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_data_quality')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_data_quality') THEN
        ALTER INDEX idx_my_ip_data_quality RENAME TO idx_mydata_data_quality;
    END IF;
    
    -- mydata_purchases indexes
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_purchases_time_ip')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_purchases_time_mydata') THEN
        ALTER INDEX idx_my_ip_purchases_time_ip RENAME TO idx_mydata_purchases_time_mydata;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_purchases_buyer_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_purchases_buyer_time') THEN
        ALTER INDEX idx_my_ip_purchases_buyer_time RENAME TO idx_mydata_purchases_buyer_time;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_purchases_type_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_purchases_type_time') THEN
        ALTER INDEX idx_my_ip_purchases_type_time RENAME TO idx_mydata_purchases_type_time;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_purchases_ip_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_purchases_mydata_time') THEN
        ALTER INDEX idx_my_ip_purchases_ip_time RENAME TO idx_mydata_purchases_mydata_time;
    END IF;
    
    -- mydata_subscriptions indexes
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_subscriptions_time_ip')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_subscriptions_time_mydata') THEN
        ALTER INDEX idx_my_ip_subscriptions_time_ip RENAME TO idx_mydata_subscriptions_time_mydata;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_subscriptions_subscriber_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_subscriptions_subscriber_time') THEN
        ALTER INDEX idx_my_ip_subscriptions_subscriber_time RENAME TO idx_mydata_subscriptions_subscriber_time;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_subscriptions_end_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_subscriptions_end_time') THEN
        ALTER INDEX idx_my_ip_subscriptions_end_time RENAME TO idx_mydata_subscriptions_end_time;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_subscriptions_ip_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_subscriptions_mydata_time') THEN
        ALTER INDEX idx_my_ip_subscriptions_ip_time RENAME TO idx_mydata_subscriptions_mydata_time;
    END IF;
    
    -- mydata_revenue indexes
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_revenue_time_ip')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_revenue_time_mydata') THEN
        ALTER INDEX idx_my_ip_revenue_time_ip RENAME TO idx_mydata_revenue_time_mydata;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_revenue_to_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_revenue_to_time') THEN
        ALTER INDEX idx_my_ip_revenue_to_time RENAME TO idx_mydata_revenue_to_time;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_revenue_type_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_revenue_type_time') THEN
        ALTER INDEX idx_my_ip_revenue_type_time RENAME TO idx_mydata_revenue_type_time;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_revenue_ip_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_revenue_mydata_time') THEN
        ALTER INDEX idx_my_ip_revenue_ip_time RENAME TO idx_mydata_revenue_mydata_time;
    END IF;
    
    -- mydata_access_logs indexes
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_access_time_ip')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_access_time_mydata') THEN
        ALTER INDEX idx_my_ip_access_time_ip RENAME TO idx_mydata_access_time_mydata;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_access_user_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_access_user_time') THEN
        ALTER INDEX idx_my_ip_access_user_time RENAME TO idx_mydata_access_user_time;
    END IF;
    
    IF EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_my_ip_access_type_time')
       AND NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_mydata_access_type_time') THEN
        ALTER INDEX idx_my_ip_access_type_time RENAME TO idx_mydata_access_type_time;
    END IF;
END $$;

-- ============================================================================
-- 5. RENAME PRIMARY KEY CONSTRAINTS
-- ============================================================================

ALTER TABLE mydata_data DROP CONSTRAINT IF EXISTS my_ip_data_pkey;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'mydata_data_pkey'
    ) THEN
        ALTER TABLE mydata_data ADD CONSTRAINT mydata_data_pkey PRIMARY KEY (mydata_id);
    END IF;
END $$;

ALTER TABLE mydata_purchases DROP CONSTRAINT IF EXISTS pk_my_ip_purchases;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'pk_mydata_purchases'
    ) THEN
        ALTER TABLE mydata_purchases ADD CONSTRAINT pk_mydata_purchases PRIMARY KEY (id, time);
    END IF;
END $$;

ALTER TABLE mydata_subscriptions DROP CONSTRAINT IF EXISTS pk_my_ip_subscriptions;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'pk_mydata_subscriptions'
    ) THEN
        ALTER TABLE mydata_subscriptions ADD CONSTRAINT pk_mydata_subscriptions PRIMARY KEY (id, time);
    END IF;
END $$;

ALTER TABLE mydata_revenue DROP CONSTRAINT IF EXISTS pk_my_ip_revenue;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'pk_mydata_revenue'
    ) THEN
        ALTER TABLE mydata_revenue ADD CONSTRAINT pk_mydata_revenue PRIMARY KEY (id, time);
    END IF;
END $$;

ALTER TABLE mydata_access_logs DROP CONSTRAINT IF EXISTS pk_my_ip_access_logs;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'pk_mydata_access_logs'
    ) THEN
        ALTER TABLE mydata_access_logs ADD CONSTRAINT pk_mydata_access_logs PRIMARY KEY (id, time);
    END IF;
END $$;

-- ============================================================================
-- 6. RECREATE TIMESCALEDB CONTINUOUS AGGREGATES
-- ============================================================================

-- Daily revenue analytics aggregate
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'mydata_daily_revenue'
    ) THEN
        CREATE MATERIALIZED VIEW mydata_daily_revenue
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 day', time) AS day,
            mydata_id,
            to_address AS creator,
            revenue_type,
            SUM(amount) AS daily_revenue,
            COUNT(*) AS transaction_count
        FROM mydata_revenue
        GROUP BY time_bucket('1 day', time), mydata_id, to_address, revenue_type
        WITH NO DATA;
        
        -- Enable automatic refresh for daily revenue
        PERFORM add_continuous_aggregate_policy('mydata_daily_revenue',
            start_offset => INTERVAL '3 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END $$;

-- Daily access statistics aggregate
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'mydata_daily_access'
    ) THEN
        CREATE MATERIALIZED VIEW mydata_daily_access
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 day', time) AS day,
            mydata_id,
            access_type,
            COUNT(DISTINCT user_address) AS unique_users,
            COUNT(*) AS total_accesses
        FROM mydata_access_logs
        GROUP BY time_bucket('1 day', time), mydata_id, access_type
        WITH NO DATA;
        
        -- Enable automatic refresh for daily access
        PERFORM add_continuous_aggregate_policy('mydata_daily_access',
            start_offset => INTERVAL '3 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END $$;

-- Popular data tracking aggregate (hourly for real-time popularity)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'mydata_popular_data'
    ) THEN
        CREATE MATERIALIZED VIEW mydata_popular_data
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 hour', time) AS hour,
            mydata_id,
            COUNT(DISTINCT buyer) AS unique_purchasers,
            SUM(CASE WHEN purchase_type = 'one_time' THEN 1 ELSE 0 END) AS one_time_purchases,
            SUM(CASE WHEN purchase_type = 'subscription' THEN 1 ELSE 0 END) AS subscriptions,
            SUM(price) AS total_revenue
        FROM mydata_purchases
        GROUP BY time_bucket('1 hour', time), mydata_id
        WITH NO DATA;
        
        -- Enable automatic refresh for popular data
        PERFORM add_continuous_aggregate_policy('mydata_popular_data',
            start_offset => INTERVAL '1 day',
            end_offset => INTERVAL '15 minutes',
            schedule_interval => INTERVAL '15 minutes');
    END IF;
END $$;

-- ============================================================================
-- 7. RECREATE VIEWS
-- ============================================================================

-- Active marketplace data view
CREATE OR REPLACE VIEW active_mydata AS
SELECT 
    d.*,
    CASE 
        WHEN d.one_time_price IS NOT NULL AND d.subscription_price IS NOT NULL THEN 'both'
        WHEN d.one_time_price IS NOT NULL THEN 'one_time'
        WHEN d.subscription_price IS NOT NULL THEN 'subscription'
        ELSE 'free'
    END AS pricing_model,
    CASE
        WHEN d.timestamp_end IS NOT NULL AND d.timestamp_end < EXTRACT(EPOCH FROM NOW()) THEN false
        ELSE true
    END AS is_current
FROM mydata_data d;

-- Popular data view (last 30 days)
CREATE OR REPLACE VIEW popular_mydata_30d AS
SELECT 
    d.mydata_id,
    d.owner,
    d.media_type,
    d.tags,
    COUNT(DISTINCT p.buyer) AS unique_purchasers,
    SUM(p.price) AS total_revenue,
    COUNT(p.id) AS total_purchases,
    MAX(p.time) AS last_purchase
FROM mydata_data d
LEFT JOIN mydata_purchases p ON d.mydata_id = p.mydata_id 
    AND p.time >= NOW() - INTERVAL '30 days'
GROUP BY d.mydata_id, d.owner, d.media_type, d.tags
ORDER BY unique_purchasers DESC, total_revenue DESC;

-- Creator revenue summary view
CREATE OR REPLACE VIEW creator_mydata_revenue_summary AS
SELECT 
    d.owner AS creator,
    COUNT(DISTINCT d.mydata_id) AS data_entries,
    SUM(r.amount) AS total_revenue,
    COUNT(DISTINCT r.from_address) AS unique_customers,
    MAX(r.time) AS last_revenue
FROM mydata_data d
LEFT JOIN mydata_revenue r ON d.mydata_id = r.mydata_id
GROUP BY d.owner
ORDER BY total_revenue DESC NULLS LAST;

-- ============================================================================
-- 8. RECREATE HELPER FUNCTIONS
-- ============================================================================

-- Function to check if user has access to data
CREATE OR REPLACE FUNCTION user_has_mydata_access(
    p_mydata_id TEXT,
    p_user_address TEXT,
    p_current_time BIGINT DEFAULT EXTRACT(EPOCH FROM NOW())
) RETURNS BOOLEAN AS $$
DECLARE
    data_owner TEXT;
    subscription_end BIGINT;
    has_purchase BOOLEAN := FALSE;
BEGIN
    -- Get data owner
    SELECT owner INTO data_owner FROM mydata_data WHERE mydata_id = p_mydata_id;
    
    -- Owner always has access
    IF data_owner = p_user_address THEN
        RETURN TRUE;
    END IF;
    
    -- Check for one-time purchase
    SELECT TRUE INTO has_purchase 
    FROM mydata_purchases 
    WHERE mydata_id = p_mydata_id AND buyer = p_user_address AND purchase_type = 'one_time'
    LIMIT 1;
    
    IF has_purchase THEN
        RETURN TRUE;
    END IF;
    
    -- Check for active subscription
    SELECT MAX(subscription_end) INTO subscription_end
    FROM mydata_subscriptions 
    WHERE mydata_id = p_mydata_id AND subscriber = p_user_address;
    
    IF subscription_end IS NOT NULL AND subscription_end >= p_current_time THEN
        RETURN TRUE;
    END IF;
    
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql;

-- Function to get data pricing info
CREATE OR REPLACE FUNCTION get_mydata_pricing(p_mydata_id TEXT)
RETURNS TABLE(
    mydata_id TEXT,
    one_time_price BIGINT,
    subscription_price BIGINT,
    subscription_duration_days BIGINT,
    total_purchasers BIGINT,
    total_subscribers BIGINT,
    total_revenue BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        d.mydata_id,
        d.one_time_price,
        d.subscription_price,
        d.subscription_duration_days,
        COALESCE(p.purchaser_count, 0) AS total_purchasers,
        COALESCE(s.subscriber_count, 0) AS total_subscribers,
        COALESCE(r.total_revenue, 0) AS total_revenue
    FROM mydata_data d
    LEFT JOIN (
        SELECT mydata_id, COUNT(DISTINCT buyer) AS purchaser_count
        FROM mydata_purchases 
        WHERE purchase_type = 'one_time'
        GROUP BY mydata_id
    ) p ON d.mydata_id = p.mydata_id
    LEFT JOIN (
        SELECT mydata_id, COUNT(DISTINCT subscriber) AS subscriber_count
        FROM mydata_subscriptions
        GROUP BY mydata_id
    ) s ON d.mydata_id = s.mydata_id
    LEFT JOIN (
        SELECT mydata_id, SUM(amount) AS total_revenue
        FROM mydata_revenue
        GROUP BY mydata_id
    ) r ON d.mydata_id = r.mydata_id
    WHERE d.mydata_id = p_mydata_id;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- 9. UPDATE TABLE COMMENTS
-- ============================================================================

COMMENT ON TABLE mydata_data IS 'MyData marketplace entries with metadata and pricing';
COMMENT ON TABLE mydata_purchases IS 'Purchase records for one-time and subscription access (TimescaleDB)';
COMMENT ON TABLE mydata_subscriptions IS 'Active subscription tracking with expiry times (TimescaleDB)';
COMMENT ON TABLE mydata_revenue IS 'Revenue distribution and tracking (TimescaleDB)';
COMMENT ON TABLE mydata_access_logs IS 'Access pattern analytics and logs (TimescaleDB)';

