-- MY IP SYSTEM WITH TIMESCALEDB INTEGRATION
-- Production-ready implementation with TimescaleDB features for high-performance analytics

-- ============================================================================
-- 1. ENABLE TIMESCALEDB EXTENSION
-- ============================================================================
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
    END IF;
END
$$;

-- ============================================================================
-- 2. CREATE CORE TABLES
-- ============================================================================
-- Primary table for intellectual property licenses
CREATE TABLE IF NOT EXISTS my_ip (
    id SERIAL NOT NULL,
    license_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    creator TEXT NOT NULL,
    creation_time BIGINT NOT NULL,
    license_type SMALLINT NOT NULL,  -- 0: Creative Commons, 1: Token Bound, 2: Custom
    permission_flags BIGINT NOT NULL, -- Bitfield of permissions
    license_state SMALLINT NOT NULL, -- 0: Active, 1: Expired, 2: Revoked
    proof_of_creativity_id TEXT,
    custom_license_uri TEXT,
    revenue_recipient TEXT,
    transferable BOOLEAN NOT NULL DEFAULT false,
    expires_at BIGINT,
    version INTEGER NOT NULL DEFAULT 1,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_my_ip PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('my_ip', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on my_ip table
ALTER TABLE my_ip SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'license_id,creator',
    timescaledb.compress_orderby = 'time DESC'
);

-- Create a unique index on license_id and time
CREATE INDEX IF NOT EXISTS idx_unique_license_id_time ON my_ip(license_id, time);

-- Create a non-unique index on just license_id for efficient lookups
CREATE INDEX IF NOT EXISTS idx_my_ip_license_id ON my_ip(license_id);

-- Permissions definitions table (for reference)
CREATE TABLE IF NOT EXISTS my_ip_permissions (
    id SERIAL PRIMARY KEY,
    permission_name TEXT NOT NULL,
    bit_position INTEGER NOT NULL,
    description TEXT NOT NULL
);

-- Insert permission definitions
INSERT INTO my_ip_permissions (permission_name, bit_position, description) 
SELECT permission_name, bit_position, description FROM (VALUES
('COMMERCIAL_USE', 0, 'Allows commercial use of the content'),
('DERIVATIVES_ALLOWED', 1, 'Allows creating derivative works'),
('PUBLIC_LICENSE', 2, 'License is public and can be viewed by anyone'),
('AUTHORITY_REQUIRED', 3, 'Authority is required for certain operations'),
('SHARE_ALIKE', 4, 'Derivatives must be shared under same license'),
('REQUIRE_ATTRIBUTION', 5, 'Attribution to original creator is required'),
('REVENUE_REDIRECT', 6, 'Revenue from content is redirected'),
('ALLOW_COMMENTS', 10, 'Allows commenting on the content'),
('ALLOW_REACTIONS', 11, 'Allows reactions/likes on the content'),
('ALLOW_REPOSTS', 12, 'Allows reposting the content'),
('ALLOW_QUOTES', 13, 'Allows quoting the content'),
('ALLOW_TIPS', 14, 'Allows tipping the content')
) AS v(permission_name, bit_position, description)
WHERE NOT EXISTS (SELECT 1 FROM my_ip_permissions);

-- License events table with time dimension
CREATE TABLE IF NOT EXISTS my_ip_events (
    id SERIAL NOT NULL,
    event_type TEXT NOT NULL,
    license_id TEXT NOT NULL,
    event_data JSONB NOT NULL,
    created_by TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_my_ip_events PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable for events
SELECT create_hypertable('my_ip_events', 'time', if_not_exists => TRUE, chunk_time_interval => INTERVAL '1 day');

-- Enable compression on my_ip_events table
ALTER TABLE my_ip_events SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'license_id,event_type',
    timescaledb.compress_orderby = 'time DESC'
);

-- License grants/transfers table with time dimension
CREATE TABLE IF NOT EXISTS my_ip_grants (
    id SERIAL NOT NULL,
    license_id TEXT NOT NULL,
    grantor TEXT NOT NULL,
    grantee TEXT NOT NULL,
    grant_type TEXT NOT NULL, -- 'TRANSFER', 'LICENSE'
    payment_amount BIGINT NOT NULL DEFAULT 0,
    payment_token TEXT,
    grant_time BIGINT NOT NULL,
    expiration_time BIGINT,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_my_ip_grants PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable for grants
SELECT create_hypertable('my_ip_grants', 'time', if_not_exists => TRUE, chunk_time_interval => INTERVAL '1 day');

-- Enable compression on my_ip_grants table
ALTER TABLE my_ip_grants SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'license_id,grantor,grantee',
    timescaledb.compress_orderby = 'time DESC'
);

-- Revenue distribution table with time dimension
CREATE TABLE IF NOT EXISTS my_ip_revenue (
    id SERIAL NOT NULL,
    license_id TEXT NOT NULL,
    post_id TEXT,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    revenue_type TEXT NOT NULL, -- 'TIP', 'LICENSE_FEE', etc.
    revenue_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_my_ip_revenue PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable for revenue
SELECT create_hypertable('my_ip_revenue', 'time', if_not_exists => TRUE, chunk_time_interval => INTERVAL '1 day');

-- Enable compression on my_ip_revenue table
ALTER TABLE my_ip_revenue SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'license_id,from_address,to_address,revenue_type',
    timescaledb.compress_orderby = 'time DESC'
);

-- Add relationship between posts and my_ip if columns don't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'my_ip_id'
    ) THEN
        ALTER TABLE posts ADD COLUMN my_ip_id TEXT;
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'revenue_recipient'
    ) THEN
        ALTER TABLE posts ADD COLUMN revenue_recipient TEXT;
    END IF;
