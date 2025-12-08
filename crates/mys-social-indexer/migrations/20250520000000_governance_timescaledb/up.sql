-- GOVERNANCE SYSTEM WITH TIMESCALEDB INTEGRATION
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
-- 2. CREATE CONFIGURATION TABLES
-- ============================================================================
-- Governance registries table - configuration data for governance types
CREATE TABLE IF NOT EXISTS governance_registries (
    id SERIAL PRIMARY KEY,
    registry_type SMALLINT NOT NULL,  -- 0: Ecosystem, 1: Reputation, 2: Community Notes
    delegate_count BIGINT NOT NULL,
    delegate_term_epochs BIGINT NOT NULL,
    proposal_submission_cost BIGINT NOT NULL,
    min_on_chain_age_days BIGINT NOT NULL,
    max_votes_per_user BIGINT NOT NULL,
    quadratic_base_cost BIGINT NOT NULL,
    voting_period_epochs BIGINT NOT NULL,
    quorum_votes BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set time column based on updated_at
CREATE OR REPLACE FUNCTION update_registry_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_registry_time ON governance_registries;
CREATE TRIGGER set_registry_time 
BEFORE INSERT OR UPDATE ON governance_registries
FOR EACH ROW
EXECUTE FUNCTION update_registry_time();

-- Create unique constraint on registry_type
CREATE UNIQUE INDEX IF NOT EXISTS idx_governance_registries_type ON governance_registries(registry_type);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_governance_registries_updated_at ON governance_registries(updated_at);
CREATE INDEX IF NOT EXISTS idx_governance_registries_transaction_id ON governance_registries(transaction_id);

-- ============================================================================
-- 3. CREATE ENTITY TABLES
-- ============================================================================
-- Delegates table - elected representatives
CREATE TABLE IF NOT EXISTS delegates (
    id SERIAL NOT NULL,
    address TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    registry_type SMALLINT NOT NULL,
    upvotes BIGINT NOT NULL DEFAULT 0,
    downvotes BIGINT NOT NULL DEFAULT 0,
    proposals_reviewed BIGINT NOT NULL DEFAULT 0,
    proposals_submitted BIGINT NOT NULL DEFAULT 0,
    sided_winning_proposals BIGINT NOT NULL DEFAULT 0,
    sided_losing_proposals BIGINT NOT NULL DEFAULT 0,
    term_start BIGINT NOT NULL,
    term_end BIGINT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set time column based on created_at/updated_at
CREATE OR REPLACE FUNCTION update_delegate_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_delegate_time ON delegates;
CREATE TRIGGER set_delegate_time 
BEFORE INSERT OR UPDATE ON delegates
FOR EACH ROW
EXECUTE FUNCTION update_delegate_time();

-- Convert to hypertable for historical tracking
SELECT create_hypertable('delegates', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Add primary key including time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'delegates_pkey'
    ) THEN
        ALTER TABLE delegates ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create constraint for uniqueness within time chunk
CREATE UNIQUE INDEX IF NOT EXISTS idx_delegates_address_type_time ON delegates(address, registry_type, time);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_delegates_address ON delegates(address, time);
CREATE INDEX IF NOT EXISTS idx_delegates_profile_id ON delegates(profile_id, time);
CREATE INDEX IF NOT EXISTS idx_delegates_registry_type ON delegates(registry_type, time);
CREATE INDEX IF NOT EXISTS idx_delegates_term_end ON delegates(term_end, time);
CREATE INDEX IF NOT EXISTS idx_delegates_is_active ON delegates(is_active, time);
CREATE INDEX IF NOT EXISTS idx_delegates_transaction_id ON delegates(transaction_id);

-- Enable compression on delegates table for historical data
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'delegates'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE delegates SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'delegates'
    ) THEN
        PERFORM add_compression_policy('delegates', INTERVAL '90 days');
    END IF;
END $$;

-- Nominated delegates table - candidates for delegate positions
CREATE TABLE IF NOT EXISTS nominated_delegates (
    id SERIAL NOT NULL,
    address TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    registry_type SMALLINT NOT NULL,
    upvotes BIGINT NOT NULL DEFAULT 0,
    downvotes BIGINT NOT NULL DEFAULT 0,
    scheduled_term_start_epoch BIGINT NOT NULL,
    nomination_time BIGINT NOT NULL,
    status SMALLINT NOT NULL DEFAULT 0, -- 0: Pending, 1: Elected, 2: Rejected
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set time column based on nomination_time
CREATE OR REPLACE FUNCTION update_nominee_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.nomination_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_nominee_time ON nominated_delegates;
CREATE TRIGGER set_nominee_time 
BEFORE INSERT OR UPDATE ON nominated_delegates
FOR EACH ROW
EXECUTE FUNCTION update_nominee_time();

-- Convert to hypertable for historical tracking
SELECT create_hypertable('nominated_delegates', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Add primary key including time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'nominated_delegates_pkey'
    ) THEN
        ALTER TABLE nominated_delegates ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create constraint for uniqueness within time chunk
CREATE UNIQUE INDEX IF NOT EXISTS idx_nominees_address_type_time ON nominated_delegates(address, registry_type, time);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_nominees_address ON nominated_delegates(address, time);
CREATE INDEX IF NOT EXISTS idx_nominees_profile_id ON nominated_delegates(profile_id, time);
CREATE INDEX IF NOT EXISTS idx_nominees_registry_type ON nominated_delegates(registry_type, time);
CREATE INDEX IF NOT EXISTS idx_nominees_status ON nominated_delegates(status, time);
CREATE INDEX IF NOT EXISTS idx_nominees_scheduled_term ON nominated_delegates(scheduled_term_start_epoch, time);
CREATE INDEX IF NOT EXISTS idx_nominees_transaction_id ON nominated_delegates(transaction_id);

-- Enable compression on nominated_delegates table for historical data
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'nominated_delegates'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE nominated_delegates SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'nominated_delegates'
    ) THEN
        PERFORM add_compression_policy('nominated_delegates', INTERVAL '90 days');
    END IF;
END $$;

-- Proposals table - governance proposals
CREATE TABLE IF NOT EXISTS proposals (
    id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    proposal_type SMALLINT NOT NULL, -- 0: Ecosystem, 1: Reputation, 2: Community Notes
    reference_id TEXT,
    metadata_json JSONB,
    submitter TEXT NOT NULL,
    submission_time BIGINT NOT NULL,
    delegate_approval_count BIGINT NOT NULL DEFAULT 0,
    delegate_rejection_count BIGINT NOT NULL DEFAULT 0,
    community_votes_for BIGINT NOT NULL DEFAULT 0,
    community_votes_against BIGINT NOT NULL DEFAULT 0,
    status SMALLINT NOT NULL, -- 0: Submitted, 1: DelegateReview, 2: CommunityVoting, 3: Approved, 4: Rejected, 5: Implemented, 6: OwnerRescinded
    voting_start_time BIGINT,
    voting_end_time BIGINT,
    reward_pool BIGINT NOT NULL,
    implemented_description TEXT,
    implementation_time BIGINT,
    rescind_time BIGINT,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set time column based on submission_time
CREATE OR REPLACE FUNCTION update_proposal_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.submission_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_proposal_time ON proposals;
CREATE TRIGGER set_proposal_time 
BEFORE INSERT OR UPDATE ON proposals
FOR EACH ROW
EXECUTE FUNCTION update_proposal_time();

-- Convert to hypertable for historical tracking
SELECT create_hypertable('proposals', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

-- Add primary key including time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'proposals_pkey'
    ) THEN
        ALTER TABLE proposals ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_proposals_proposal_type ON proposals(proposal_type, time);
CREATE INDEX IF NOT EXISTS idx_proposals_submitter ON proposals(submitter, time);
CREATE INDEX IF NOT EXISTS idx_proposals_status ON proposals(status, time);
CREATE INDEX IF NOT EXISTS idx_proposals_reference_id ON proposals(reference_id, time) 
    WHERE reference_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_proposals_voting_end_time ON proposals(voting_end_time, time) 
    WHERE voting_end_time IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_proposals_transaction_id ON proposals(transaction_id);

-- Enable compression on proposals table for historical data
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'proposals'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE proposals SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'proposals'
    ) THEN
        PERFORM add_compression_policy('proposals', INTERVAL '180 days');
    END IF;
END $$;

-- ============================================================================
-- 4. CREATE TIME-SERIES ACTIVITY TABLES
-- ============================================================================
-- Delegate ratings table - feedback on delegates
CREATE TABLE IF NOT EXISTS delegate_ratings (
    id SERIAL NOT NULL,
    target_address TEXT NOT NULL,
    voter_address TEXT NOT NULL,
    registry_type SMALLINT NOT NULL,
    is_active_delegate BOOLEAN NOT NULL,
    upvote BOOLEAN NOT NULL,
    rated_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set time column based on rated_at
CREATE OR REPLACE FUNCTION update_rating_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.rated_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_rating_time ON delegate_ratings;
CREATE TRIGGER set_rating_time 
BEFORE INSERT OR UPDATE ON delegate_ratings
FOR EACH ROW
EXECUTE FUNCTION update_rating_time();

-- Convert to hypertable with chunking by time
SELECT create_hypertable('delegate_ratings', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE, 
                          chunk_time_interval => INTERVAL '1 week');

-- Add primary key including time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'delegate_ratings_pkey'
    ) THEN
        ALTER TABLE delegate_ratings ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint that includes time
CREATE UNIQUE INDEX IF NOT EXISTS idx_ratings_target_voter_registry_time 
ON delegate_ratings(target_address, voter_address, registry_type, time);

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_ratings_target_address ON delegate_ratings(target_address, time);
CREATE INDEX IF NOT EXISTS idx_ratings_voter_address ON delegate_ratings(voter_address, time);
CREATE INDEX IF NOT EXISTS idx_ratings_registry_type ON delegate_ratings(registry_type, time);
CREATE INDEX IF NOT EXISTS idx_ratings_transaction_id ON delegate_ratings(transaction_id);

-- Enable compression policy
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'delegate_ratings'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE delegate_ratings SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'delegate_ratings'
    ) THEN
        PERFORM add_compression_policy('delegate_ratings', INTERVAL '30 days');
    END IF;
