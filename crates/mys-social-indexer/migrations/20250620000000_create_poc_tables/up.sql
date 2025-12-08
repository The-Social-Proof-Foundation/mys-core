-- PROOF OF CREATIVITY (POC) SYSTEM WITH TIMESCALEDB INTEGRATION
-- This migration creates the complete PoC system with optimized TimescaleDB features

-- ============================================================================
-- 1. ENSURE TIMESCALEDB EXTENSION IS AVAILABLE
-- ============================================================================
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
    END IF;
END
$$;

-- ============================================================================
-- 2. UPDATE POSTS TABLE WITH POC FIELDS
-- ============================================================================
-- Add PoC-related fields to existing posts table
DO $$
BEGIN
    -- Add poc_badge_id column if it doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'poc_badge_id'
    ) THEN
        ALTER TABLE posts ADD COLUMN poc_badge_id TEXT NULL;
    END IF;
    
    -- Add revenue_redirect_to column if it doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'revenue_redirect_to'
    ) THEN
        ALTER TABLE posts ADD COLUMN revenue_redirect_to TEXT NULL;
    END IF;
    
    -- Add revenue_redirect_percentage column if it doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'revenue_redirect_percentage'
    ) THEN
        ALTER TABLE posts ADD COLUMN revenue_redirect_percentage BIGINT NULL;
    END IF;
END
$$;

-- Create indexes for new PoC fields on posts table
CREATE INDEX IF NOT EXISTS idx_posts_poc_badge_id ON posts(poc_badge_id, time) WHERE poc_badge_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_posts_revenue_redirect_to ON posts(revenue_redirect_to, time) WHERE revenue_redirect_to IS NOT NULL;

