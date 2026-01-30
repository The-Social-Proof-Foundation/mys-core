-- Rollback: Rename spt_pools back to social_proof_token_pools

-- ============================================================================
-- 1. DROP VIEWS THAT DEPEND ON THE TABLE
-- ============================================================================

DROP VIEW IF EXISTS active_token_pools CASCADE;
DROP VIEW IF EXISTS popular_token_pools CASCADE;

-- ============================================================================
-- 2. REMOVE COMPRESSION POLICY
-- ============================================================================

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_pools'
    ) THEN
        PERFORM remove_compression_policy('spt_pools', if_exists => true);
    END IF;
END $$;

-- ============================================================================
-- 3. RENAME THE TABLE BACK
-- ============================================================================

ALTER TABLE spt_pools RENAME TO social_proof_token_pools;
ALTER TABLE social_proof_token_pools RENAME CONSTRAINT pk_spt_pools TO pk_social_proof_token_pools;

-- ============================================================================
-- 4. RECREATE COMPRESSION POLICY
-- ============================================================================

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
END $$;

-- ============================================================================
-- 5. RECREATE VIEWS WITH OLD TABLE NAME
-- ============================================================================

CREATE VIEW active_token_pools AS
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