END $$;

-- Delegate votes table - how delegates vote on proposals
CREATE TABLE IF NOT EXISTS delegate_votes (
    id SERIAL NOT NULL,
    proposal_id TEXT NOT NULL,
    delegate_address TEXT NOT NULL,
    approve BOOLEAN NOT NULL,
    vote_time BIGINT NOT NULL,
    reason TEXT,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set time column based on vote_time
CREATE OR REPLACE FUNCTION update_delegate_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.vote_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_delegate_vote_time ON delegate_votes;
CREATE TRIGGER set_delegate_vote_time 
BEFORE INSERT OR UPDATE ON delegate_votes
FOR EACH ROW
EXECUTE FUNCTION update_delegate_vote_time();

-- Convert to hypertable
SELECT create_hypertable('delegate_votes', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE, 
                          chunk_time_interval => INTERVAL '1 week');

-- Add primary key
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'delegate_votes_pkey'
    ) THEN
        ALTER TABLE delegate_votes ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint with time
CREATE UNIQUE INDEX IF NOT EXISTS idx_delegate_votes_proposal_delegate_time 
ON delegate_votes(proposal_id, delegate_address, time);

-- Add validation trigger for proposal_id reference
CREATE OR REPLACE FUNCTION validate_proposal_delegate_vote()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM proposals WHERE id = NEW.proposal_id) THEN
        RAISE EXCEPTION 'Referenced proposal_id does not exist: %', NEW.proposal_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS check_proposal_delegate_vote ON delegate_votes;
