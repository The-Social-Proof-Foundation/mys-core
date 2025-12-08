-- Production-ready implementation for reservation pools and reservations

-- ============================================================================
-- 1. REMOVE AUCTION SYSTEM
-- ============================================================================

-- Drop auction-related views that depend on auction tables
DROP VIEW IF EXISTS popular_token_pools CASCADE;

-- Remove compression policies for auction tables
SELECT remove_compression_policy('spt_auction_pools', if_exists => true);
SELECT remove_compression_policy('spt_auction_contributions', if_exists => true);

-- Drop auction contributions table and indexes
DROP INDEX IF EXISTS idx_spt_auction_contributions_auction_id;
DROP INDEX IF EXISTS idx_spt_auction_contributions_contributor_address;
DROP TABLE IF EXISTS spt_auction_contributions CASCADE;

-- Drop auction pools table and indexes  
DROP INDEX IF EXISTS idx_spt_auction_pools_auction_id;
DROP INDEX IF EXISTS idx_spt_auction_pools_associated_id;
DROP INDEX IF EXISTS idx_spt_auction_pools_owner;
DROP INDEX IF EXISTS idx_spt_auction_pools_status;
DROP TABLE IF EXISTS spt_auction_pools CASCADE;

-- ============================================================================
-- 2. CREATE STAKING SYSTEM TABLES
-- ============================================================================

