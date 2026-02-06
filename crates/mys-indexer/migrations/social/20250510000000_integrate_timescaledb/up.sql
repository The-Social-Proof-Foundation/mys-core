-- CONSOLIDATED MIGRATION FOR TIMESCALEDB INTEGRATION
-- Combines the following migrations:
-- 1. Add TimescaleDB Extension
-- 2. Convert Event Tables to Hypertables
-- 3. Add Continuous Aggregates
-- 4. Add Checkpoint Processing Table
-- 5. Add Refresh Continuous Aggregates

-- ============================================================================
-- 1. ENABLE TIMESCALEDB EXTENSION
-- ============================================================================
-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;

-- ============================================================================
-- 2. CONVERT EVENT TABLES TO HYPERTABLES
-- ============================================================================
-- Convert social_graph_events to hypertable
-- The primary key must include the partitioning column (created_at)
-- Step 1: Create index on created_at for hypertable conversion
CREATE INDEX IF NOT EXISTS idx_social_graph_events_created_at ON social_graph_events(created_at);

-- Step 2: Drop primary key constraint if it exists
ALTER TABLE social_graph_events DROP CONSTRAINT IF EXISTS social_graph_events_pkey CASCADE;

-- Step 3: Add created_at to primary key if it doesn't already exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'social_graph_events_pkey'
    ) THEN
        ALTER TABLE social_graph_events ADD PRIMARY KEY (id, created_at);
    END IF;
END $$;

-- Step 4: Convert to hypertable
SELECT create_hypertable('social_graph_events', 'created_at', if_not_exists => TRUE, 
                         migrate_data => TRUE, chunk_time_interval => INTERVAL '1 week');

-- Step 5: Add compression policy (compress chunks older than 7 days)
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'social_graph_events'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE social_graph_events SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'social_graph_events'
    ) THEN
        PERFORM add_compression_policy('social_graph_events', INTERVAL '7 days');
    END IF;
END $$;

-- Convert profile_events to hypertable
-- Step 1: Create index on created_at for hypertable conversion
CREATE INDEX IF NOT EXISTS idx_profile_events_created_at ON profile_events(created_at);

-- Step 2: Drop primary key constraint if it exists
ALTER TABLE profile_events DROP CONSTRAINT IF EXISTS profile_events_pkey CASCADE;

-- Step 3: Add created_at to primary key if it doesn't already exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'profile_events_pkey'
    ) THEN
        ALTER TABLE profile_events ADD PRIMARY KEY (id, created_at);
    END IF;
END $$;

-- Step 4: Convert to hypertable
SELECT create_hypertable('profile_events', 'created_at', if_not_exists => TRUE,
                         migrate_data => TRUE, chunk_time_interval => INTERVAL '1 week');

-- Step 5: Add compression policy (compress chunks older than 14 days)
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'profile_events'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE profile_events SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'profile_events'
    ) THEN
        PERFORM add_compression_policy('profile_events', INTERVAL '14 days');
    END IF;
END $$;

-- Convert platform_events to hypertable
-- Step 1: Create index on created_at for hypertable conversion
CREATE INDEX IF NOT EXISTS idx_platform_events_created_at ON platform_events(created_at);

-- Step 2: Drop primary key constraint if it exists
ALTER TABLE platform_events DROP CONSTRAINT IF EXISTS platform_events_pkey CASCADE;

-- Step 3: Add created_at to primary key if it doesn't already exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'platform_events_pkey'
    ) THEN
        ALTER TABLE platform_events ADD PRIMARY KEY (id, created_at);
    END IF;
END $$;

-- Step 4: Convert to hypertable
SELECT create_hypertable('platform_events', 'created_at', if_not_exists => TRUE,
                         migrate_data => TRUE, chunk_time_interval => INTERVAL '1 week');

-- Step 5: Add compression policy (compress chunks older than 14 days)
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'platform_events'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE platform_events SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'platform_events'
    ) THEN
        PERFORM add_compression_policy('platform_events', INTERVAL '14 days');
    END IF;
END $$;