CREATE TRIGGER check_proposal_delegate_vote
BEFORE INSERT OR UPDATE ON delegate_votes
FOR EACH ROW
EXECUTE FUNCTION validate_proposal_delegate_vote();

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_delegate_votes_proposal_id ON delegate_votes(proposal_id, time);
CREATE INDEX IF NOT EXISTS idx_delegate_votes_delegate_address ON delegate_votes(delegate_address, time);
CREATE INDEX IF NOT EXISTS idx_delegate_votes_approve ON delegate_votes(approve, time);
CREATE INDEX IF NOT EXISTS idx_delegate_votes_transaction_id ON delegate_votes(transaction_id);

-- Enable compression policy
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'delegate_votes'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE delegate_votes SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'delegate_votes'
    ) THEN
        PERFORM add_compression_policy('delegate_votes', INTERVAL '30 days');
    END IF;
END $$;

-- Community votes table - how community members vote on proposals
CREATE TABLE IF NOT EXISTS community_votes (
    id SERIAL NOT NULL,
    proposal_id TEXT NOT NULL,
    voter_address TEXT NOT NULL,
    vote_weight BIGINT NOT NULL,
    approve BOOLEAN NOT NULL,
    vote_time BIGINT NOT NULL,
    vote_cost BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set time column based on vote_time
CREATE OR REPLACE FUNCTION update_community_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.vote_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_community_vote_time ON community_votes;
CREATE TRIGGER set_community_vote_time 
BEFORE INSERT OR UPDATE ON community_votes
FOR EACH ROW
EXECUTE FUNCTION update_community_vote_time();

-- Convert to hypertable
SELECT create_hypertable('community_votes', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE, 
                          chunk_time_interval => INTERVAL '1 week');

-- Add primary key
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'community_votes_pkey'
    ) THEN
        ALTER TABLE community_votes ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint with time