END
$$;

-- ============================================================================
-- 3. CREATE INDEXES
-- ============================================================================
CREATE INDEX IF NOT EXISTS idx_my_ip_creator ON my_ip(creator);
CREATE INDEX IF NOT EXISTS idx_my_ip_license_type ON my_ip(license_type);
CREATE INDEX IF NOT EXISTS idx_my_ip_license_state ON my_ip(license_state);
CREATE INDEX IF NOT EXISTS idx_my_ip_creation_time ON my_ip(creation_time);
CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_recipient ON my_ip(revenue_recipient);

CREATE INDEX IF NOT EXISTS idx_my_ip_events_license_id ON my_ip_events(license_id);
CREATE INDEX IF NOT EXISTS idx_my_ip_events_event_type ON my_ip_events(event_type);
CREATE INDEX IF NOT EXISTS idx_my_ip_events_created_by ON my_ip_events(created_by);
CREATE INDEX IF NOT EXISTS idx_my_ip_events_created_at ON my_ip_events(created_at);

CREATE INDEX IF NOT EXISTS idx_my_ip_grants_license_id ON my_ip_grants(license_id);
CREATE INDEX IF NOT EXISTS idx_my_ip_grants_grantor ON my_ip_grants(grantor);
CREATE INDEX IF NOT EXISTS idx_my_ip_grants_grantee ON my_ip_grants(grantee);
CREATE INDEX IF NOT EXISTS idx_my_ip_grants_grant_time ON my_ip_grants(grant_time);

CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_license_id ON my_ip_revenue(license_id);
CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_post_id ON my_ip_revenue(post_id);
CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_from_address ON my_ip_revenue(from_address);
CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_to_address ON my_ip_revenue(to_address);
CREATE INDEX IF NOT EXISTS idx_my_ip_revenue_time ON my_ip_revenue(revenue_time);

CREATE INDEX IF NOT EXISTS idx_posts_my_ip_id ON posts(my_ip_id);
CREATE INDEX IF NOT EXISTS idx_posts_revenue_recipient ON posts(revenue_recipient);

-- ============================================================================
-- 4. CREATE VIEWS AND MATERIALIZED VIEWS
-- ============================================================================
-- Create views and recreate them if they already exist
CREATE OR REPLACE VIEW active_licenses AS
SELECT 
    l.*,
    (l.permission_flags & (1<<0)) > 0 AS commercial_use,
    (l.permission_flags & (1<<1)) > 0 AS derivatives_allowed,
    (l.permission_flags & (1<<2)) > 0 AS public_license,
    (l.permission_flags & (1<<3)) > 0 AS authority_required,
    (l.permission_flags & (1<<4)) > 0 AS share_alike,
    (l.permission_flags & (1<<5)) > 0 AS require_attribution,
    (l.permission_flags & (1<<6)) > 0 AS revenue_redirect,
    (l.permission_flags & (1<<10)) > 0 AS allow_comments,
    (l.permission_flags & (1<<11)) > 0 AS allow_reactions,
    (l.permission_flags & (1<<12)) > 0 AS allow_reposts,
    (l.permission_flags & (1<<13)) > 0 AS allow_quotes,
    (l.permission_flags & (1<<14)) > 0 AS allow_tips
