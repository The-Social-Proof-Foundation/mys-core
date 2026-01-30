-- Fix typo: Rename reservatior_address to reserver_address in spt_reservations table
-- This fixes the column name mismatch that prevents inserts from working

-- ============================================================================
-- 1. DROP VIEWS THAT DEPEND ON THE COLUMN
-- ============================================================================

DROP VIEW IF EXISTS user_reservation_holdings CASCADE;

-- ============================================================================
-- 2. REMOVE COMPRESSION POLICY (if exists)
-- ============================================================================

-- Remove compression policy before renaming column
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_reservations'
    ) THEN
        PERFORM remove_compression_policy('spt_reservations', if_exists => true);
    END IF;
END $$;

-- ============================================================================
-- 3. DROP INDEXES THAT REFERENCE THE OLD COLUMN NAME
-- ============================================================================

DROP INDEX IF EXISTS idx_spt_reservations_reservatior_address;

-- ============================================================================
-- 4. RENAME THE COLUMN
-- ============================================================================

ALTER TABLE spt_reservations RENAME COLUMN reservatior_address TO reserver_address;

-- ============================================================================
-- 5. RECREATE INDEXES WITH CORRECT COLUMN NAME
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_spt_reservations_reserver_address ON spt_reservations(reserver_address);

-- ============================================================================
-- 6. UPDATE COMPRESSION CONFIGURATION
-- ============================================================================

-- Update compression segmentby with correct column name
-- Note: This will only affect future compressions. Existing compressed chunks
-- will retain the old segmentby until they are decompressed and recompressed.
DO $$
BEGIN
    -- Check if compression is enabled
    IF EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'spt_reservations'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        -- Update compression settings with correct column name
        -- This updates the configuration for future compressions
        ALTER TABLE spt_reservations SET (
            timescaledb.compress_segmentby = 'pool_id,reserver_address',
            timescaledb.compress_orderby = 'time DESC'
        );
    ELSE
        -- If compression wasn't enabled, enable it now with correct column name
        ALTER TABLE spt_reservations SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'pool_id,reserver_address',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- Re-add compression policy
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'spt_reservations'
    ) THEN
        PERFORM add_compression_policy('spt_reservations', INTERVAL '7 days');
    END IF;
END $$;

-- ============================================================================
-- 7. RECREATE VIEWS WITH CORRECT COLUMN NAME
-- ============================================================================

-- Recreate user_reservation_holdings view with correct column name
CREATE OR REPLACE VIEW user_reservation_holdings AS
SELECT
    s.reserver_address,
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
        WHERE sub.pool_id = s.pool_id AND sub.reserver_address = s.reserver_address
    )
    AND sp.time = (
        SELECT MAX(time) FROM spt_reservation_pools sub
        WHERE sub.pool_id = sp.pool_id
    )
    AND s.amount > 0
ORDER BY 
    s.reserved_at DESC;