-- Reservation Pools table with time dimension
CREATE TABLE IF NOT EXISTS spt_reservation_pools (
    id SERIAL NOT NULL,
    pool_id TEXT NOT NULL,
    associated_id TEXT NOT NULL,
    token_type SMALLINT NOT NULL,  -- 1: Profile, 2: Post
    owner TEXT NOT NULL,
    total_reserved BIGINT NOT NULL DEFAULT 0,
    required_threshold BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active', -- 'active', 'threshold_met', 'converted'
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_spt_reservation_pools PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_reservation_pools', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on reservation pools table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_reservation_pools'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_reservation_pools SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'pool_id,associated_id,owner',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- Individual Reservations table with time dimension
CREATE TABLE IF NOT EXISTS spt_reservations (
    id SERIAL NOT NULL,
    pool_id TEXT NOT NULL,
    reservatior_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    reserved_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_spt_reservations PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_reservations', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on reservations table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_reservations'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_reservations SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'pool_id,reservatior_address',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- Exchange Configuration table to track thresholds and settings
CREATE TABLE IF NOT EXISTS spt_exchange_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL,
    post_threshold BIGINT NOT NULL,
    profile_threshold BIGINT NOT NULL,
    max_individual_reservation_bps BIGINT NOT NULL,
    total_fee_bps BIGINT NOT NULL,
    creator_fee_bps BIGINT NOT NULL,
    platform_fee_bps BIGINT NOT NULL,
    treasury_fee_bps BIGINT NOT NULL,
    base_price BIGINT NOT NULL,
    quadratic_coefficient BIGINT NOT NULL,
    ecosystem_treasury TEXT NOT NULL,
    max_hold_percent_bps BIGINT NOT NULL,
    trading_halted BOOLEAN NOT NULL DEFAULT false,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_spt_exchange_config PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_exchange_config', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on config table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_exchange_config'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_exchange_config SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'updated_by',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- ============================================================================
-- 3. CREATE INDEXES
-- ============================================================================

-- Reservation pool indexes
CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_pool_id ON spt_reservation_pools(pool_id);
CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_associated_id ON spt_reservation_pools(associated_id);
CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_owner ON spt_reservation_pools(owner);
CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_status ON spt_reservation_pools(status);
CREATE INDEX IF NOT EXISTS idx_spt_reservation_pools_token_type ON spt_reservation_pools(token_type);

-- Reservations indexes
CREATE INDEX IF NOT EXISTS idx_spt_reservations_pool_id ON spt_reservations(pool_id);
CREATE INDEX IF NOT EXISTS idx_spt_reservations_reservatior_address ON spt_reservations(reservatior_address);

-- Exchange config indexes
CREATE INDEX IF NOT EXISTS idx_spt_exchange_config_updated_by ON spt_exchange_config(updated_by);

-- ============================================================================
-- 4. CREATE VIEWS
-- ============================================================================

-- Recreate popular token pools view without auction dependencies
CREATE OR REPLACE VIEW popular_token_pools AS
SELECT
    p.pool_id,
    p.token_type,
    p.owner,
    p.associated_id,
    p.symbol,
    p.name,
    p.circulating_supply,
    COUNT(t.id) AS transaction_count,
    SUM(CASE WHEN t.transaction_type = 'BUY' THEN t.mys_amount ELSE 0 END) AS buy_volume,
    SUM(CASE WHEN t.transaction_type = 'SELL' THEN t.mys_amount ELSE 0 END) AS sell_volume,
    SUM(t.mys_amount) AS total_volume,
    COALESCE(ph.price, p.base_price) AS current_price
FROM 
    social_proof_token_pools p
JOIN 
    spt_transactions t ON p.pool_id = t.pool_id
LEFT JOIN (
    SELECT DISTINCT ON (pool_id) pool_id, price
    FROM spt_price_history
    ORDER BY pool_id, time DESC
) ph ON p.pool_id = ph.pool_id
WHERE 
    t.time > NOW() - INTERVAL '7 days'
    AND p.time = (
        SELECT MAX(time) FROM social_proof_token_pools sub
        WHERE sub.pool_id = p.pool_id
    )
GROUP BY 
    p.pool_id, p.token_type, p.owner, p.associated_id, p.symbol, p.name, 
    p.circulating_supply, p.base_price, ph.price
ORDER BY 
    total_volume DESC;

-- Create view for active reservation pools with aggregated data
CREATE OR REPLACE VIEW active_reservation_pools AS
SELECT
    sp.pool_id,
    sp.associated_id,
    sp.token_type,
    sp.owner,
    sp.total_reserved,
    sp.required_threshold,
    sp.status,
    sp.created_at,
    (sp.total_reserved >= sp.required_threshold) AS threshold_met,
    COUNT(s.id) AS reservatior_count,
    COALESCE(MAX(s.time), sp.time) AS last_activity
FROM 
    spt_reservation_pools sp
LEFT JOIN 
    spt_reservations s ON sp.pool_id = s.pool_id
WHERE 
    sp.time = (
        SELECT MAX(time) FROM spt_reservation_pools sub
        WHERE sub.pool_id = sp.pool_id
    )
GROUP BY 
    sp.pool_id, sp.associated_id, sp.token_type, sp.owner, 
    sp.total_reserved, sp.required_threshold, sp.status, sp.created_at, sp.time
ORDER BY 
    sp.total_reserved DESC;

-- Create view for user reservation holdings across all pools
CREATE OR REPLACE VIEW user_reservation_holdings AS
SELECT
    s.reservatior_address,
    s.pool_id,
    sp.associated_id,
    sp.token_type,
    sp.owner,
    s.amount,
    s.reserved_at,
    sp.total_reserved,
    sp.required_threshold,
    (sp.total_reserved >= sp.required_threshold) AS threshold_met,
    sp.status AS pool_status
FROM 
    spt_reservations s
JOIN 
    spt_reservation_pools sp ON s.pool_id = sp.pool_id
WHERE 
    s.time = (
        SELECT MAX(time) FROM spt_reservations sub
        WHERE sub.pool_id = s.pool_id AND sub.reservatior_address = s.reservatior_address
    )
    AND sp.time = (
        SELECT MAX(time) FROM spt_reservation_pools sub
        WHERE sub.pool_id = sp.pool_id
    )
    AND s.amount > 0
ORDER BY 
    s.amount DESC;

-- ============================================================================
-- 5. SET UP AUTOMATIC COMPRESSION POLICIES
-- ============================================================================

-- Add compression policies to compress chunks after 7 days
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_reservation_pools'
    ) THEN
        PERFORM add_compression_policy('spt_reservation_pools', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_reservations'
    ) THEN
        PERFORM add_compression_policy('spt_reservations', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_exchange_config'
    ) THEN
        PERFORM add_compression_policy('spt_exchange_config', INTERVAL '7 days');
    END IF;
END $$;

-- ============================================================================
-- 6. CREATE FUNCTIONS FOR RESERVATION POOL MANAGEMENT
-- ============================================================================

-- Function to get current exchange configuration
CREATE OR REPLACE FUNCTION get_current_exchange_config()
RETURNS TABLE(
    post_threshold BIGINT,
    profile_threshold BIGINT,
    max_individual_reservation_bps BIGINT,
    trading_halted BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        c.post_threshold,
        c.profile_threshold, 
        c.max_individual_reservation_bps,
        c.trading_halted
    FROM spt_exchange_config c
    ORDER BY c.time DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

-- Function to check if reservation pool threshold is met
CREATE OR REPLACE FUNCTION is_reservation_threshold_met(pool_id_param TEXT)
RETURNS BOOLEAN AS $$
DECLARE
    result BOOLEAN;
BEGIN
    SELECT (sp.total_reserved >= sp.required_threshold) INTO result
    FROM spt_reservation_pools sp
    WHERE sp.pool_id = pool_id_param
    ORDER BY sp.time DESC
    LIMIT 1;
    
    RETURN COALESCE(result, false);
END;
$$ LANGUAGE plpgsql; 