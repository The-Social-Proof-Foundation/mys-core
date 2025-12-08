-- PROMOTION SYSTEM WITH TIMESCALEDB INTEGRATION
-- This migration adds promotion functionality to the posts system

-- ============================================================================
-- 1. ADD PROMOTION_ID FIELD TO EXISTING POSTS TABLE
-- ============================================================================
-- Add promotion_id field to posts table to link to promotion data
ALTER TABLE posts ADD COLUMN IF NOT EXISTS promotion_id TEXT;

-- Create index for promotion_id lookups
CREATE INDEX IF NOT EXISTS idx_posts_promotion_id ON posts(promotion_id, time);

-- ============================================================================
-- 2. CREATE PROMOTION CORE TABLES
-- ============================================================================
-- Promoted posts table - stores promotion metadata
CREATE TABLE IF NOT EXISTS promoted_posts (
    id SERIAL NOT NULL,
    promotion_id TEXT NOT NULL,
    post_id TEXT NOT NULL,
    owner TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    payment_per_view BIGINT NOT NULL,
    total_budget BIGINT NOT NULL,
    remaining_budget BIGINT NOT NULL DEFAULT 0,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on created_at for all rows
UPDATE promoted_posts SET time = to_timestamp(created_at / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from created_at for new rows
CREATE OR REPLACE FUNCTION update_promoted_post_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_promoted_post_time ON promoted_posts;
CREATE TRIGGER set_promoted_post_time 
BEFORE INSERT ON promoted_posts
FOR EACH ROW
EXECUTE FUNCTION update_promoted_post_time();

-- Convert to hypertable first
SELECT create_hypertable('promoted_posts', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'promoted_posts_pkey'
    ) THEN
        ALTER TABLE promoted_posts ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint for promotion_id that includes time
CREATE UNIQUE INDEX IF NOT EXISTS idx_promoted_posts_promotion_id_time 
ON promoted_posts(promotion_id, time);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_promoted_posts_post_id ON promoted_posts(post_id, time);
CREATE INDEX IF NOT EXISTS idx_promoted_posts_owner ON promoted_posts(owner, time);
CREATE INDEX IF NOT EXISTS idx_promoted_posts_profile_id ON promoted_posts(profile_id, time);
CREATE INDEX IF NOT EXISTS idx_promoted_posts_active ON promoted_posts(active, time);
CREATE INDEX IF NOT EXISTS idx_promoted_posts_transaction_id ON promoted_posts(transaction_id);

-- Enable compression on promoted_posts table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'promoted_posts'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE promoted_posts SET (timescaledb.compress = true);
    END IF;
END $$;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'promoted_posts'
    ) THEN
        PERFORM add_compression_policy('promoted_posts', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 3. CREATE PROMOTION VIEWS TABLE
-- ============================================================================
-- Promotion views table - tracks individual views and payments
CREATE TABLE IF NOT EXISTS promotion_views (
    id SERIAL NOT NULL,
    post_id TEXT NOT NULL,
    promotion_id TEXT NOT NULL,
    viewer TEXT NOT NULL,
    payment_amount BIGINT NOT NULL,
    view_duration BIGINT NOT NULL,
    platform_id TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on timestamp for all rows
UPDATE promotion_views SET time = to_timestamp(timestamp / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from timestamp for new rows
CREATE OR REPLACE FUNCTION update_promotion_view_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_promotion_view_time ON promotion_views;
CREATE TRIGGER set_promotion_view_time 
BEFORE INSERT ON promotion_views
FOR EACH ROW
EXECUTE FUNCTION update_promotion_view_time();

-- Convert to hypertable first
SELECT create_hypertable('promotion_views', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 week');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'promotion_views_pkey'
    ) THEN
        ALTER TABLE promotion_views ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint to prevent duplicate views by same user
CREATE UNIQUE INDEX IF NOT EXISTS idx_promotion_views_post_viewer_time 
ON promotion_views(post_id, viewer, time);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_promotion_views_post_id ON promotion_views(post_id, time);
CREATE INDEX IF NOT EXISTS idx_promotion_views_promotion_id ON promotion_views(promotion_id, time);
CREATE INDEX IF NOT EXISTS idx_promotion_views_viewer ON promotion_views(viewer, time);
CREATE INDEX IF NOT EXISTS idx_promotion_views_platform_id ON promotion_views(platform_id, time);
CREATE INDEX IF NOT EXISTS idx_promotion_views_transaction_id ON promotion_views(transaction_id);

-- Enable compression on promotion_views table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'promotion_views'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE promotion_views SET (timescaledb.compress = true);
    END IF;
END $$;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'promotion_views'
    ) THEN
        PERFORM add_compression_policy('promotion_views', INTERVAL '30 days');
    END IF;
END $$;

-- ============================================================================
-- 4. CREATE PROMOTION STATUS EVENTS TABLE
-- ============================================================================
-- Promotion status events table - tracks status changes, deactivations, withdrawals
CREATE TABLE IF NOT EXISTS promotion_status_events (
    id SERIAL NOT NULL,
    post_id TEXT NOT NULL,
    promotion_id TEXT NOT NULL,
    event_type TEXT NOT NULL, -- 'status_toggled', 'deactivated', 'funds_withdrawn'
    triggered_by TEXT NOT NULL,
    new_status BOOLEAN, -- for status_toggled events
    amount BIGINT, -- for funds_withdrawn events
    timestamp BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on timestamp for all rows
UPDATE promotion_status_events SET time = to_timestamp(timestamp / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from timestamp for new rows
CREATE OR REPLACE FUNCTION update_promotion_status_event_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_promotion_status_event_time ON promotion_status_events;
CREATE TRIGGER set_promotion_status_event_time 
BEFORE INSERT ON promotion_status_events
FOR EACH ROW
EXECUTE FUNCTION update_promotion_status_event_time();

-- Convert to hypertable first
SELECT create_hypertable('promotion_status_events', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'promotion_status_events_pkey'
    ) THEN
        ALTER TABLE promotion_status_events ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add indexes
CREATE INDEX IF NOT EXISTS idx_promotion_status_events_post_id ON promotion_status_events(post_id, time);
CREATE INDEX IF NOT EXISTS idx_promotion_status_events_promotion_id ON promotion_status_events(promotion_id, time);
CREATE INDEX IF NOT EXISTS idx_promotion_status_events_event_type ON promotion_status_events(event_type, time);
CREATE INDEX IF NOT EXISTS idx_promotion_status_events_triggered_by ON promotion_status_events(triggered_by, time);
CREATE INDEX IF NOT EXISTS idx_promotion_status_events_transaction_id ON promotion_status_events(transaction_id);

-- Enable compression on promotion_status_events table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'promotion_status_events'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE promotion_status_events SET (timescaledb.compress = true);
    END IF;
END $$;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'promotion_status_events'
    ) THEN
        PERFORM add_compression_policy('promotion_status_events', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 5. CREATE PROMOTION BUDGET TRACKING TABLE
-- ============================================================================
-- Promotion budget tracking table - tracks budget changes over time
CREATE TABLE IF NOT EXISTS promotion_budget_events (
    id SERIAL NOT NULL,
    promotion_id TEXT NOT NULL,
    post_id TEXT NOT NULL,
    event_type TEXT NOT NULL, -- 'initial_deposit', 'view_payment', 'withdrawal', 'refund'
    amount BIGINT NOT NULL,
    remaining_budget BIGINT NOT NULL,
    timestamp BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on timestamp for all rows
UPDATE promotion_budget_events SET time = to_timestamp(timestamp / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from timestamp for new rows
CREATE OR REPLACE FUNCTION update_promotion_budget_event_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.timestamp / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_promotion_budget_event_time ON promotion_budget_events;
CREATE TRIGGER set_promotion_budget_event_time 
BEFORE INSERT ON promotion_budget_events
FOR EACH ROW
EXECUTE FUNCTION update_promotion_budget_event_time();

-- Convert to hypertable first
SELECT create_hypertable('promotion_budget_events', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'promotion_budget_events_pkey'
    ) THEN
        ALTER TABLE promotion_budget_events ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add indexes
CREATE INDEX IF NOT EXISTS idx_promotion_budget_events_promotion_id ON promotion_budget_events(promotion_id, time);
CREATE INDEX IF NOT EXISTS idx_promotion_budget_events_post_id ON promotion_budget_events(post_id, time);
CREATE INDEX IF NOT EXISTS idx_promotion_budget_events_event_type ON promotion_budget_events(event_type, time);
CREATE INDEX IF NOT EXISTS idx_promotion_budget_events_transaction_id ON promotion_budget_events(transaction_id);

-- Enable compression on promotion_budget_events table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'promotion_budget_events'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE promotion_budget_events SET (timescaledb.compress = true);
    END IF;
END $$;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'promotion_budget_events'
    ) THEN
        PERFORM add_compression_policy('promotion_budget_events', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 6. CREATE CONTINUOUS AGGREGATES FOR PROMOTION ANALYTICS
-- ============================================================================
-- Create a continuous aggregate for promotion views over time
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'promotion_views_hourly'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW promotion_views_hourly
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 hour', time) AS bucket,
            post_id,
            promotion_id,
            platform_id,
            COUNT(*) AS view_count,
            SUM(payment_amount) AS total_payments,
            AVG(view_duration) AS avg_view_duration
        FROM promotion_views
        GROUP BY bucket, post_id, promotion_id, platform_id
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('promotion_views_hourly',
            start_offset => INTERVAL '7 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END
$$;

-- Create a continuous aggregate for promotion spending over time
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'promotion_spending_daily'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW promotion_spending_daily
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 day', time) AS bucket,
            COUNT(DISTINCT promotion_id) AS active_promotions,
            SUM(payment_amount) AS total_spending,
            COUNT(*) AS total_views,
            AVG(payment_amount) AS avg_payment_per_view
        FROM promotion_views
        GROUP BY bucket
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('promotion_spending_daily',
            start_offset => INTERVAL '30 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '6 hours');
    END IF;
END
$$;

-- ============================================================================
-- 7. CREATE PROMOTION ANALYTICS VIEWS
-- ============================================================================
-- Create a view for active promoted posts with current stats
CREATE OR REPLACE VIEW active_promoted_posts AS
SELECT 
    pp.promotion_id,
    pp.post_id,
    pp.owner,
    pp.profile_id,
    pp.payment_per_view,
    pp.total_budget,
    pp.remaining_budget,
    pp.created_at,
    p.content,
    p.media_urls,
    p.post_type,
    COALESCE(pv_stats.view_count, 0) AS total_views,
    COALESCE(pv_stats.total_paid, 0) AS total_paid,
    COALESCE(pv_stats.avg_view_duration, 0) AS avg_view_duration
FROM 
    promoted_posts pp
JOIN 
    posts p ON pp.post_id = p.post_id
LEFT JOIN (
    SELECT 
        promotion_id,
        COUNT(*) AS view_count,
        SUM(payment_amount) AS total_paid,
        AVG(view_duration) AS avg_view_duration
    FROM promotion_views
    GROUP BY promotion_id
) pv_stats ON pp.promotion_id = pv_stats.promotion_id
WHERE 
    pp.active = TRUE AND
    p.deleted_at IS NULL AND
    p.removed_from_platform = FALSE;

-- Create a view for promotion performance analytics
CREATE OR REPLACE VIEW promotion_performance AS
WITH promotion_stats AS (
    SELECT 
        pp.promotion_id,
        pp.post_id,
        pp.owner,
        pp.payment_per_view,
        pp.total_budget,
        pp.remaining_budget,
        pp.created_at,
        COALESCE(pv.view_count, 0) AS total_views,
        COALESCE(pv.total_paid, 0) AS total_paid,
        COALESCE(pv.unique_viewers, 0) AS unique_viewers
    FROM promoted_posts pp
    LEFT JOIN (
        SELECT 
            promotion_id,
            COUNT(*) AS view_count,
            SUM(payment_amount) AS total_paid,
            COUNT(DISTINCT viewer) AS unique_viewers
        FROM promotion_views
        GROUP BY promotion_id
    ) pv ON pp.promotion_id = pv.promotion_id
)
SELECT 
    ps.*,
    CASE 
        WHEN ps.total_budget > 0 THEN (ps.total_paid::FLOAT / ps.total_budget * 100)
        ELSE 0
    END AS budget_utilization_percent,
    CASE 
        WHEN ps.total_views > 0 THEN (ps.total_paid::FLOAT / ps.total_views)
        ELSE 0
    END AS actual_cost_per_view,
    CASE 
        WHEN ps.remaining_budget > 0 AND ps.payment_per_view > 0 
        THEN (ps.remaining_budget / ps.payment_per_view)
        ELSE 0
    END AS estimated_remaining_views
FROM promotion_stats ps;

-- Create a view for top performing promoted posts
CREATE OR REPLACE VIEW top_promoted_posts AS
SELECT 
    pp.promotion_id,
    pp.post_id,
    pp.owner,
    pp.profile_id,
    p.content,
    p.created_at,
    pv_stats.view_count,
    pv_stats.total_paid,
    pv_stats.unique_viewers,
    pv_stats.avg_view_duration,
    (pv_stats.view_count::FLOAT / EXTRACT(EPOCH FROM (NOW() - to_timestamp(pp.created_at / 1000))) * 3600) AS views_per_hour
FROM 
    promoted_posts pp
JOIN 
    posts p ON pp.post_id = p.post_id
LEFT JOIN (
    SELECT 
        promotion_id,
        COUNT(*) AS view_count,
        SUM(payment_amount) AS total_paid,
        COUNT(DISTINCT viewer) AS unique_viewers,
        AVG(view_duration) AS avg_view_duration
    FROM promotion_views
    WHERE time >= NOW() - INTERVAL '7 days'
    GROUP BY promotion_id
) pv_stats ON pp.promotion_id = pv_stats.promotion_id
WHERE 
    p.deleted_at IS NULL AND
    p.removed_from_platform = FALSE AND
    pv_stats.view_count > 0
ORDER BY 
    views_per_hour DESC;

-- Update the continuous aggregate tracking table
INSERT INTO continuous_aggregate_refresh_status (view_name, notes) 
VALUES 
    ('promotion_views_hourly', 'Created in promotion migration'),
    ('promotion_spending_daily', 'Created in promotion migration')
ON CONFLICT (view_name) DO UPDATE 
SET last_manual_refresh = NOW(),
    notes = 'Updated in promotion migration';