CREATE UNIQUE INDEX IF NOT EXISTS idx_community_votes_proposal_voter_time 
ON community_votes(proposal_id, voter_address, time);

-- Add validation trigger for proposal_id reference
CREATE OR REPLACE FUNCTION validate_proposal_community_vote()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM proposals WHERE id = NEW.proposal_id) THEN
        RAISE EXCEPTION 'Referenced proposal_id does not exist: %', NEW.proposal_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS check_proposal_community_vote ON community_votes;
CREATE TRIGGER check_proposal_community_vote
BEFORE INSERT OR UPDATE ON community_votes
FOR EACH ROW
EXECUTE FUNCTION validate_proposal_community_vote();

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_community_votes_proposal_id ON community_votes(proposal_id, time);
CREATE INDEX IF NOT EXISTS idx_community_votes_voter_address ON community_votes(voter_address, time);
CREATE INDEX IF NOT EXISTS idx_community_votes_approve ON community_votes(approve, time);
CREATE INDEX IF NOT EXISTS idx_community_votes_vote_weight ON community_votes(vote_weight, time);
CREATE INDEX IF NOT EXISTS idx_community_votes_transaction_id ON community_votes(transaction_id);

-- Enable compression policy
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'community_votes'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE community_votes SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'community_votes'
    ) THEN
        PERFORM add_compression_policy('community_votes', INTERVAL '30 days');
    END IF;
END $$;

-- Reward distributions table - payouts for proposal participation
CREATE TABLE IF NOT EXISTS reward_distributions (
    id SERIAL NOT NULL,
    proposal_id TEXT NOT NULL,
    recipient_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    distribution_time BIGINT NOT NULL,
    distribution_type TEXT, -- 'winner_reward', 'refund', 'rescind_refund'
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set time column based on distribution_time
CREATE OR REPLACE FUNCTION update_distribution_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.distribution_time);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_distribution_time ON reward_distributions;
CREATE TRIGGER set_distribution_time 
BEFORE INSERT OR UPDATE ON reward_distributions
FOR EACH ROW
EXECUTE FUNCTION update_distribution_time();

-- Convert to hypertable
SELECT create_hypertable('reward_distributions', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE, 
                          chunk_time_interval => INTERVAL '1 month');

-- Add primary key
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'reward_distributions_pkey'
    ) THEN
        ALTER TABLE reward_distributions ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add validation trigger for proposal_id reference
CREATE OR REPLACE FUNCTION validate_proposal_reward()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM proposals WHERE id = NEW.proposal_id) THEN
        RAISE EXCEPTION 'Referenced proposal_id does not exist: %', NEW.proposal_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS check_proposal_reward ON reward_distributions;
CREATE TRIGGER check_proposal_reward
BEFORE INSERT OR UPDATE ON reward_distributions
FOR EACH ROW
EXECUTE FUNCTION validate_proposal_reward();