FROM 
    my_ip l
WHERE 
    l.license_state = 0 -- Active licenses only
    AND (l.expires_at IS NULL OR l.expires_at > EXTRACT(EPOCH FROM NOW()));

CREATE OR REPLACE VIEW popular_licenses AS
SELECT 
    l.license_id,
    l.name,
    l.creator,
    l.license_type,
    COUNT(p.id) AS post_count,
    SUM(p.reaction_count) AS total_reactions,
    SUM(p.comment_count) AS total_comments,
    SUM(p.repost_count) AS total_reposts,
    SUM(p.tips_received) AS total_tips
FROM 
    my_ip l
JOIN 
    posts p ON p.my_ip_id = l.license_id
WHERE 
    p.deleted_at IS NULL
    AND p.removed_from_platform = false
GROUP BY 
    l.license_id, l.name, l.creator, l.license_type
ORDER BY 
    post_count DESC, total_reactions DESC;

-- Materialized view for daily revenue by license
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_matviews WHERE matviewname = 'daily_license_revenue'
    ) THEN
        CREATE MATERIALIZED VIEW daily_license_revenue AS
SELECT
    time_bucket('1 day', time) AS bucket,
    license_id,
    revenue_type,
    SUM(amount) AS total_amount,
    COUNT(*) AS transaction_count
FROM 
    my_ip_revenue
GROUP BY 
    bucket, license_id, revenue_type;

        -- Create index on the materialized view
        CREATE INDEX IF NOT EXISTS idx_daily_license_revenue_bucket ON daily_license_revenue(bucket);
        CREATE INDEX IF NOT EXISTS idx_daily_license_revenue_license_id ON daily_license_revenue(license_id);
    END IF;
END $$;

-- Materialized view for weekly revenue by creator
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_matviews WHERE matviewname = 'weekly_creator_revenue'
    ) THEN
        CREATE MATERIALIZED VIEW weekly_creator_revenue AS
SELECT
    time_bucket('1 week', r.time) AS bucket,
    l.creator,
    SUM(r.amount) AS total_amount,
    COUNT(*) AS transaction_count
FROM 
    my_ip_revenue r
JOIN 
    my_ip l ON r.license_id = l.license_id
GROUP BY 
    bucket, l.creator;

        -- Create index on the materialized view
        CREATE INDEX IF NOT EXISTS idx_weekly_creator_revenue_bucket ON weekly_creator_revenue(bucket);
        CREATE INDEX IF NOT EXISTS idx_weekly_creator_revenue_creator ON weekly_creator_revenue(creator);
    END IF;
END $$;

-- Create or replace the refresh function
CREATE OR REPLACE FUNCTION refresh_license_materialized_views()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW daily_license_revenue;
    REFRESH MATERIALIZED VIEW weekly_creator_revenue;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- 5. SET UP AUTOMATIC COMPRESSION POLICIES AND JOB SCHEDULING
-- ============================================================================

-- Add compression policies to compress chunks after 7 days
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'my_ip'
    ) THEN
        PERFORM add_compression_policy('my_ip', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'my_ip_events'
    ) THEN
        PERFORM add_compression_policy('my_ip_events', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'my_ip_grants'
    ) THEN
        PERFORM add_compression_policy('my_ip_grants', INTERVAL '7 days');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'my_ip_revenue'
    ) THEN
        PERFORM add_compression_policy('my_ip_revenue', INTERVAL '7 days');
    END IF;
END $$;

-- Create a scheduled job to refresh materialized views daily
DO $$
DECLARE
    job_exists BOOLEAN;
BEGIN
    -- Check if job already exists
    SELECT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs 
        WHERE proc_name = 'refresh_license_materialized_views'
    ) INTO job_exists;
    
    IF NOT job_exists THEN
        -- Create a job that runs at midnight every day
        PERFORM add_job(
            'refresh_license_materialized_views',
            '24 hours',
            initial_start => now()
        );
    END IF;
END $$; 