-- ============================================================================
-- 3. CREATE/HANDLE CONTINUOUS AGGREGATES
-- ============================================================================
-- Create a continuous aggregate for daily social graph events (follow/unfollow)
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'social_graph_daily_stats'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW social_graph_daily_stats
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 day', created_at) AS day,
            event_type,
            COUNT(*) AS event_count
        FROM social_graph_events
        GROUP BY day, event_type
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('social_graph_daily_stats',
            start_offset => INTERVAL '1 month',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    ELSE
        -- Update policy if view exists but ensure policy is correct
        BEGIN
            PERFORM remove_continuous_aggregate_policy('social_graph_daily_stats', if_exists => TRUE);
            PERFORM add_continuous_aggregate_policy('social_graph_daily_stats',
                start_offset => INTERVAL '1 month',
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '1 hour');
        EXCEPTION WHEN OTHERS THEN
            -- Policy might already exist, just continue
            RAISE NOTICE 'Could not update policy for social_graph_daily_stats: %', SQLERRM;
        END;
    END IF;
END
$$;

-- Create a continuous aggregate for profile event statistics
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'profile_daily_stats'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW profile_daily_stats
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 day', created_at) AS day,
            event_type,
            COUNT(*) AS event_count
        FROM profile_events
        GROUP BY day, event_type
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('profile_daily_stats',
            start_offset => INTERVAL '1 month',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    ELSE
        -- Update policy if view exists but ensure policy is correct
        BEGIN
            PERFORM remove_continuous_aggregate_policy('profile_daily_stats', if_exists => TRUE);
            PERFORM add_continuous_aggregate_policy('profile_daily_stats',
                start_offset => INTERVAL '1 month',
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '1 hour');
        EXCEPTION WHEN OTHERS THEN
            -- Policy might already exist, just continue
            RAISE NOTICE 'Could not update policy for profile_daily_stats: %', SQLERRM;
        END;
    END IF;
END
$$;

-- Create a continuous aggregate for platform events
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'platform_daily_stats'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW platform_daily_stats
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 day', created_at) AS day,
            event_type,
            COUNT(*) AS event_count
        FROM platform_events
        GROUP BY day, event_type
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('platform_daily_stats',
            start_offset => INTERVAL '1 month',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    ELSE
        -- Update policy if view exists but ensure policy is correct
        BEGIN
            PERFORM remove_continuous_aggregate_policy('platform_daily_stats', if_exists => TRUE);
            PERFORM add_continuous_aggregate_policy('platform_daily_stats',
                start_offset => INTERVAL '1 month',
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '1 hour');
        EXCEPTION WHEN OTHERS THEN
            -- Policy might already exist, just continue
            RAISE NOTICE 'Could not update policy for platform_daily_stats: %', SQLERRM;
        END;
    END IF;
END
$$;

-- Create a tracking table to record when views have been created/refreshed
CREATE TABLE IF NOT EXISTS continuous_aggregate_refresh_status (
    view_name TEXT PRIMARY KEY,
    last_manual_refresh TIMESTAMP DEFAULT NOW(),
    notes TEXT
);

-- Record the views that should exist
INSERT INTO continuous_aggregate_refresh_status (view_name, notes) 
VALUES 
    ('social_graph_daily_stats', 'Verified in migration'),
    ('profile_daily_stats', 'Verified in migration'),
    ('platform_daily_stats', 'Verified in migration'),
    ('checkpoint_daily_stats', 'Verified in migration')
ON CONFLICT (view_name) DO UPDATE 
SET last_manual_refresh = NOW(),
    notes = 'Updated in migration';

-- ============================================================================
-- 4. ADD CHECKPOINT PROCESSING TABLE
-- ============================================================================
-- Create checkpoint processing table to track indexer performance
CREATE TABLE IF NOT EXISTS checkpoint_processing (
    id SERIAL,
    checkpoint_number BIGINT NOT NULL,
    processing_start_time TIMESTAMP NOT NULL DEFAULT NOW(),
    processing_end_time TIMESTAMP,
    events_processed INT DEFAULT 0,
    profiles_created INT DEFAULT 0,
    profiles_updated INT DEFAULT 0,
    follows_created INT DEFAULT 0,
    follows_removed INT DEFAULT 0,
    platform_events INT DEFAULT 0,
    block_events INT DEFAULT 0,
    processing_status TEXT DEFAULT 'in_progress',
    processing_duration_ms INT,
    error_message TEXT,
    -- Include the time column in the primary key for hypertables
    PRIMARY KEY (id, processing_start_time)
);

-- Create index on processing_start_time for hypertable
CREATE INDEX IF NOT EXISTS idx_checkpoint_processing_start_time ON checkpoint_processing(processing_start_time);

-- Safe check for existing hypertable
DO $$
BEGIN
    -- Skip hypertable creation if it's already a hypertable
    -- Use the safer approach that works with TimescaleDB
    PERFORM create_hypertable('checkpoint_processing', 'processing_start_time', 
                               if_not_exists => TRUE);
    
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'checkpoint_processing'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE checkpoint_processing SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'checkpoint_processing'
    ) THEN
        PERFORM add_compression_policy('checkpoint_processing', INTERVAL '30 days');
    END IF;
END $$;

-- Create a continuous aggregate for daily indexer performance stats
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'checkpoint_daily_stats'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW checkpoint_daily_stats
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 day', processing_start_time) AS day,
            COUNT(*) AS checkpoints_processed,
            SUM(events_processed) AS total_events_processed,
            AVG(processing_duration_ms) AS avg_processing_duration_ms,
            MAX(processing_duration_ms) AS max_processing_duration_ms,
            MIN(processing_duration_ms) AS min_processing_duration_ms
        FROM checkpoint_processing
        WHERE processing_status = 'completed' AND processing_duration_ms IS NOT NULL
        GROUP BY day
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('checkpoint_daily_stats',
            start_offset => INTERVAL '1 month',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    ELSE
        -- Update policy if view exists but ensure policy is correct
        BEGIN
            PERFORM remove_continuous_aggregate_policy('checkpoint_daily_stats', if_exists => TRUE);
            PERFORM add_continuous_aggregate_policy('checkpoint_daily_stats',
                start_offset => INTERVAL '1 month',
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '1 hour');
        EXCEPTION WHEN OTHERS THEN
            -- Policy might already exist, just continue
            RAISE NOTICE 'Could not update policy for checkpoint_daily_stats: %', SQLERRM;
        END;
    END IF;
END
$$; 