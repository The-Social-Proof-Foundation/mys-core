-- ROLLBACK STAKING SYSTEM TO AUCTION SYSTEM
-- Restore auction tables and remove staking tables

-- ============================================================================
-- 1. DROP STAKING SYSTEM
-- ============================================================================

-- Drop views that depend on staking tables
DROP VIEW IF EXISTS active_reservation_pools CASCADE;
DROP VIEW IF EXISTS user_reservation_holdings CASCADE;
DROP VIEW IF EXISTS popular_token_pools CASCADE;

-- Drop functions
DROP FUNCTION IF EXISTS get_current_exchange_config() CASCADE;
DROP FUNCTION IF EXISTS is_reservation_threshold_met(TEXT) CASCADE;

-- Remove compression policies for staking tables
SELECT remove_compression_policy('spt_reservation_pools', if_exists => true);
SELECT remove_compression_policy('spt_reservations', if_exists => true);
SELECT remove_compression_policy('spt_exchange_config', if_exists => true);

-- Drop staking tables and indexes
DROP INDEX IF EXISTS idx_spt_reservation_pools_pool_id;
DROP INDEX IF EXISTS idx_spt_reservation_pools_associated_id;
DROP INDEX IF EXISTS idx_spt_reservation_pools_owner;
DROP INDEX IF EXISTS idx_spt_reservation_pools_status;
DROP INDEX IF EXISTS idx_spt_reservation_pools_token_type;
DROP TABLE IF EXISTS spt_reservation_pools CASCADE;

DROP INDEX IF EXISTS idx_spt_reservations_pool_id;
DROP INDEX IF EXISTS idx_spt_reservations_reservatior_address;
DROP TABLE IF EXISTS spt_reservations CASCADE;

DROP INDEX IF EXISTS idx_spt_exchange_config_updated_by;
DROP TABLE IF EXISTS spt_exchange_config CASCADE;

-- ============================================================================
-- 2. RESTORE AUCTION SYSTEM
-- ============================================================================

-- Token Auction Pools table with time dimension
CREATE TABLE IF NOT EXISTS spt_auction_pools (
    id SERIAL NOT NULL,
    auction_id TEXT NOT NULL,
    associated_id TEXT NOT NULL,
    token_type SMALLINT NOT NULL,
    owner TEXT NOT NULL,
    status SMALLINT NOT NULL,  -- 0: Pending, 1: Active, 2: Ended, 3: Finalized
    start_time BIGINT NOT NULL,
    duration BIGINT NOT NULL,
    total_contribution BIGINT NOT NULL,
    total_tokens BIGINT NOT NULL,
    finalized_at BIGINT,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_spt_auction_pools PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_auction_pools', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on auction pools table
ALTER TABLE spt_auction_pools SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'auction_id,associated_id,owner',
    timescaledb.compress_orderby = 'time DESC'
);

-- Auction Contributions table with time dimension
CREATE TABLE IF NOT EXISTS spt_auction_contributions (
    id SERIAL NOT NULL,
    auction_id TEXT NOT NULL,
    contributor_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    contributed_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_spt_auction_contributions PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_auction_contributions', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on auction contributions table
ALTER TABLE spt_auction_contributions SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'auction_id,contributor_address',
    timescaledb.compress_orderby = 'time DESC'
);

-- ============================================================================
-- 3. CREATE INDEXES
-- ============================================================================

-- Auction pools indexes
CREATE INDEX IF NOT EXISTS idx_spt_auction_pools_auction_id ON spt_auction_pools(auction_id);
CREATE INDEX IF NOT EXISTS idx_spt_auction_pools_associated_id ON spt_auction_pools(associated_id);
CREATE INDEX IF NOT EXISTS idx_spt_auction_pools_owner ON spt_auction_pools(owner);
CREATE INDEX IF NOT EXISTS idx_spt_auction_pools_status ON spt_auction_pools(status);

-- Auction contributions indexes
CREATE INDEX IF NOT EXISTS idx_spt_auction_contributions_auction_id ON spt_auction_contributions(auction_id);
CREATE INDEX IF NOT EXISTS idx_spt_auction_contributions_contributor_address ON spt_auction_contributions(contributor_address);

-- ============================================================================
-- 4. RESTORE VIEWS
-- ============================================================================

-- Restore original popular token pools view
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
    SUM(CASE WHEN t.transaction_type = 'BUY' THEN t.amount ELSE 0 END) AS buy_volume,
    SUM(CASE WHEN t.transaction_type = 'SELL' THEN t.amount ELSE 0 END) AS sell_volume,
    SUM(t.mys_amount) AS total_volume
FROM 
    social_proof_token_pools p
JOIN 
    spt_transactions t ON p.pool_id = t.pool_id
WHERE 
    t.time > NOW() - INTERVAL '7 days'
GROUP BY 
    p.pool_id, p.token_type, p.owner, p.associated_id, p.symbol, p.name, p.circulating_supply
ORDER BY 
    total_volume DESC;

-- ============================================================================
-- 5. SET UP AUTOMATIC COMPRESSION POLICIES
-- ============================================================================

-- Add compression policies to compress chunks after 7 days
SELECT add_compression_policy('spt_auction_pools', INTERVAL '7 days');
SELECT add_compression_policy('spt_auction_contributions', INTERVAL '7 days'); 