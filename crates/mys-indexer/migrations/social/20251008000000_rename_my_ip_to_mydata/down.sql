-- REVERT MYDATA TO MY_IP - ROLLBACK MIGRATION
-- Safely reverts all naming changes from mydata back to my_ip

-- ============================================================================
-- 1. DROP DEPENDENT OBJECTS (Views, Materialized Views, Functions)
-- ============================================================================

-- Drop views that depend on the tables
DROP VIEW IF EXISTS active_mydata CASCADE;
DROP VIEW IF EXISTS popular_mydata_30d CASCADE;
DROP VIEW IF EXISTS creator_mydata_revenue_summary CASCADE;

-- Drop continuous aggregates (TimescaleDB materialized views)
DROP MATERIALIZED VIEW IF EXISTS mydata_daily_revenue CASCADE;
DROP MATERIALIZED VIEW IF EXISTS mydata_daily_access CASCADE;
DROP MATERIALIZED VIEW IF EXISTS mydata_popular_data CASCADE;

-- Drop functions that depend on the tables
DROP FUNCTION IF EXISTS user_has_mydata_access(TEXT, TEXT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_mydata_pricing(TEXT) CASCADE;

-- ============================================================================
-- 2. RENAME TABLES BACK
-- ============================================================================

-- Rename main data table back
ALTER TABLE mydata_data RENAME TO my_ip_data;

-- Rename hypertables back
ALTER TABLE mydata_purchases RENAME TO my_ip_purchases;
ALTER TABLE mydata_subscriptions RENAME TO my_ip_subscriptions;
ALTER TABLE mydata_revenue RENAME TO my_ip_revenue;
ALTER TABLE mydata_access_logs RENAME TO my_ip_access_logs;

-- ============================================================================
-- 3. RENAME COLUMNS BACK (mydata_id -> ip_id)
-- ============================================================================

-- Rename primary key column back
ALTER TABLE my_ip_data RENAME COLUMN mydata_id TO ip_id;

-- Rename foreign key columns back
ALTER TABLE my_ip_purchases RENAME COLUMN mydata_id TO ip_id;
ALTER TABLE my_ip_subscriptions RENAME COLUMN mydata_id TO ip_id;
ALTER TABLE my_ip_revenue RENAME COLUMN mydata_id TO ip_id;
ALTER TABLE my_ip_access_logs RENAME COLUMN mydata_id TO ip_id;

-- ============================================================================
-- 4. RENAME INDEXES BACK
-- ============================================================================

-- my_ip_data indexes
ALTER INDEX IF EXISTS idx_mydata_data_owner RENAME TO idx_my_ip_data_owner;
ALTER INDEX IF EXISTS idx_mydata_data_media_type RENAME TO idx_my_ip_data_media_type;
ALTER INDEX IF EXISTS idx_mydata_data_tags RENAME TO idx_my_ip_data_tags;
ALTER INDEX IF EXISTS idx_mydata_data_platform_id RENAME TO idx_my_ip_data_platform_id;
ALTER INDEX IF EXISTS idx_mydata_data_time RENAME TO idx_my_ip_data_time;
ALTER INDEX IF EXISTS idx_mydata_data_pricing RENAME TO idx_my_ip_data_pricing;
ALTER INDEX IF EXISTS idx_mydata_data_geographic RENAME TO idx_my_ip_data_geographic;
ALTER INDEX IF EXISTS idx_mydata_data_quality RENAME TO idx_my_ip_data_quality;

-- my_ip_purchases indexes
ALTER INDEX IF EXISTS idx_mydata_purchases_time_mydata RENAME TO idx_my_ip_purchases_time_ip;
ALTER INDEX IF EXISTS idx_mydata_purchases_buyer_time RENAME TO idx_my_ip_purchases_buyer_time;
ALTER INDEX IF EXISTS idx_mydata_purchases_type_time RENAME TO idx_my_ip_purchases_type_time;
ALTER INDEX IF EXISTS idx_mydata_purchases_mydata_time RENAME TO idx_my_ip_purchases_ip_time;

-- my_ip_subscriptions indexes
ALTER INDEX IF EXISTS idx_mydata_subscriptions_time_mydata RENAME TO idx_my_ip_subscriptions_time_ip;
ALTER INDEX IF EXISTS idx_mydata_subscriptions_subscriber_time RENAME TO idx_my_ip_subscriptions_subscriber_time;
ALTER INDEX IF EXISTS idx_mydata_subscriptions_end_time RENAME TO idx_my_ip_subscriptions_end_time;
ALTER INDEX IF EXISTS idx_mydata_subscriptions_mydata_time RENAME TO idx_my_ip_subscriptions_ip_time;

-- my_ip_revenue indexes
ALTER INDEX IF EXISTS idx_mydata_revenue_time_mydata RENAME TO idx_my_ip_revenue_time_ip;
ALTER INDEX IF EXISTS idx_mydata_revenue_to_time RENAME TO idx_my_ip_revenue_to_time;
ALTER INDEX IF EXISTS idx_mydata_revenue_type_time RENAME TO idx_my_ip_revenue_type_time;
ALTER INDEX IF EXISTS idx_mydata_revenue_mydata_time RENAME TO idx_my_ip_revenue_ip_time;

-- my_ip_access_logs indexes
ALTER INDEX IF EXISTS idx_mydata_access_time_mydata RENAME TO idx_my_ip_access_time_ip;
ALTER INDEX IF EXISTS idx_mydata_access_user_time RENAME TO idx_my_ip_access_user_time;
ALTER INDEX IF EXISTS idx_mydata_access_type_time RENAME TO idx_my_ip_access_type_time;

-- ============================================================================
-- 5. RENAME PRIMARY KEY CONSTRAINTS BACK
-- ============================================================================

ALTER TABLE my_ip_data DROP CONSTRAINT IF EXISTS mydata_data_pkey;
ALTER TABLE my_ip_data ADD CONSTRAINT my_ip_data_pkey PRIMARY KEY (ip_id);

ALTER TABLE my_ip_purchases DROP CONSTRAINT IF EXISTS pk_mydata_purchases;
ALTER TABLE my_ip_purchases ADD CONSTRAINT pk_my_ip_purchases PRIMARY KEY (id, time);

ALTER TABLE my_ip_subscriptions DROP CONSTRAINT IF EXISTS pk_mydata_subscriptions;
ALTER TABLE my_ip_subscriptions ADD CONSTRAINT pk_my_ip_subscriptions PRIMARY KEY (id, time);

ALTER TABLE my_ip_revenue DROP CONSTRAINT IF EXISTS pk_mydata_revenue;
ALTER TABLE my_ip_revenue ADD CONSTRAINT pk_my_ip_revenue PRIMARY KEY (id, time);

ALTER TABLE my_ip_access_logs DROP CONSTRAINT IF EXISTS pk_mydata_access_logs;
ALTER TABLE my_ip_access_logs ADD CONSTRAINT pk_my_ip_access_logs PRIMARY KEY (id, time);

-- ============================================================================
-- 6. RECREATE TIMESCALEDB CONTINUOUS AGGREGATES
-- ============================================================================

-- Daily revenue analytics aggregate
CREATE MATERIALIZED VIEW my_ip_daily_revenue
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 day', time) AS day,
    ip_id,
    to_address AS creator,
    revenue_type,
    SUM(amount) AS daily_revenue,
    COUNT(*) AS transaction_count
FROM my_ip_revenue
GROUP BY time_bucket('1 day', time), ip_id, to_address, revenue_type
WITH NO DATA;

-- Enable automatic refresh for daily revenue
SELECT add_continuous_aggregate_policy('my_ip_daily_revenue',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

-- Daily access statistics aggregate
CREATE MATERIALIZED VIEW my_ip_daily_access
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 day', time) AS day,
    ip_id,
    access_type,
    COUNT(DISTINCT user_address) AS unique_users,
    COUNT(*) AS total_accesses
FROM my_ip_access_logs
GROUP BY time_bucket('1 day', time), ip_id, access_type
WITH NO DATA;

-- Enable automatic refresh for daily access
SELECT add_continuous_aggregate_policy('my_ip_daily_access',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

-- Popular data tracking aggregate (hourly for real-time popularity)
CREATE MATERIALIZED VIEW my_ip_popular_data
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 hour', time) AS hour,
    ip_id,
    COUNT(DISTINCT buyer) AS unique_purchasers,
    SUM(CASE WHEN purchase_type = 'one_time' THEN 1 ELSE 0 END) AS one_time_purchases,
    SUM(CASE WHEN purchase_type = 'subscription' THEN 1 ELSE 0 END) AS subscriptions,
    SUM(price) AS total_revenue
FROM my_ip_purchases
GROUP BY time_bucket('1 hour', time), ip_id
WITH NO DATA;

-- Enable automatic refresh for popular data
SELECT add_continuous_aggregate_policy('my_ip_popular_data',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '15 minutes');

-- ============================================================================
-- 7. RECREATE VIEWS
-- ============================================================================

-- Active marketplace data view
CREATE OR REPLACE VIEW active_marketplace_data AS
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
FROM my_ip_data d;

-- Popular data view (last 30 days)
CREATE OR REPLACE VIEW popular_data_30d AS
SELECT 
    d.ip_id,
    d.owner,
    d.media_type,
    d.tags,
    COUNT(DISTINCT p.buyer) AS unique_purchasers,
    SUM(p.price) AS total_revenue,
    COUNT(p.id) AS total_purchases,
    MAX(p.time) AS last_purchase
FROM my_ip_data d
LEFT JOIN my_ip_purchases p ON d.ip_id = p.ip_id 
    AND p.time >= NOW() - INTERVAL '30 days'
GROUP BY d.ip_id, d.owner, d.media_type, d.tags
ORDER BY unique_purchasers DESC, total_revenue DESC;

-- Creator revenue summary view
CREATE OR REPLACE VIEW creator_revenue_summary AS
SELECT 
    d.owner AS creator,
    COUNT(DISTINCT d.ip_id) AS data_entries,
    SUM(r.amount) AS total_revenue,
    COUNT(DISTINCT r.from_address) AS unique_customers,
    MAX(r.time) AS last_revenue
FROM my_ip_data d
LEFT JOIN my_ip_revenue r ON d.ip_id = r.ip_id
GROUP BY d.owner
ORDER BY total_revenue DESC NULLS LAST;

-- ============================================================================
-- 8. RECREATE HELPER FUNCTIONS
-- ============================================================================

-- Function to check if user has access to data
CREATE OR REPLACE FUNCTION user_has_access(
    p_ip_id TEXT,
    p_user_address TEXT,
    p_current_time BIGINT DEFAULT EXTRACT(EPOCH FROM NOW())
) RETURNS BOOLEAN AS $$
DECLARE
    data_owner TEXT;
    subscription_end BIGINT;
    has_purchase BOOLEAN := FALSE;
BEGIN
    -- Get data owner
    SELECT owner INTO data_owner FROM my_ip_data WHERE ip_id = p_ip_id;
    
    -- Owner always has access
    IF data_owner = p_user_address THEN
        RETURN TRUE;
    END IF;
    
    -- Check for one-time purchase
    SELECT TRUE INTO has_purchase 
    FROM my_ip_purchases 
    WHERE ip_id = p_ip_id AND buyer = p_user_address AND purchase_type = 'one_time'
    LIMIT 1;
    
    IF has_purchase THEN
        RETURN TRUE;
    END IF;
    
    -- Check for active subscription
    SELECT MAX(subscription_end) INTO subscription_end
    FROM my_ip_subscriptions 
    WHERE ip_id = p_ip_id AND subscriber = p_user_address;
    
    IF subscription_end IS NOT NULL AND subscription_end >= p_current_time THEN
        RETURN TRUE;
    END IF;
    
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql;

-- Function to get data pricing info
CREATE OR REPLACE FUNCTION get_data_pricing(p_ip_id TEXT)
RETURNS TABLE(
    ip_id TEXT,
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
        d.ip_id,
        d.one_time_price,
        d.subscription_price,
        d.subscription_duration_days,
        COALESCE(p.purchaser_count, 0) AS total_purchasers,
        COALESCE(s.subscriber_count, 0) AS total_subscribers,
        COALESCE(r.total_revenue, 0) AS total_revenue
    FROM my_ip_data d
    LEFT JOIN (
        SELECT ip_id, COUNT(DISTINCT buyer) AS purchaser_count
        FROM my_ip_purchases 
        WHERE purchase_type = 'one_time'
        GROUP BY ip_id
    ) p ON d.ip_id = p.ip_id
    LEFT JOIN (
        SELECT ip_id, COUNT(DISTINCT subscriber) AS subscriber_count
        FROM my_ip_subscriptions
        GROUP BY ip_id
    ) s ON d.ip_id = s.ip_id
    LEFT JOIN (
        SELECT ip_id, SUM(amount) AS total_revenue
        FROM my_ip_revenue
        GROUP BY ip_id
    ) r ON d.ip_id = r.ip_id
    WHERE d.ip_id = p_ip_id;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- 9. UPDATE TABLE COMMENTS
-- ============================================================================

COMMENT ON TABLE my_ip_data IS 'Main marketplace data entries with metadata and pricing';
COMMENT ON TABLE my_ip_purchases IS 'Purchase records for one-time and subscription access (TimescaleDB)';
COMMENT ON TABLE my_ip_subscriptions IS 'Active subscription tracking with expiry times (TimescaleDB)';
COMMENT ON TABLE my_ip_revenue IS 'Revenue distribution and tracking (TimescaleDB)';
COMMENT ON TABLE my_ip_access_logs IS 'Access pattern analytics and logs (TimescaleDB)';