-- ============================================================================
-- 3. CREATE POC BADGES TABLE (TIMESCALEDB HYPERTABLE)
-- ============================================================================
CREATE TABLE IF NOT EXISTS poc_badges (
    badge_id TEXT NOT NULL,
    post_id TEXT NOT NULL,
    media_type SMALLINT NOT NULL,
    issued_by TEXT NOT NULL,
    issued_at BIGINT NOT NULL,
    revoked BOOLEAN DEFAULT FALSE,
    revoked_at BIGINT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Set the time column based on issued_at for all rows
UPDATE poc_badges SET time = to_timestamp(issued_at / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from issued_at for new rows
CREATE OR REPLACE FUNCTION update_poc_badge_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.issued_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_poc_badge_time ON poc_badges;
CREATE TRIGGER set_poc_badge_time 
BEFORE INSERT ON poc_badges
FOR EACH ROW
EXECUTE FUNCTION update_poc_badge_time();

-- Convert to TimescaleDB hypertable for time-series optimization
SELECT create_hypertable('poc_badges', 'time', 
    chunk_time_interval => INTERVAL '7 days',
    if_not_exists => TRUE,
    create_default_indexes => FALSE
);

-- Add primary key that includes time (only if it doesn't exist)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conrelid = 'poc_badges'::regclass 
        AND contype = 'p'
    ) THEN
        ALTER TABLE poc_badges ADD PRIMARY KEY (badge_id, time);
    END IF;
END $$;

-- TimescaleDB-optimized indexes
CREATE INDEX IF NOT EXISTS idx_poc_badges_time_post ON poc_badges (time DESC, post_id);
CREATE INDEX IF NOT EXISTS idx_poc_badges_issued_by_time ON poc_badges (issued_by, time DESC);
-- TimescaleDB requires unique indexes to include partitioning column (time)
CREATE UNIQUE INDEX IF NOT EXISTS idx_poc_badges_badge_id ON poc_badges (badge_id, time);

-- Enable compression
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'poc_badges'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE poc_badges SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'poc_badges'
    ) THEN
        PERFORM add_compression_policy('poc_badges', INTERVAL '30 days');
    END IF;
END $$;

-- ============================================================================
-- 4. CREATE POC REVENUE REDIRECTIONS TABLE (TIMESCALEDB HYPERTABLE)
-- ============================================================================
CREATE TABLE IF NOT EXISTS poc_revenue_redirections (
    redirection_id TEXT NOT NULL,
    accused_post_id TEXT NOT NULL,
    original_post_id TEXT NOT NULL,
    redirect_percentage BIGINT NOT NULL,
    similarity_score BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    removed BOOLEAN DEFAULT FALSE,
    removed_at BIGINT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Set the time column based on created_at for all rows
UPDATE poc_revenue_redirections SET time = to_timestamp(created_at / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from created_at for new rows
CREATE OR REPLACE FUNCTION update_poc_redirection_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_poc_redirection_time ON poc_revenue_redirections;
CREATE TRIGGER set_poc_redirection_time 
BEFORE INSERT ON poc_revenue_redirections
FOR EACH ROW
EXECUTE FUNCTION update_poc_redirection_time();

-- Convert to TimescaleDB hypertable
SELECT create_hypertable('poc_revenue_redirections', 'time', 
    chunk_time_interval => INTERVAL '30 days',
    if_not_exists => TRUE,
    create_default_indexes => FALSE
);

-- Add primary key that includes time (only if it doesn't exist)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conrelid = 'poc_revenue_redirections'::regclass 
        AND contype = 'p'
    ) THEN
        ALTER TABLE poc_revenue_redirections ADD PRIMARY KEY (redirection_id, time);
    END IF;
END $$;

-- Optimized indexes for revenue tracking queries
CREATE INDEX IF NOT EXISTS idx_poc_redirections_time_accused ON poc_revenue_redirections (time DESC, accused_post_id);
CREATE INDEX IF NOT EXISTS idx_poc_redirections_original_time ON poc_revenue_redirections (original_post_id, time DESC);
-- TimescaleDB requires unique indexes to include partitioning column (time)
CREATE UNIQUE INDEX IF NOT EXISTS idx_poc_redirections_id ON poc_revenue_redirections (redirection_id, time);

-- Enable compression
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'poc_revenue_redirections'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE poc_revenue_redirections SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'poc_revenue_redirections'
    ) THEN
        PERFORM add_compression_policy('poc_revenue_redirections', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 5. CREATE POC ANALYSIS RESULTS TABLE (TIMESCALEDB HYPERTABLE)
-- ============================================================================
CREATE TABLE IF NOT EXISTS poc_analysis_results (
    post_id TEXT NOT NULL,
    media_type SMALLINT NOT NULL,
    similarity_detected BOOLEAN NOT NULL,
    highest_similarity_score BIGINT NOT NULL,
    oracle_address TEXT NOT NULL,
    original_creator TEXT NULL,
    analysis_timestamp BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Set the time column based on analysis_timestamp for all rows
UPDATE poc_analysis_results SET time = to_timestamp(analysis_timestamp / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from analysis_timestamp for new rows
CREATE OR REPLACE FUNCTION update_poc_analysis_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.analysis_timestamp / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_poc_analysis_time ON poc_analysis_results;
CREATE TRIGGER set_poc_analysis_time 
BEFORE INSERT ON poc_analysis_results
FOR EACH ROW
EXECUTE FUNCTION update_poc_analysis_time();

-- Convert to TimescaleDB hypertable with shorter chunks for recent data
SELECT create_hypertable('poc_analysis_results', 'time', 
    chunk_time_interval => INTERVAL '3 days',
    if_not_exists => TRUE,
    create_default_indexes => FALSE
);

-- Add primary key that includes time (post_id + time for uniqueness)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conrelid = 'poc_analysis_results'::regclass 
        AND contype = 'p'
    ) THEN
        ALTER TABLE poc_analysis_results ADD PRIMARY KEY (post_id, time);
    END IF;
END $$;

-- Indexes for analysis lookup patterns
CREATE INDEX IF NOT EXISTS idx_poc_analysis_time_post ON poc_analysis_results (time DESC, post_id);
CREATE INDEX IF NOT EXISTS idx_poc_analysis_oracle_time ON poc_analysis_results (oracle_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_poc_analysis_creator_time ON poc_analysis_results (original_creator, time DESC) WHERE original_creator IS NOT NULL;

-- Enable compression
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'poc_analysis_results'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE poc_analysis_results SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'poc_analysis_results'
    ) THEN
        PERFORM add_compression_policy('poc_analysis_results', INTERVAL '7 days');
    END IF;
END $$;

-- ============================================================================
-- 6. CREATE POC DISPUTES TABLE (TIMESCALEDB HYPERTABLE)
-- ============================================================================
CREATE TABLE IF NOT EXISTS poc_disputes (
    dispute_id TEXT NOT NULL,
    post_id TEXT NOT NULL,
    disputer TEXT NOT NULL,
    dispute_type SMALLINT NOT NULL,
    evidence TEXT NOT NULL,
    status SMALLINT NOT NULL,
    stake_amount BIGINT NOT NULL,
    voting_start_epoch BIGINT NOT NULL,
    voting_end_epoch BIGINT NOT NULL,
    resolution SMALLINT NULL,
    winning_side SMALLINT NULL,
    total_winning_stake BIGINT NULL,
    total_losing_stake BIGINT NULL,
    submitted_at BIGINT NOT NULL,
    resolved_at BIGINT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Set the time column based on submitted_at for all rows
UPDATE poc_disputes SET time = to_timestamp(submitted_at / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from submitted_at for new rows
CREATE OR REPLACE FUNCTION update_poc_dispute_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.submitted_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_poc_dispute_time ON poc_disputes;
CREATE TRIGGER set_poc_dispute_time 
BEFORE INSERT ON poc_disputes
FOR EACH ROW
EXECUTE FUNCTION update_poc_dispute_time();

-- Convert to TimescaleDB hypertable
SELECT create_hypertable('poc_disputes', 'time', 
    chunk_time_interval => INTERVAL '30 days',
    if_not_exists => TRUE,
    create_default_indexes => FALSE
);

-- Add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conrelid = 'poc_disputes'::regclass 
        AND contype = 'p'
    ) THEN
        ALTER TABLE poc_disputes ADD PRIMARY KEY (dispute_id, time);
    END IF;
END $$;

-- Optimized indexes for dispute tracking
CREATE INDEX IF NOT EXISTS idx_poc_disputes_time_status ON poc_disputes (time DESC, status);
CREATE INDEX IF NOT EXISTS idx_poc_disputes_post_time ON poc_disputes (post_id, time DESC);
-- TimescaleDB requires unique indexes to include partitioning column (time)
CREATE UNIQUE INDEX IF NOT EXISTS idx_poc_disputes_id ON poc_disputes (dispute_id, time);

-- Enable compression
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'poc_disputes'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE poc_disputes SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'poc_disputes'
    ) THEN
        PERFORM add_compression_policy('poc_disputes', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 7. CREATE POC DISPUTE VOTES TABLE (TIMESCALEDB HYPERTABLE)
-- ============================================================================
CREATE TABLE IF NOT EXISTS poc_dispute_votes (
    dispute_id TEXT NOT NULL,
    voter TEXT NOT NULL,
    vote_choice SMALLINT NOT NULL,
    stake_amount BIGINT NOT NULL,
    voted_at BIGINT NOT NULL,
    reward_claimed BOOLEAN DEFAULT FALSE,
    reward_amount BIGINT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Set the time column based on voted_at for all rows
UPDATE poc_dispute_votes SET time = to_timestamp(voted_at / 1000) WHERE time = NOW();

-- Create a trigger to automatically update time from voted_at for new rows
CREATE OR REPLACE FUNCTION update_poc_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.voted_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_poc_vote_time ON poc_dispute_votes;
CREATE TRIGGER set_poc_vote_time 
BEFORE INSERT ON poc_dispute_votes
FOR EACH ROW
EXECUTE FUNCTION update_poc_vote_time();

-- Convert to TimescaleDB hypertable
SELECT create_hypertable('poc_dispute_votes', 'time', 
    chunk_time_interval => INTERVAL '14 days',
    if_not_exists => TRUE,
    create_default_indexes => FALSE
);

-- Add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conrelid = 'poc_dispute_votes'::regclass 
        AND contype = 'p'
    ) THEN
        ALTER TABLE poc_dispute_votes ADD PRIMARY KEY (dispute_id, voter, time);
    END IF;
END $$;

-- Composite unique constraint and indexes
-- TimescaleDB requires unique indexes to include partitioning column (time)
CREATE UNIQUE INDEX IF NOT EXISTS idx_poc_votes_dispute_voter ON poc_dispute_votes (dispute_id, voter, time);
CREATE INDEX IF NOT EXISTS idx_poc_votes_time_voter ON poc_dispute_votes (time DESC, voter);
CREATE INDEX IF NOT EXISTS idx_poc_votes_dispute_time ON poc_dispute_votes (dispute_id, time DESC);

-- Enable compression
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'poc_dispute_votes'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE poc_dispute_votes SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'poc_dispute_votes'
    ) THEN
        PERFORM add_compression_policy('poc_dispute_votes', INTERVAL '30 days');
    END IF;
END $$;

-- ============================================================================
-- 8. CREATE POC CONFIGURATION TABLE (REGULAR TABLE)
-- ============================================================================
-- Configuration table doesn't need time partitioning (infrequent updates)
CREATE TABLE IF NOT EXISTS poc_configuration (
    id SERIAL PRIMARY KEY,
    image_threshold BIGINT NOT NULL,
    video_threshold BIGINT NOT NULL,
    audio_threshold BIGINT NOT NULL,
    revenue_redirect_percentage BIGINT NOT NULL,
    dispute_cost BIGINT NOT NULL,
    dispute_protocol_fee BIGINT NOT NULL,
    min_vote_stake BIGINT NOT NULL,
    max_vote_stake BIGINT NOT NULL,
    voting_duration_epochs BIGINT NOT NULL,
    updated_by TEXT NOT NULL,
    updated_at BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Simple index for latest configuration lookup
CREATE INDEX IF NOT EXISTS idx_poc_config_time ON poc_configuration (time DESC);

-- ============================================================================
-- 9. CREATE POC DAILY STATISTICS (TIMESCALEDB CONTINUOUS AGGREGATE)
-- ============================================================================
-- Use TimescaleDB continuous aggregates for efficient daily statistics
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'poc_daily_stats'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW poc_daily_stats
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 day', time) AS day,
            COUNT(*) FILTER (WHERE NOT revoked) AS badges_issued,
            0 AS redirections_created, -- Will be updated by separate query
            0 AS disputes_submitted, -- Will be updated by separate query
            0 AS votes_cast -- Will be updated by separate query
        FROM poc_badges
        GROUP BY time_bucket('1 day', time)
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('poc_daily_stats',
            start_offset => INTERVAL '3 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END $$;

-- ============================================================================
-- 10. CREATE ADDITIONAL CONTINUOUS AGGREGATES FOR ANALYTICS
-- ============================================================================

-- Create hourly PoC stats for real-time monitoring (badges only)
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'poc_hourly_stats'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW poc_hourly_stats
        WITH (timescaledb.continuous) AS
        SELECT 
            time_bucket('1 hour', time) AS hour,
            COUNT(*) FILTER (WHERE NOT revoked) AS badges_issued_hourly,
            COUNT(*) AS total_badges
        FROM poc_badges
        GROUP BY time_bucket('1 hour', time)
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('poc_hourly_stats',
            start_offset => INTERVAL '1 day',
            end_offset => INTERVAL '1 hour', 
            schedule_interval => INTERVAL '15 minutes');
    END IF;
END $$;

-- ============================================================================
-- 11. ADD REFERENTIAL INTEGRITY TRIGGERS
-- ============================================================================

-- Trigger to validate post references in PoC tables
CREATE OR REPLACE FUNCTION validate_poc_post_reference()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM posts WHERE post_id = NEW.post_id) THEN
        RAISE EXCEPTION 'Referenced post_id does not exist: %', NEW.post_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply to PoC tables that reference posts
DROP TRIGGER IF EXISTS check_poc_badge_post_reference ON poc_badges;
CREATE TRIGGER check_poc_badge_post_reference
BEFORE INSERT OR UPDATE ON poc_badges
FOR EACH ROW
EXECUTE FUNCTION validate_poc_post_reference();

DROP TRIGGER IF EXISTS check_poc_redirection_accused_post_reference ON poc_revenue_redirections;
CREATE TRIGGER check_poc_redirection_accused_post_reference
BEFORE INSERT OR UPDATE ON poc_revenue_redirections
FOR EACH ROW
EXECUTE FUNCTION validate_poc_post_reference();

DROP TRIGGER IF EXISTS check_poc_analysis_post_reference ON poc_analysis_results;
CREATE TRIGGER check_poc_analysis_post_reference
BEFORE INSERT OR UPDATE ON poc_analysis_results
FOR EACH ROW
EXECUTE FUNCTION validate_poc_post_reference();

DROP TRIGGER IF EXISTS check_poc_dispute_post_reference ON poc_disputes;
CREATE TRIGGER check_poc_dispute_post_reference
BEFORE INSERT OR UPDATE ON poc_disputes
FOR EACH ROW
EXECUTE FUNCTION validate_poc_post_reference();

-- Trigger to validate original post references in redirections
CREATE OR REPLACE FUNCTION validate_poc_original_post_reference()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM posts WHERE post_id = NEW.original_post_id) THEN
        RAISE EXCEPTION 'Referenced original_post_id does not exist: %', NEW.original_post_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS check_poc_redirection_original_post_reference ON poc_revenue_redirections;
CREATE TRIGGER check_poc_redirection_original_post_reference
BEFORE INSERT OR UPDATE ON poc_revenue_redirections
FOR EACH ROW
EXECUTE FUNCTION validate_poc_original_post_reference();

-- Trigger to validate dispute references in votes
CREATE OR REPLACE FUNCTION validate_poc_dispute_reference()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM poc_disputes WHERE dispute_id = NEW.dispute_id) THEN
        RAISE EXCEPTION 'Referenced dispute_id does not exist: %', NEW.dispute_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS check_poc_vote_dispute_reference ON poc_dispute_votes;
CREATE TRIGGER check_poc_vote_dispute_reference
BEFORE INSERT OR UPDATE ON poc_dispute_votes
FOR EACH ROW
EXECUTE FUNCTION validate_poc_dispute_reference();

-- ============================================================================
-- 12. CREATE RETENTION POLICIES FOR DATA CLEANUP
-- ============================================================================

-- Add retention policies for data cleanup as specified in plan
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_retention' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'poc_analysis_results'
    ) THEN
        PERFORM add_retention_policy('poc_analysis_results', INTERVAL '1 year');
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_retention' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'poc_dispute_votes'
    ) THEN
        PERFORM add_retention_policy('poc_dispute_votes', INTERVAL '2 years');
    END IF;
END $$;

-- ============================================================================
-- 13. COMMENTS AND DOCUMENTATION
-- ============================================================================

-- Add comments to tables for documentation
COMMENT ON TABLE poc_badges IS 'PoC badges issued for original content verification';
COMMENT ON TABLE poc_revenue_redirections IS 'Revenue redirection records for derivative content';
COMMENT ON TABLE poc_analysis_results IS 'Oracle analysis results for content similarity detection';
COMMENT ON TABLE poc_disputes IS 'Community disputes challenging PoC decisions';
COMMENT ON TABLE poc_dispute_votes IS 'Community votes on PoC disputes';
COMMENT ON TABLE poc_configuration IS 'System-wide PoC configuration parameters';

COMMENT ON COLUMN poc_badges.media_type IS '1=image, 2=video, 3=audio';
COMMENT ON COLUMN poc_disputes.dispute_type IS '1=challenge badge, 2=challenge redirection';
COMMENT ON COLUMN poc_disputes.status IS '1=voting, 2=resolved_upheld, 3=resolved_overturned';
COMMENT ON COLUMN poc_dispute_votes.vote_choice IS '1=uphold, 2=overturn'; 