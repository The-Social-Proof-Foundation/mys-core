-- SOCIAL PROOF TOKEN SYSTEM WITH TIMESCALEDB INTEGRATION
-- Production-ready implementation for token pools, transactions, and auctions

-- ============================================================================
-- 1. CREATE CORE TABLES
-- ============================================================================

-- Social Proof Token Pools table with time dimension
CREATE TABLE IF NOT EXISTS social_proof_token_pools (
    id SERIAL NOT NULL,
    pool_id TEXT NOT NULL,
    token_type SMALLINT NOT NULL,  -- 1: Profile, 2: Post
    owner TEXT NOT NULL,
    associated_id TEXT NOT NULL,
    symbol TEXT NOT NULL,
    name TEXT NOT NULL,
    circulating_supply BIGINT NOT NULL,
    base_price BIGINT NOT NULL,
    quadratic_coefficient BIGINT NOT NULL,
    created_at BIGINT NOT NULL, 
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_social_proof_token_pools PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('social_proof_token_pools', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on token pools table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'social_proof_token_pools'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE social_proof_token_pools SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'pool_id,owner,associated_id',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- Token Holdings table with time dimension
CREATE TABLE IF NOT EXISTS spt_holdings (
    id SERIAL NOT NULL,
    pool_id TEXT NOT NULL,
    holder_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    acquired_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_spt_holdings PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_holdings', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on holdings table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_holdings'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_holdings SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'pool_id,holder_address',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- Token Transactions table with time dimension
CREATE TABLE IF NOT EXISTS spt_transactions (
    id SERIAL NOT NULL,
    pool_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,  -- 'BUY', 'SELL'
    sender TEXT NOT NULL,
    amount BIGINT NOT NULL,
    mys_amount BIGINT NOT NULL,
    fee_amount BIGINT NOT NULL,
    creator_fee BIGINT NOT NULL,
    platform_fee BIGINT NOT NULL,
    treasury_fee BIGINT NOT NULL,
    price BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_spt_transactions PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_transactions', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on transactions table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_transactions'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_transactions SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'pool_id,transaction_type,sender',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

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
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_auction_pools'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_auction_pools SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'auction_id,associated_id,owner',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

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
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_auction_contributions'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_auction_contributions SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'auction_id,contributor_address',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- Token Price History table with time dimension
CREATE TABLE IF NOT EXISTS spt_price_history (
    id SERIAL NOT NULL,
    pool_id TEXT NOT NULL,
    price BIGINT NOT NULL,
    circulating_supply BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_spt_price_history PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable with 1-day chunks for price history
SELECT create_hypertable('spt_price_history', 'time', if_not_exists => TRUE, chunk_time_interval => INTERVAL '1 day');

-- Enable compression on price history table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_price_history'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_price_history SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'pool_id',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- ============================================================================
-- 2. CREATE INDEXES
-- ============================================================================

-- Pool indexes
CREATE INDEX IF NOT EXISTS idx_spt_pools_pool_id ON social_proof_token_pools(pool_id);
CREATE INDEX IF NOT EXISTS idx_spt_pools_owner ON social_proof_token_pools(owner);
CREATE INDEX IF NOT EXISTS idx_spt_pools_token_type ON social_proof_token_pools(token_type);
CREATE INDEX IF NOT EXISTS idx_spt_pools_associated_id ON social_proof_token_pools(associated_id);

-- Holdings indexes
CREATE INDEX IF NOT EXISTS idx_spt_holdings_pool_id ON spt_holdings(pool_id);
CREATE INDEX IF NOT EXISTS idx_spt_holdings_holder_address ON spt_holdings(holder_address);

-- Transactions indexes
CREATE INDEX IF NOT EXISTS idx_spt_transactions_pool_id ON spt_transactions(pool_id);
CREATE INDEX IF NOT EXISTS idx_spt_transactions_sender ON spt_transactions(sender);
CREATE INDEX IF NOT EXISTS idx_spt_transactions_transaction_type ON spt_transactions(transaction_type);

-- Auction pools indexes
CREATE INDEX IF NOT EXISTS idx_spt_auction_pools_auction_id ON spt_auction_pools(auction_id);
CREATE INDEX IF NOT EXISTS idx_spt_auction_pools_associated_id ON spt_auction_pools(associated_id);
CREATE INDEX IF NOT EXISTS idx_spt_auction_pools_owner ON spt_auction_pools(owner);
CREATE INDEX IF NOT EXISTS idx_spt_auction_pools_status ON spt_auction_pools(status);

-- Auction contributions indexes
CREATE INDEX IF NOT EXISTS idx_spt_auction_contributions_auction_id ON spt_auction_contributions(auction_id);
CREATE INDEX IF NOT EXISTS idx_spt_auction_contributions_contributor_address ON spt_auction_contributions(contributor_address);

-- Price history indexes
CREATE INDEX IF NOT EXISTS idx_spt_price_history_pool_id ON spt_price_history(pool_id);

-- ============================================================================
-- 3. CREATE VIEWS AND MATERIALIZED VIEWS
-- ============================================================================

-- Create hourly price aggregation view
CREATE MATERIALIZED VIEW IF NOT EXISTS spt_price_hourly
WITH (timescaledb.continuous) AS
SELECT 
    pool_id,
    time_bucket('1 hour', time) AS bucket,
    FIRST(price, time) AS open,
    MAX(price) AS high,
    MIN(price) AS low,
    LAST(price, time) AS close,
    LAST(circulating_supply, time) AS circulating_supply
FROM spt_price_history
GROUP BY pool_id, bucket
WITH NO DATA;

-- Create daily price aggregation view
CREATE MATERIALIZED VIEW IF NOT EXISTS spt_price_daily
WITH (timescaledb.continuous) AS
SELECT 
    pool_id,
    time_bucket('1 day', time) AS bucket,
    FIRST(price, time) AS open,
    MAX(price) AS high,
    MIN(price) AS low,
    LAST(price, time) AS close,
    LAST(circulating_supply, time) AS circulating_supply
FROM spt_price_history
GROUP BY pool_id, bucket
WITH NO DATA;

-- Create view for active token pools with their current price
CREATE OR REPLACE VIEW active_token_pools AS
SELECT
    p.*,
    ph.price AS current_price
FROM 
    social_proof_token_pools p
JOIN 
    (
        SELECT DISTINCT ON (pool_id) 
            pool_id, 
            price
        FROM 
            spt_price_history
        ORDER BY 
            pool_id, time DESC
    ) ph ON p.pool_id = ph.pool_id;

-- Create view for popular token pools based on transaction volume
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
-- 4. SET UP AUTOMATIC COMPRESSION POLICIES
-- ============================================================================

-- Add compression policies to compress chunks after 7 days
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'social_proof_token_pools'
    ) THEN
        PERFORM add_compression_policy('social_proof_token_pools', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_holdings'
    ) THEN
        PERFORM add_compression_policy('spt_holdings', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_transactions'
    ) THEN
        PERFORM add_compression_policy('spt_transactions', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_auction_pools'
    ) THEN
        PERFORM add_compression_policy('spt_auction_pools', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_auction_contributions'
    ) THEN
        PERFORM add_compression_policy('spt_auction_contributions', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_price_history'
    ) THEN
        PERFORM add_compression_policy('spt_price_history', INTERVAL '7 days');
    END IF;
END $$;

-- ============================================================================
-- 5. CREATE REFRESH FUNCTIONS AND POLICIES FOR MATERIALIZED VIEWS
-- ============================================================================

-- Create refresh function for hourly price materialized view
CREATE OR REPLACE FUNCTION refresh_spt_price_hourly()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW spt_price_hourly;
END;
$$ LANGUAGE plpgsql;

-- Create refresh function for daily price materialized view
CREATE OR REPLACE FUNCTION refresh_spt_price_daily()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW spt_price_daily;
END;
$$ LANGUAGE plpgsql;

-- Create a scheduled job to refresh materialized views
DO $$
BEGIN
    -- Create a job to refresh hourly price view every hour
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'refresh_spt_price_hourly'
    ) THEN
        PERFORM add_job(
            'refresh_spt_price_hourly',
            '1 hour',
            initial_start => now()
        );
    END IF;
    
    -- Create a job to refresh daily price view every day
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'refresh_spt_price_daily'
    ) THEN
        PERFORM add_job(
            'refresh_spt_price_daily',
            '1 day',
            initial_start => now()
        );
    END IF;
END $$; 