-- Rename social_proof_token_pools table to spt_pools to match naming convention
-- This migration renames the table and updates all dependent objects

-- ============================================================================
-- 1. DROP VIEWS THAT DEPEND ON THE TABLE
-- ============================================================================

-- Drop views that reference the old table name
DROP VIEW IF EXISTS active_token_pools CASCADE;
DROP VIEW IF EXISTS popular_token_pools CASCADE;

-- ============================================================================
-- 2. REMOVE COMPRESSION POLICY (if exists)
-- ============================================================================

-- Remove compression policy before renaming
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'social_proof_token_pools'
    ) THEN
        PERFORM remove_compression_policy('social_proof_token_pools', if_exists => true);
    END IF;
END $$;

-- ============================================================================
-- 3. RENAME THE TABLE
-- ============================================================================

-- Rename the table
ALTER TABLE social_proof_token_pools RENAME TO spt_pools;

-- Rename the primary key constraint
ALTER TABLE spt_pools RENAME CONSTRAINT pk_social_proof_token_pools TO pk_spt_pools;

-- ============================================================================
-- 4. RECREATE INDEXES (indexes keep their names but need to be on new table)
-- ============================================================================

-- The indexes already have the correct names (idx_spt_pools_*), so they should
-- automatically be associated with the renamed table. However, we verify they exist.
-- If they don't exist for some reason, recreate them.
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE tablename = 'spt_pools' 
        AND indexname = 'idx_spt_pools_pool_id'
    ) THEN
        CREATE INDEX idx_spt_pools_pool_id ON spt_pools(pool_id);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE tablename = 'spt_pools' 
        AND indexname = 'idx_spt_pools_owner'
    ) THEN
        CREATE INDEX idx_spt_pools_owner ON spt_pools(owner);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE tablename = 'spt_pools' 
        AND indexname = 'idx_spt_pools_token_type'
    ) THEN
        CREATE INDEX idx_spt_pools_token_type ON spt_pools(token_type);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE tablename = 'spt_pools' 
        AND indexname = 'idx_spt_pools_associated_id'
    ) THEN
        CREATE INDEX idx_spt_pools_associated_id ON spt_pools(associated_id);
    END IF;
END $$;

-- ============================================================================
-- 5. RECREATE COMPRESSION POLICY
-- ============================================================================

-- Re-enable compression on the renamed table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_pools'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_pools SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'pool_id,owner,associated_id',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- Add compression policy back
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_pools'
    ) THEN
        PERFORM add_compression_policy('spt_pools', INTERVAL '7 days');
    END IF;
END $$;

-- ============================================================================
-- 6. RECREATE VIEWS WITH NEW TABLE NAME
-- ============================================================================

-- Recreate active_token_pools view
CREATE VIEW active_token_pools AS
SELECT
    p.*,
    ph.price AS current_price
FROM 
    spt_pools p
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

-- Recreate popular_token_pools view
CREATE VIEW popular_token_pools AS
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
    spt_pools p
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
        SELECT MAX(time) FROM spt_pools sub
        WHERE sub.pool_id = p.pool_id
    )
GROUP BY 
    p.pool_id, p.token_type, p.owner, p.associated_id, p.symbol, p.name, 
    p.circulating_supply, p.base_price, ph.price
ORDER BY 
    total_volume DESC;

