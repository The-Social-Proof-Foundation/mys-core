-- REPLACE AUCTION SYSTEM WITH STAKING SYSTEM
-- Production-ready implementation for stake pools and stakes

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

-- Stake Pools table with time dimension
CREATE TABLE IF NOT EXISTS spt_stake_pools (
    id SERIAL NOT NULL,
    pool_id VARCHAR NOT NULL,
    associated_id VARCHAR NOT NULL,
    token_type SMALLINT NOT NULL,  -- 1: Profile, 2: Post
    owner VARCHAR NOT NULL,
    total_staked BIGINT NOT NULL DEFAULT 0,
    required_threshold BIGINT NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'active', -- 'active', 'threshold_met', 'converted'
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL,
    CONSTRAINT pk_spt_stake_pools PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_stake_pools', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on stake pools table
ALTER TABLE spt_stake_pools SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'pool_id,associated_id,owner',
    timescaledb.compress_orderby = 'time DESC'
);

-- Individual Stakes table with time dimension
CREATE TABLE IF NOT EXISTS spt_stakes (
    id SERIAL NOT NULL,
    pool_id VARCHAR NOT NULL,
    staker_address VARCHAR NOT NULL,
    amount BIGINT NOT NULL,
    staked_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL,
    CONSTRAINT pk_spt_stakes PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_stakes', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on stakes table
ALTER TABLE spt_stakes SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'pool_id,staker_address',
    timescaledb.compress_orderby = 'time DESC'
);

-- Exchange Configuration table to track thresholds and settings
CREATE TABLE IF NOT EXISTS spt_exchange_config (
    id SERIAL NOT NULL,
    updated_by VARCHAR NOT NULL,
    post_threshold BIGINT NOT NULL,
    profile_threshold BIGINT NOT NULL,
    max_individual_stake_bps BIGINT NOT NULL,
    total_fee_bps BIGINT NOT NULL,
    creator_fee_bps BIGINT NOT NULL,
    platform_fee_bps BIGINT NOT NULL,
    treasury_fee_bps BIGINT NOT NULL,
    base_price BIGINT NOT NULL,
    quadratic_coefficient BIGINT NOT NULL,
    ecosystem_treasury VARCHAR NOT NULL,
    max_hold_percent_bps BIGINT NOT NULL,
    trading_halted BOOLEAN NOT NULL DEFAULT false,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id VARCHAR NOT NULL,
    CONSTRAINT pk_spt_exchange_config PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_exchange_config', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on config table
ALTER TABLE spt_exchange_config SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'updated_by',
    timescaledb.compress_orderby = 'time DESC'
);

-- ============================================================================
-- 3. CREATE INDEXES
-- ============================================================================

-- Stake pool indexes
CREATE INDEX IF NOT EXISTS idx_spt_stake_pools_pool_id ON spt_stake_pools(pool_id);
CREATE INDEX IF NOT EXISTS idx_spt_stake_pools_associated_id ON spt_stake_pools(associated_id);
CREATE INDEX IF NOT EXISTS idx_spt_stake_pools_owner ON spt_stake_pools(owner);
CREATE INDEX IF NOT EXISTS idx_spt_stake_pools_status ON spt_stake_pools(status);
CREATE INDEX IF NOT EXISTS idx_spt_stake_pools_token_type ON spt_stake_pools(token_type);

-- Stakes indexes
CREATE INDEX IF NOT EXISTS idx_spt_stakes_pool_id ON spt_stakes(pool_id);
CREATE INDEX IF NOT EXISTS idx_spt_stakes_staker_address ON spt_stakes(staker_address);

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

-- Create view for active stake pools with aggregated data
CREATE OR REPLACE VIEW active_stake_pools AS
SELECT
    sp.pool_id,
    sp.associated_id,
    sp.token_type,
    sp.owner,
    sp.total_staked,
    sp.required_threshold,
    sp.status,
    sp.created_at,
    (sp.total_staked >= sp.required_threshold) AS threshold_met,
    COUNT(s.id) AS staker_count,
    COALESCE(MAX(s.time), sp.time) AS last_activity
FROM 
    spt_stake_pools sp
LEFT JOIN 
    spt_stakes s ON sp.pool_id = s.pool_id
WHERE 
    sp.time = (
        SELECT MAX(time) FROM spt_stake_pools sub
        WHERE sub.pool_id = sp.pool_id
    )
GROUP BY 
    sp.pool_id, sp.associated_id, sp.token_type, sp.owner, 
    sp.total_staked, sp.required_threshold, sp.status, sp.created_at, sp.time
ORDER BY 
    sp.total_staked DESC;

-- Create view for user stake holdings across all pools
CREATE OR REPLACE VIEW user_stake_holdings AS
SELECT
    s.staker_address,
    s.pool_id,
    sp.associated_id,
    sp.token_type,
    sp.owner,
    s.amount,
    s.staked_at,
    sp.total_staked,
    sp.required_threshold,
    (sp.total_staked >= sp.required_threshold) AS threshold_met,
    sp.status AS pool_status
FROM 
    spt_stakes s
JOIN 
    spt_stake_pools sp ON s.pool_id = sp.pool_id
WHERE 
    s.time = (
        SELECT MAX(time) FROM spt_stakes sub
        WHERE sub.pool_id = s.pool_id AND sub.staker_address = s.staker_address
    )
    AND sp.time = (
        SELECT MAX(time) FROM spt_stake_pools sub
        WHERE sub.pool_id = sp.pool_id
    )
    AND s.amount > 0
ORDER BY 
    s.amount DESC;

-- ============================================================================
-- 5. SET UP AUTOMATIC COMPRESSION POLICIES
-- ============================================================================

-- Add compression policies to compress chunks after 7 days
SELECT add_compression_policy('spt_stake_pools', INTERVAL '7 days');
SELECT add_compression_policy('spt_stakes', INTERVAL '7 days');
SELECT add_compression_policy('spt_exchange_config', INTERVAL '7 days');

-- ============================================================================
-- 6. CREATE FUNCTIONS FOR STAKE POOL MANAGEMENT
-- ============================================================================

-- Function to get current exchange configuration
CREATE OR REPLACE FUNCTION get_current_exchange_config()
RETURNS TABLE(
    post_threshold BIGINT,
    profile_threshold BIGINT,
    max_individual_stake_bps BIGINT,
    trading_halted BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        c.post_threshold,
        c.profile_threshold, 
        c.max_individual_stake_bps,
        c.trading_halted
    FROM spt_exchange_config c
    ORDER BY c.time DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

-- Function to check if stake pool threshold is met
CREATE OR REPLACE FUNCTION is_stake_threshold_met(pool_id_param VARCHAR)
RETURNS BOOLEAN AS $$
DECLARE
    result BOOLEAN;
BEGIN
    SELECT (sp.total_staked >= sp.required_threshold) INTO result
    FROM spt_stake_pools sp
    WHERE sp.pool_id = pool_id_param
    ORDER BY sp.time DESC
    LIMIT 1;
    
    RETURN COALESCE(result, false);
END;
$$ LANGUAGE plpgsql; 