-- Add other indexes
CREATE INDEX IF NOT EXISTS idx_reward_proposal_id ON reward_distributions(proposal_id, time);
CREATE INDEX IF NOT EXISTS idx_reward_recipient_address ON reward_distributions(recipient_address, time);
CREATE INDEX IF NOT EXISTS idx_reward_amount ON reward_distributions(amount, time);
CREATE INDEX IF NOT EXISTS idx_reward_distribution_type ON reward_distributions(distribution_type, time);
CREATE INDEX IF NOT EXISTS idx_reward_transaction_id ON reward_distributions(transaction_id);

-- Enable compression policy
DO $$
BEGIN
    -- Enable compression if not already enabled
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'reward_distributions'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE reward_distributions SET (timescaledb.compress = true);
    END IF;
    
    -- Check if a compression policy job already exists
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'reward_distributions'
    ) THEN
        PERFORM add_compression_policy('reward_distributions', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 5. CREATE CONTINUOUS AGGREGATES FOR ANALYTICS
-- ============================================================================
-- Daily delegate rating aggregates
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'delegate_ratings_daily'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW delegate_ratings_daily
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 day', time) AS day,
            registry_type,
            target_address,
            SUM(CASE WHEN upvote THEN 1 ELSE 0 END) AS upvotes,
            SUM(CASE WHEN NOT upvote THEN 1 ELSE 0 END) AS downvotes,
            COUNT(*) AS total_ratings
        FROM delegate_ratings
        GROUP BY day, registry_type, target_address
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('delegate_ratings_daily',
            start_offset => INTERVAL '30 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 day');
    END IF;
END
$$;

-- Hourly delegate voting activity on proposals
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'delegate_voting_hourly'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW delegate_voting_hourly
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 hour', time) AS hour,
            proposal_id,
            SUM(CASE WHEN approve THEN 1 ELSE 0 END) AS approve_count,
            SUM(CASE WHEN NOT approve THEN 1 ELSE 0 END) AS reject_count,
            COUNT(*) AS total_votes
        FROM delegate_votes
        GROUP BY hour, proposal_id
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('delegate_voting_hourly',
            start_offset => INTERVAL '14 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END
$$;

-- Hourly community voting activity on proposals
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'community_voting_hourly'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW community_voting_hourly
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 hour', time) AS hour,
            proposal_id,
            SUM(CASE WHEN approve THEN vote_weight ELSE 0 END) AS approve_weight,
            SUM(CASE WHEN NOT approve THEN vote_weight ELSE 0 END) AS reject_weight,
            COUNT(*) AS total_votes
        FROM community_votes
        GROUP BY hour, proposal_id
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('community_voting_hourly',
            start_offset => INTERVAL '14 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END
$$;

-- Daily reward distribution stats
DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    -- Check if the continuous aggregate already exists using TimescaleDB information schema
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'rewards_daily'
    ) INTO view_exists;
    
    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW rewards_daily
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 day', time) AS day,
            distribution_type,
            COUNT(*) AS distribution_count,
            SUM(amount) AS total_amount
        FROM reward_distributions
        GROUP BY day, distribution_type
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('rewards_daily',
            start_offset => INTERVAL '60 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 day');
    END IF;
END
$$;

-- ============================================================================
-- 6. CREATE VIEWS FOR BUSINESS LOGIC
-- ============================================================================
-- Create view for governance stats
CREATE OR REPLACE VIEW governance_stats AS
SELECT
    g.registry_type,
    COUNT(DISTINCT d.id) AS active_delegates,
    COUNT(DISTINCT n.id) AS pending_nominees,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 0) AS submitted_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 1) AS in_review_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 2) AS voting_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 3) AS approved_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 4) AS rejected_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 5) AS implemented_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 6) AS rescinded_proposals
FROM
    governance_registries g
LEFT JOIN
    delegates d ON g.registry_type = d.registry_type AND d.is_active = true
LEFT JOIN
    nominated_delegates n ON g.registry_type = n.registry_type AND n.status = 0
LEFT JOIN
    proposals p ON g.registry_type = p.proposal_type
GROUP BY
    g.registry_type;

