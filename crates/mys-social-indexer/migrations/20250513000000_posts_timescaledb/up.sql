-- POST SYSTEM WITH TIMESCALEDB INTEGRATION
-- This migration creates the complete posts system with optimized TimescaleDB features

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
-- Posts table - core content storage
CREATE TABLE IF NOT EXISTS posts (
    id TEXT NOT NULL,
    post_id TEXT NOT NULL,
    owner TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    content TEXT NOT NULL,
    media_urls JSONB,
    mentions JSONB,
    metadata_json JSONB,
    post_type TEXT NOT NULL,
    parent_post_id TEXT,
    created_at BIGINT NOT NULL,
    updated_at BIGINT,
    deleted_at BIGINT,
    reaction_count BIGINT DEFAULT 0,
    comment_count BIGINT DEFAULT 0,
    repost_count BIGINT DEFAULT 0,
    tips_received BIGINT DEFAULT 0,
    removed_from_platform BOOLEAN DEFAULT FALSE,
    removed_by TEXT,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Set the time column based on created_at for all rows
UPDATE posts SET time = to_timestamp(created_at) WHERE time = NOW();

-- Create a trigger to automatically update time from created_at for new rows
CREATE OR REPLACE FUNCTION update_post_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_post_time ON posts;
CREATE TRIGGER set_post_time 
BEFORE INSERT ON posts
FOR EACH ROW
EXECUTE FUNCTION update_post_time();

-- Convert to hypertable first (before adding constraints)
SELECT create_hypertable('posts', 'time', if_not_exists => TRUE, 
                         create_default_indexes => FALSE,
                         chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'posts_pkey'
    ) THEN
        ALTER TABLE posts ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Make post_id unique but include time column to satisfy TimescaleDB partitioning
CREATE UNIQUE INDEX IF NOT EXISTS idx_posts_post_id_time ON posts(post_id, time);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_posts_owner ON posts(owner, time);
CREATE INDEX IF NOT EXISTS idx_posts_profile_id ON posts(profile_id, time);
CREATE INDEX IF NOT EXISTS idx_posts_post_type ON posts(post_type, time);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);
CREATE INDEX IF NOT EXISTS idx_posts_transaction_id ON posts(transaction_id);
CREATE INDEX IF NOT EXISTS idx_posts_deleted_at ON posts(deleted_at);

-- Enable compression on posts table
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'posts'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE posts SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'posts'
    ) THEN
        PERFORM add_compression_policy('posts', INTERVAL '90 days');
    END IF;
END $$;

-- Comments table
CREATE TABLE IF NOT EXISTS comments (
    id TEXT NOT NULL,
    comment_id TEXT NOT NULL,
    post_id TEXT NOT NULL,
    parent_comment_id TEXT,
    owner TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    content TEXT NOT NULL,
    media_urls JSONB,
    mentions JSONB,
    metadata_json JSONB,
    created_at BIGINT NOT NULL,
    updated_at BIGINT,
    deleted_at BIGINT,
    reaction_count BIGINT DEFAULT 0,
    comment_count BIGINT DEFAULT 0,
    repost_count BIGINT DEFAULT 0,
    tips_received BIGINT DEFAULT 0,
    removed_from_platform BOOLEAN DEFAULT FALSE,
    removed_by TEXT,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Set the time column based on created_at for all rows
UPDATE comments SET time = to_timestamp(created_at) WHERE time = NOW();

-- Create a trigger to automatically update time from created_at for new rows
CREATE OR REPLACE FUNCTION update_comment_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_comment_time ON comments;
CREATE TRIGGER set_comment_time 
BEFORE INSERT ON comments
FOR EACH ROW
EXECUTE FUNCTION update_comment_time();

-- Convert to hypertable first
SELECT create_hypertable('comments', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'comments_pkey'
    ) THEN
        ALTER TABLE comments ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Make comment_id unique but include time column to satisfy TimescaleDB partitioning
CREATE UNIQUE INDEX IF NOT EXISTS idx_comments_comment_id_time ON comments(comment_id, time);

-- Remove the foreign key reference since hypertables can't have foreign keys to other hypertables
-- Instead, create a trigger-based integrity check
CREATE OR REPLACE FUNCTION validate_post_reference()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM posts WHERE post_id = NEW.post_id) THEN
        RAISE EXCEPTION 'Referenced post_id does not exist: %', NEW.post_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS check_post_reference ON comments;
CREATE TRIGGER check_post_reference
BEFORE INSERT OR UPDATE ON comments
FOR EACH ROW
EXECUTE FUNCTION validate_post_reference();

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_comments_post_id ON comments(post_id, time);
CREATE INDEX IF NOT EXISTS idx_comments_parent_comment_id ON comments(parent_comment_id, time);
CREATE INDEX IF NOT EXISTS idx_comments_owner ON comments(owner, time);
CREATE INDEX IF NOT EXISTS idx_comments_profile_id ON comments(profile_id, time);
CREATE INDEX IF NOT EXISTS idx_comments_created_at ON comments(created_at);
CREATE INDEX IF NOT EXISTS idx_comments_transaction_id ON comments(transaction_id);
CREATE INDEX IF NOT EXISTS idx_comments_deleted_at ON comments(deleted_at);

-- Enable compression on comments table
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'comments'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE comments SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'comments'
    ) THEN
        PERFORM add_compression_policy('comments', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 3. CREATE INTERACTION TABLES AS HYPERTABLES
-- ============================================================================
-- Reactions table - convert to TimescaleDB hypertable for time-series data
CREATE TABLE IF NOT EXISTS reactions (
    id SERIAL NOT NULL,
    object_id TEXT NOT NULL,
    user_address TEXT NOT NULL,
    reaction_text TEXT NOT NULL,
    is_post BOOLEAN NOT NULL,
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on created_at for all rows
UPDATE reactions SET time = to_timestamp(created_at) WHERE time = NOW();

-- Create a trigger to automatically update time from created_at for new rows
CREATE OR REPLACE FUNCTION update_reaction_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_reaction_time ON reactions;
CREATE TRIGGER set_reaction_time 
BEFORE INSERT ON reactions
FOR EACH ROW
EXECUTE FUNCTION update_reaction_time();

-- Convert to hypertable first
SELECT create_hypertable('reactions', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 week');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'reactions_pkey'
    ) THEN
        ALTER TABLE reactions ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint that includes time
CREATE UNIQUE INDEX IF NOT EXISTS idx_reactions_object_user_time 
ON reactions(object_id, user_address, time);

-- Add regular indexes
CREATE INDEX IF NOT EXISTS idx_reactions_object_id ON reactions(object_id, time);
CREATE INDEX IF NOT EXISTS idx_reactions_user_address ON reactions(user_address, time);
CREATE INDEX IF NOT EXISTS idx_reactions_reaction_text ON reactions(reaction_text);
CREATE INDEX IF NOT EXISTS idx_reactions_transaction_id ON reactions(transaction_id);

-- Enable compression policy for reactions
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'reactions'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE reactions SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'reactions'
    ) THEN
        PERFORM add_compression_policy('reactions', INTERVAL '30 days');
    END IF;
END $$;

-- Reaction counts table (for faster lookups)
CREATE TABLE IF NOT EXISTS reaction_counts (
    id SERIAL PRIMARY KEY,
    object_id TEXT NOT NULL,
    reaction_text TEXT NOT NULL,
    count BIGINT NOT NULL DEFAULT 0
);

-- Create unique constraint
CREATE UNIQUE INDEX IF NOT EXISTS idx_reaction_counts_object_reaction 
ON reaction_counts(object_id, reaction_text);

-- Regular indexes
CREATE INDEX IF NOT EXISTS idx_reaction_counts_object_id ON reaction_counts(object_id);
CREATE INDEX IF NOT EXISTS idx_reaction_counts_reaction_text ON reaction_counts(reaction_text);

-- Reposts table - convert to TimescaleDB hypertable
CREATE TABLE IF NOT EXISTS reposts (
    id TEXT NOT NULL,
    repost_id TEXT NOT NULL,
    original_id TEXT NOT NULL,
    original_post_id TEXT NOT NULL,
    is_original_post BOOLEAN NOT NULL,
    owner TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on created_at for all rows
UPDATE reposts SET time = to_timestamp(created_at) WHERE time = NOW();

-- Create a trigger to automatically update time from created_at for new rows
CREATE OR REPLACE FUNCTION update_repost_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_repost_time ON reposts;
CREATE TRIGGER set_repost_time 
BEFORE INSERT ON reposts
FOR EACH ROW
EXECUTE FUNCTION update_repost_time();

-- Convert to hypertable first
SELECT create_hypertable('reposts', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 week');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'reposts_pkey'
    ) THEN
        ALTER TABLE reposts ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint for repost_id that includes time
CREATE UNIQUE INDEX IF NOT EXISTS idx_reposts_repost_id_time 
ON reposts(repost_id, time);

-- Add regular indexes
CREATE INDEX IF NOT EXISTS idx_reposts_original_id ON reposts(original_id, time);
CREATE INDEX IF NOT EXISTS idx_reposts_original_post_id ON reposts(original_post_id, time);
CREATE INDEX IF NOT EXISTS idx_reposts_owner ON reposts(owner, time);
CREATE INDEX IF NOT EXISTS idx_reposts_profile_id ON reposts(profile_id, time);
CREATE INDEX IF NOT EXISTS idx_reposts_transaction_id ON reposts(transaction_id);

-- Enable compression policy for reposts
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'reposts'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE reposts SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'reposts'
    ) THEN
        PERFORM add_compression_policy('reposts', INTERVAL '30 days');
    END IF;
END $$;

-- Tips table - convert to TimescaleDB hypertable
CREATE TABLE IF NOT EXISTS tips (
    id SERIAL NOT NULL,
    tipper TEXT NOT NULL,
    recipient TEXT NOT NULL,
    object_id TEXT NOT NULL,
    amount BIGINT NOT NULL,
    is_post BOOLEAN NOT NULL,
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on created_at for all rows
UPDATE tips SET time = to_timestamp(created_at) WHERE time = NOW();

-- Create a trigger to automatically update time from created_at for new rows
CREATE OR REPLACE FUNCTION update_tip_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_tip_time ON tips;
CREATE TRIGGER set_tip_time 
BEFORE INSERT ON tips
FOR EACH ROW
EXECUTE FUNCTION update_tip_time();

-- Convert to hypertable first
SELECT create_hypertable('tips', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 week');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'tips_pkey'
    ) THEN
        ALTER TABLE tips ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add regular indexes
CREATE INDEX IF NOT EXISTS idx_tips_tipper ON tips(tipper, time);
CREATE INDEX IF NOT EXISTS idx_tips_recipient ON tips(recipient, time);
CREATE INDEX IF NOT EXISTS idx_tips_object_id ON tips(object_id, time);
CREATE INDEX IF NOT EXISTS idx_tips_transaction_id ON tips(transaction_id);

-- Enable compression policy for tips
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'tips'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE tips SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'tips'
    ) THEN
        PERFORM add_compression_policy('tips', INTERVAL '30 days');
    END IF;
END $$;

-- ============================================================================
-- 4. CREATE MODERATION AND MANAGEMENT TABLES
-- ============================================================================
-- Reports table
CREATE TABLE IF NOT EXISTS posts_reports (
    id SERIAL NOT NULL,
    object_id TEXT NOT NULL,
    is_comment BOOLEAN NOT NULL,
    reporter TEXT NOT NULL,
    reason_code SMALLINT NOT NULL,
    description TEXT NOT NULL,
    reported_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on reported_at for all rows
UPDATE posts_reports SET time = to_timestamp(reported_at) WHERE time = NOW();

-- Create a trigger to automatically update time from reported_at for new rows
CREATE OR REPLACE FUNCTION update_report_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.reported_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_report_time ON posts_reports;
CREATE TRIGGER set_report_time 
BEFORE INSERT ON posts_reports
FOR EACH ROW
EXECUTE FUNCTION update_report_time();

-- Convert to hypertable first
SELECT create_hypertable('posts_reports', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'posts_reports_pkey'
    ) THEN
        ALTER TABLE posts_reports ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add regular indexes
CREATE INDEX IF NOT EXISTS idx_reports_object_id ON posts_reports(object_id, time);
CREATE INDEX IF NOT EXISTS idx_reports_reporter ON posts_reports(reporter, time);
CREATE INDEX IF NOT EXISTS idx_reports_transaction_id ON posts_reports(transaction_id);

-- Enable compression policy for reports
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'posts_reports'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE posts_reports SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'posts_reports'
    ) THEN
        PERFORM add_compression_policy('posts_reports', INTERVAL '90 days');
    END IF;
END $$;

-- Transfers table
CREATE TABLE IF NOT EXISTS posts_transfers (
    id SERIAL NOT NULL,
    object_id TEXT NOT NULL,
    previous_owner TEXT NOT NULL,
    new_owner TEXT NOT NULL,
    is_post BOOLEAN NOT NULL,
    transferred_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on transferred_at for all rows
UPDATE posts_transfers SET time = to_timestamp(transferred_at) WHERE time = NOW();

-- Create a trigger to automatically update time from transferred_at for new rows
CREATE OR REPLACE FUNCTION update_transfer_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.transferred_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_transfer_time ON posts_transfers;
CREATE TRIGGER set_transfer_time 
BEFORE INSERT ON posts_transfers
FOR EACH ROW
EXECUTE FUNCTION update_transfer_time();

-- Convert to hypertable first
SELECT create_hypertable('posts_transfers', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'posts_transfers_pkey'
    ) THEN
        ALTER TABLE posts_transfers ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add regular indexes
CREATE INDEX IF NOT EXISTS idx_transfers_object_id ON posts_transfers(object_id, time);
CREATE INDEX IF NOT EXISTS idx_transfers_previous_owner ON posts_transfers(previous_owner, time);
CREATE INDEX IF NOT EXISTS idx_transfers_new_owner ON posts_transfers(new_owner, time);
CREATE INDEX IF NOT EXISTS idx_transfers_transaction_id ON posts_transfers(transaction_id);

-- Enable compression policy for transfers
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'posts_transfers'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE posts_transfers SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'posts_transfers'
    ) THEN
        PERFORM add_compression_policy('posts_transfers', INTERVAL '90 days');
    END IF;
END $$;

-- Moderation events table
CREATE TABLE IF NOT EXISTS posts_moderation_events (
    id SERIAL NOT NULL,
    object_id TEXT NOT NULL,
    platform_id TEXT NOT NULL,
    removed BOOLEAN NOT NULL,
    moderated_by TEXT NOT NULL,
    moderated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on moderated_at for all rows
UPDATE posts_moderation_events SET time = to_timestamp(moderated_at) WHERE time = NOW();

-- Create a trigger to automatically update time from moderated_at for new rows
CREATE OR REPLACE FUNCTION update_moderation_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.moderated_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_moderation_time ON posts_moderation_events;
CREATE TRIGGER set_moderation_time 
BEFORE INSERT ON posts_moderation_events
FOR EACH ROW
EXECUTE FUNCTION update_moderation_time();

-- Convert to hypertable first
SELECT create_hypertable('posts_moderation_events', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'posts_moderation_events_pkey'
    ) THEN
        ALTER TABLE posts_moderation_events ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add regular indexes
CREATE INDEX IF NOT EXISTS idx_moderation_object_id ON posts_moderation_events(object_id, time);
CREATE INDEX IF NOT EXISTS idx_moderation_platform_id ON posts_moderation_events(platform_id, time);
CREATE INDEX IF NOT EXISTS idx_moderation_moderated_by ON posts_moderation_events(moderated_by, time);
CREATE INDEX IF NOT EXISTS idx_moderation_transaction_id ON posts_moderation_events(transaction_id);

-- Enable compression policy for moderation events
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'posts_moderation_events'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE posts_moderation_events SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'posts_moderation_events'
    ) THEN
        PERFORM add_compression_policy('posts_moderation_events', INTERVAL '90 days');
    END IF;
END $$;

-- Deletion events table
CREATE TABLE IF NOT EXISTS posts_deletion_events (
    id SERIAL NOT NULL,
    object_id TEXT NOT NULL,
    owner TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    is_post BOOLEAN NOT NULL,
    post_type TEXT,
    post_id TEXT,
    deleted_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set the time column based on deleted_at for all rows
UPDATE posts_deletion_events SET time = to_timestamp(deleted_at) WHERE time = NOW();

-- Create a trigger to automatically update time from deleted_at for new rows
CREATE OR REPLACE FUNCTION update_deletion_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.deleted_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_deletion_time ON posts_deletion_events;
CREATE TRIGGER set_deletion_time
BEFORE INSERT ON posts_deletion_events
FOR EACH ROW
EXECUTE FUNCTION update_deletion_time();

-- Convert to hypertable first
SELECT create_hypertable('posts_deletion_events', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Now add primary key that includes time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'posts_deletion_events_pkey'
    ) THEN
        ALTER TABLE posts_deletion_events ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add regular indexes
CREATE INDEX IF NOT EXISTS idx_deletion_object_id ON posts_deletion_events(object_id, time);
CREATE INDEX IF NOT EXISTS idx_deletion_owner ON posts_deletion_events(owner, time);
CREATE INDEX IF NOT EXISTS idx_deletion_transaction_id ON posts_deletion_events(transaction_id);

-- Enable compression policy for deletion events
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'posts_deletion_events'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE posts_deletion_events SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'posts_deletion_events'
    ) THEN
        PERFORM add_compression_policy('posts_deletion_events', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 5. CREATE CONTINUOUS AGGREGATES FOR ANALYTICS
-- ============================================================================
-- Create a continuous aggregate for reactions over time
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'reactions_hourly'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW reactions_hourly
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 hour', time) AS bucket,
            object_id,
            reaction_text,
            COUNT(*) AS reaction_count
        FROM reactions
        GROUP BY bucket, object_id, reaction_text
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('reactions_hourly',
            start_offset => INTERVAL '7 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    ELSE
        -- Update policy if view exists but ensure policy is correct
        BEGIN
            PERFORM remove_continuous_aggregate_policy('reactions_hourly', if_exists => TRUE);
            PERFORM add_continuous_aggregate_policy('reactions_hourly',
                start_offset => INTERVAL '7 days',
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '1 hour');
        EXCEPTION WHEN OTHERS THEN
            -- Policy might already exist, just continue
            RAISE NOTICE 'Could not update policy for reactions_hourly: %', SQLERRM;
        END;
    END IF;
END
$$;

-- Create a continuous aggregate for reposts over time
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'reposts_hourly'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW reposts_hourly
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 hour', time) AS bucket,
            original_post_id,
            COUNT(*) AS repost_count
        FROM reposts
        GROUP BY bucket, original_post_id
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('reposts_hourly',
            start_offset => INTERVAL '7 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    ELSE
        -- Update policy if view exists but ensure policy is correct
        BEGIN
            PERFORM remove_continuous_aggregate_policy('reposts_hourly', if_exists => TRUE);
            PERFORM add_continuous_aggregate_policy('reposts_hourly',
                start_offset => INTERVAL '7 days',
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '1 hour');
        EXCEPTION WHEN OTHERS THEN
            -- Policy might already exist, just continue
            RAISE NOTICE 'Could not update policy for reposts_hourly: %', SQLERRM;
        END;
    END IF;
END
$$;

-- Create a continuous aggregate for tips over time
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'tips_hourly'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW tips_hourly
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 hour', time) AS bucket,
            object_id,
            is_post,
            SUM(amount) AS total_amount,
            COUNT(*) AS tip_count
        FROM tips
        GROUP BY bucket, object_id, is_post
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('tips_hourly',
            start_offset => INTERVAL '7 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    ELSE
        -- Update policy if view exists but ensure policy is correct
        BEGIN
            PERFORM remove_continuous_aggregate_policy('tips_hourly', if_exists => TRUE);
            PERFORM add_continuous_aggregate_policy('tips_hourly',
                start_offset => INTERVAL '7 days',
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '1 hour');
        EXCEPTION WHEN OTHERS THEN
            -- Policy might already exist, just continue
            RAISE NOTICE 'Could not update policy for tips_hourly: %', SQLERRM;
        END;
    END IF;
END
$$;

-- Create a continuous aggregate for daily post stats
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'post_stats_daily'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        -- Create the continuous aggregate if it doesn't exist
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW post_stats_daily
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 day', time) AS bucket,
            COUNT(DISTINCT tips.object_id) AS posts_with_tips,
            SUM(amount) AS total_tip_amount,
            COUNT(*) AS total_tips
        FROM tips
        WHERE is_post = TRUE
        GROUP BY bucket
        WITH NO DATA
        $sql$;
        
        -- Set refresh policy
        PERFORM add_continuous_aggregate_policy('post_stats_daily',
            start_offset => INTERVAL '30 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '6 hours');
    ELSE
        -- Update policy if view exists but ensure policy is correct
        BEGIN
            PERFORM remove_continuous_aggregate_policy('post_stats_daily', if_exists => TRUE);
            PERFORM add_continuous_aggregate_policy('post_stats_daily',
                start_offset => INTERVAL '30 days',
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '6 hours');
        EXCEPTION WHEN OTHERS THEN
            -- Policy might already exist, just continue
            RAISE NOTICE 'Could not update policy for post_stats_daily: %', SQLERRM;
        END;
    END IF;
END
$$;

-- ============================================================================
-- 6. CREATE VIEWS FOR BUSINESS LOGIC
-- ============================================================================
-- Create a view for trending posts with advanced algorithm
CREATE OR REPLACE VIEW trending_posts AS
SELECT 
    p.id,
    p.post_id,
    p.owner,
    p.profile_id,
    p.content,
    p.media_urls,
    p.post_type,
    p.created_at,
    p.reaction_count,
    p.comment_count,
    p.repost_count,
    p.tips_received,
    (p.reaction_count * 1 + p.comment_count * 2 + p.repost_count * 3 + p.tips_received) AS engagement_score,
    EXTRACT(EPOCH FROM NOW()) - p.created_at AS age_seconds,
    ((p.reaction_count * 1 + p.comment_count * 2 + p.repost_count * 3 + p.tips_received) / 
     (EXTRACT(EPOCH FROM NOW()) - p.created_at + 3600) * 10000) AS trending_score
FROM 
    posts p
WHERE 
    p.deleted_at IS NULL AND
    p.removed_from_platform = FALSE AND
    (EXTRACT(EPOCH FROM NOW()) - p.created_at) < 604800 -- Last 7 days
ORDER BY 
    trending_score DESC;

-- Create a view for recent popular posts by reaction count
CREATE OR REPLACE VIEW popular_posts AS
SELECT
    p.id,
    p.post_id,
    p.owner,
    p.profile_id,
    p.content,
    p.media_urls,
    p.post_type,
    p.created_at,
    p.reaction_count,
    p.comment_count,
    p.repost_count,
    p.tips_received
FROM
    posts p
WHERE
    p.deleted_at IS NULL AND
    p.removed_from_platform = FALSE AND
    (EXTRACT(EPOCH FROM NOW()) - p.created_at) < 2592000 -- Last 30 days
ORDER BY
    p.reaction_count DESC,
    p.created_at DESC;

-- Create a view for highly tipped content
CREATE OR REPLACE VIEW top_tipped_content AS
WITH tipped_content AS (
    SELECT
        t.object_id,
        t.is_post,
        SUM(t.amount) AS total_tips,
        COUNT(*) AS tip_count
    FROM
        tips t
    WHERE
        (EXTRACT(EPOCH FROM NOW()) - t.created_at) < 2592000 -- Last 30 days
    GROUP BY
        t.object_id, t.is_post
)
SELECT
    tc.object_id,
    CASE WHEN tc.is_post THEN 'post' ELSE 'comment' END AS content_type,
    CASE 
        WHEN tc.is_post THEN p.owner
        ELSE c.owner
    END AS owner,
    CASE 
        WHEN tc.is_post THEN p.profile_id
        ELSE c.profile_id
    END AS profile_id,
    CASE 
        WHEN tc.is_post THEN p.content
        ELSE c.content
    END AS content,
    tc.total_tips,
    tc.tip_count
FROM
    tipped_content tc
LEFT JOIN
    posts p ON tc.is_post AND tc.object_id = p.post_id
LEFT JOIN
    comments c ON NOT tc.is_post AND tc.object_id = c.comment_id
WHERE
    (tc.is_post AND p.deleted_at IS NULL AND p.removed_from_platform = FALSE)
    OR
    (NOT tc.is_post AND c.deleted_at IS NULL AND c.removed_from_platform = FALSE)
ORDER BY
    tc.total_tips DESC;

-- Create a view for post-comment-reaction relationships
CREATE OR REPLACE VIEW post_interactions AS
SELECT
    p.post_id,
    p.owner AS post_owner,
    p.profile_id AS post_profile_id,
    p.content AS post_content,
    p.created_at AS post_created_at,
    c.comment_id,
    c.owner AS comment_owner,
    c.profile_id AS comment_profile_id,
    c.content AS comment_content,
    c.created_at AS comment_created_at,
    r.reaction_text,
    r.user_address AS reaction_user,
    r.created_at AS reaction_created_at
FROM
    posts p
LEFT JOIN
    comments c ON p.post_id = c.post_id
LEFT JOIN
    reactions r ON (r.object_id = p.post_id AND r.is_post = TRUE) 
              OR (r.object_id = c.comment_id AND r.is_post = FALSE)
WHERE
    p.deleted_at IS NULL AND
    (c.deleted_at IS NULL OR c.deleted_at IS NOT NULL);

-- Record the views that have been created in the tracking table
DO $$
BEGIN
    -- Check if tracking table exists, create it if not
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'continuous_aggregate_refresh_status') THEN
        CREATE TABLE continuous_aggregate_refresh_status (
            view_name TEXT PRIMARY KEY,
            last_manual_refresh TIMESTAMP DEFAULT NOW(),
            notes TEXT
        );
    END IF;
END
$$;

-- Insert or update records in the tracking table
INSERT INTO continuous_aggregate_refresh_status (view_name, notes) 
VALUES 
    ('reactions_hourly', 'Created in posts migration'),
    ('reposts_hourly', 'Created in posts migration'),
    ('tips_hourly', 'Created in posts migration'),
    ('post_stats_daily', 'Created in posts migration')
ON CONFLICT (view_name) DO UPDATE 
SET last_manual_refresh = NOW(),
    notes = 'Updated in posts migration'; 