-- Create view for delegate performance
CREATE OR REPLACE VIEW delegate_performance AS
SELECT
    d.address,
    d.profile_id,
    d.registry_type,
    d.upvotes,
    d.downvotes,
    d.proposals_reviewed,
    d.proposals_submitted,
    d.sided_winning_proposals,
    d.sided_losing_proposals,
    d.term_start,
    d.term_end,
    d.is_active,
    CASE 
        WHEN d.proposals_reviewed > 0 THEN 
            d.sided_winning_proposals::FLOAT / NULLIF(d.proposals_reviewed, 0) 
        ELSE NULL 
    END AS winning_rate,
    COUNT(DISTINCT dv.proposal_id) AS recent_votes,
    SUM(CASE WHEN dv.approve THEN 1 ELSE 0 END) AS recent_approvals,
    SUM(CASE WHEN NOT dv.approve THEN 1 ELSE 0 END) AS recent_rejections
FROM
    delegates d
LEFT JOIN
    delegate_votes dv ON d.address = dv.delegate_address 
                      AND dv.time > NOW() - INTERVAL '30 days'
GROUP BY
    d.id, d.address, d.profile_id, d.registry_type, d.upvotes, d.downvotes, 
    d.proposals_reviewed, d.proposals_submitted, d.sided_winning_proposals, 
    d.sided_losing_proposals, d.term_start, d.term_end, d.is_active;

-- Create view for proposal voting status
CREATE OR REPLACE VIEW proposal_voting_status AS
SELECT
    p.id AS proposal_id,
    p.title,
    p.proposal_type,
    p.status,
    p.submitter,
    p.delegate_approval_count,
    p.delegate_rejection_count,
    p.community_votes_for,
    p.community_votes_against,
    p.voting_start_time,
    p.voting_end_time,
    COALESCE(dv_counts.total_delegate_votes, 0) AS confirmed_delegate_votes,
    COALESCE(cv_counts.total_community_voters, 0) AS confirmed_community_voters,
    COALESCE(cv_counts.total_community_weight, 0) AS confirmed_community_weight,
    CASE
        WHEN p.status = 2 AND p.voting_end_time > extract(epoch from now()) THEN
            (p.voting_end_time - extract(epoch from now())) / 86400.0
        ELSE NULL
    END AS days_remaining
FROM
    proposals p
LEFT JOIN (
    SELECT
        proposal_id,
        COUNT(*) AS total_delegate_votes
    FROM
        delegate_votes
    GROUP BY
        proposal_id
) dv_counts ON p.id = dv_counts.proposal_id
LEFT JOIN (
    SELECT
        proposal_id,
        COUNT(*) AS total_community_voters,
        SUM(vote_weight) AS total_community_weight
    FROM
        community_votes
    GROUP BY
        proposal_id
) cv_counts ON p.id = cv_counts.proposal_id;

-- Create view for combined voting activity (used instead of continuous aggregate)
CREATE OR REPLACE VIEW voting_activity AS
SELECT
    'delegate' AS voter_type,
    hour,
    proposal_id,
    approve_count,
    reject_count,
    total_votes,
    0 AS approve_weight,
    0 AS reject_weight
FROM
    delegate_voting_hourly
UNION ALL
SELECT
    'community' AS voter_type,
    hour,
    proposal_id,
    0 AS approve_count,
    0 AS reject_count,
    total_votes,
    approve_weight,
    reject_weight
FROM
    community_voting_hourly;

-- Create a tracking table for continuous aggregate refresh status
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'continuous_aggregate_refresh_status') THEN
        CREATE TABLE continuous_aggregate_refresh_status (
            view_name TEXT PRIMARY KEY,
            last_manual_refresh TIMESTAMP DEFAULT NOW(),
            notes TEXT
        );
    END IF;
END
$$;

-- Record the views that have been created in the tracking table
INSERT INTO continuous_aggregate_refresh_status (view_name, notes) 
VALUES 
    ('delegate_ratings_daily', 'Created in governance migration'),
    ('delegate_voting_hourly', 'Created in governance migration'),
    ('community_voting_hourly', 'Created in governance migration'),
    ('rewards_daily', 'Created in governance migration')
ON CONFLICT (view_name) DO UPDATE 
SET last_manual_refresh = NOW(),
    notes = 'Updated in governance migration'; 