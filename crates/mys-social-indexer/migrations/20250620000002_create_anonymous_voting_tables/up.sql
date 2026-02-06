-- ANONYMOUS VOTING SYSTEM MIGRATION
-- Production-ready implementation with TimescaleDB features

-- ============================================================================
-- 1. UPDATE PROPOSALS TABLE FOR ANONYMOUS VOTING
-- ============================================================================

-- Add anonymous voting fields to existing proposals table
ALTER TABLE proposals ADD COLUMN IF NOT EXISTS anonymous_votes_for BIGINT DEFAULT 0;
ALTER TABLE proposals ADD COLUMN IF NOT EXISTS anonymous_votes_against BIGINT DEFAULT 0;
ALTER TABLE proposals ADD COLUMN IF NOT EXISTS anonymous_voters_count BIGINT DEFAULT 0;
ALTER TABLE proposals ADD COLUMN IF NOT EXISTS pending_anonymous_decryption BOOLEAN DEFAULT FALSE;
ALTER TABLE proposals ADD COLUMN IF NOT EXISTS anonymous_decryption_completed_at BIGINT NULL;

-- Add indexes for anonymous voting fields
CREATE INDEX IF NOT EXISTS idx_proposals_anonymous_votes ON proposals(anonymous_votes_for, anonymous_votes_against, time);
CREATE INDEX IF NOT EXISTS idx_proposals_pending_decryption ON proposals(pending_anonymous_decryption, time) 
    WHERE pending_anonymous_decryption = true;

-- ============================================================================
-- 2. CREATE ANONYMOUS VOTES TABLE
-- ============================================================================

-- Store encrypted votes with TimescaleDB optimization
CREATE TABLE IF NOT EXISTS anonymous_votes (
    id SERIAL NOT NULL,
    proposal_id TEXT NOT NULL,
    voter_address TEXT NOT NULL,
    encrypted_vote_data BYTEA, -- Encrypted vote data from blockchain event
    submitted_at BIGINT NOT NULL,
    decrypted BOOLEAN DEFAULT FALSE,
    decrypted_at BIGINT NULL,
    decrypted_vote SMALLINT NULL, -- 0 for against, 1 for approve, NULL if failed/pending
    decryption_status SMALLINT DEFAULT 0, -- 0: pending, 1: success, 2: failed
    decryption_error TEXT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    processing_success BOOLEAN DEFAULT TRUE,
    processing_error TEXT NULL
);

-- Set time column based on submitted_at
CREATE OR REPLACE FUNCTION update_anonymous_vote_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.submitted_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_anonymous_vote_time ON anonymous_votes;
CREATE TRIGGER set_anonymous_vote_time 
BEFORE INSERT OR UPDATE ON anonymous_votes
FOR EACH ROW
EXECUTE FUNCTION update_anonymous_vote_time();

-- Create hypertable with 1-day chunks (frequent during voting periods)
SELECT create_hypertable('anonymous_votes', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 day');

-- Add primary key including time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'anonymous_votes_pkey'
    ) THEN
        ALTER TABLE anonymous_votes ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Create unique constraint to prevent double voting
CREATE UNIQUE INDEX IF NOT EXISTS idx_anonymous_votes_unique_vote 
ON anonymous_votes(proposal_id, voter_address, time);

-- Add other indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_anonymous_votes_proposal_time ON anonymous_votes(proposal_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_anonymous_votes_voter_time ON anonymous_votes(voter_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_anonymous_votes_status_time ON anonymous_votes(decryption_status, time DESC);
CREATE INDEX IF NOT EXISTS idx_anonymous_votes_decrypted_time ON anonymous_votes(decrypted, time DESC) 
    WHERE decrypted = true;
CREATE INDEX IF NOT EXISTS idx_anonymous_votes_transaction_id ON anonymous_votes(transaction_id);

-- Add validation trigger for proposal_id reference
CREATE OR REPLACE FUNCTION validate_anonymous_vote_proposal()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM proposals WHERE id = NEW.proposal_id) THEN
        RAISE EXCEPTION 'Referenced proposal_id does not exist: %', NEW.proposal_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS check_anonymous_vote_proposal ON anonymous_votes;
CREATE TRIGGER check_anonymous_vote_proposal
BEFORE INSERT OR UPDATE ON anonymous_votes
FOR EACH ROW
EXECUTE FUNCTION validate_anonymous_vote_proposal();

-- Enable compression on anonymous_votes table for historical data
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'anonymous_votes'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE anonymous_votes SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'anonymous_votes'
    ) THEN
        PERFORM add_compression_policy('anonymous_votes', INTERVAL '30 days');
    END IF;
END $$;

-- ============================================================================
-- 3. CREATE VOTE DECRYPTION FAILURES TABLE
-- ============================================================================

-- Track decryption failures for transparency and debugging
CREATE TABLE IF NOT EXISTS vote_decryption_failures (
    id SERIAL NOT NULL,
    proposal_id TEXT NOT NULL,
    voter_address TEXT NOT NULL,
    failure_reason TEXT NOT NULL,
    attempted_at BIGINT NOT NULL,
    encrypted_vote_length INTEGER, -- For debugging
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Set time column based on attempted_at
CREATE OR REPLACE FUNCTION update_decryption_failure_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.attempted_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_decryption_failure_time ON vote_decryption_failures;
CREATE TRIGGER set_decryption_failure_time 
BEFORE INSERT OR UPDATE ON vote_decryption_failures
FOR EACH ROW
EXECUTE FUNCTION update_decryption_failure_time();

-- Create hypertable with 30-day chunks
SELECT create_hypertable('vote_decryption_failures', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '30 days');

-- Add primary key including time
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'vote_decryption_failures_pkey'
    ) THEN
        ALTER TABLE vote_decryption_failures ADD PRIMARY KEY (id, time);
    END IF;
END $$;

-- Add indexes
CREATE INDEX IF NOT EXISTS idx_decryption_failures_proposal_time ON vote_decryption_failures(proposal_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_decryption_failures_voter_time ON vote_decryption_failures(voter_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_decryption_failures_reason_time ON vote_decryption_failures(failure_reason, time DESC);
CREATE INDEX IF NOT EXISTS idx_decryption_failures_transaction_id ON vote_decryption_failures(transaction_id);

-- Enable compression on failures table for historical data
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' 
        AND c.relname = 'vote_decryption_failures'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE vote_decryption_failures SET (timescaledb.compress = true);
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'vote_decryption_failures'
    ) THEN
        PERFORM add_compression_policy('vote_decryption_failures', INTERVAL '90 days');
    END IF;
END $$;

-- ============================================================================
-- 4. CREATE CONTINUOUS AGGREGATES FOR ANALYTICS
-- ============================================================================

-- Pre-computed analytics for anonymous voting patterns
-- Create continuous aggregate without initial data to avoid transaction block issues
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.continuous_aggregates 
        WHERE view_name = 'anonymous_voting_daily_stats'
    ) THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW anonymous_voting_daily_stats
        WITH (timescaledb.continuous) AS
        SELECT
            time_bucket('1 day', time) AS day,
            proposal_id,
            COUNT(*) as total_anonymous_votes,
            COUNT(*) FILTER (WHERE decrypted = true) as successfully_decrypted,
            COUNT(*) FILTER (WHERE decryption_status = 2) as failed_decryptions,
            COUNT(*) FILTER (WHERE decrypted_vote = 1) as anonymous_votes_for,
            COUNT(*) FILTER (WHERE decrypted_vote = 0) as anonymous_votes_against,
            COUNT(*) FILTER (WHERE decryption_status = 0) as pending_decryption
        FROM anonymous_votes
        GROUP BY day, proposal_id
        WITH NO DATA
        $sql$;
        
        PERFORM add_continuous_aggregate_policy('anonymous_voting_daily_stats',
            start_offset => INTERVAL '3 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour');
    END IF;
END $$;

-- Note: Continuous aggregate will be populated by the refresh policy automatically
-- Initial data will be available after the first scheduled refresh (1 hour interval)

-- ============================================================================
-- 5. UPDATE GOVERNANCE EVENTS TABLE FOR ANONYMOUS VOTING (CONDITIONAL)
-- ============================================================================

-- Add anonymous voting flag to governance_events table only if it exists
DO $$
BEGIN
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'governance_events') THEN
        -- Add anonymous voting flag to existing governance_events table
        ALTER TABLE governance_events ADD COLUMN IF NOT EXISTS anonymous_voting_related BOOLEAN DEFAULT FALSE;
        
        -- Add index for anonymous voting events
        CREATE INDEX IF NOT EXISTS idx_governance_events_anonymous ON governance_events(anonymous_voting_related, created_at DESC) 
            WHERE anonymous_voting_related = true;
    ELSE
        RAISE NOTICE 'governance_events table does not exist, skipping anonymous voting integration';
    END IF;
END $$;

-- ============================================================================
-- 6. ADD RETENTION POLICIES
-- ============================================================================

-- Keep raw anonymous votes for 2 years
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_retention' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'anonymous_votes'
    ) THEN
        PERFORM add_retention_policy('anonymous_votes', INTERVAL '2 years');
    END IF;
END $$;

-- Keep decryption failures for 1 year
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_retention' 
        AND hypertable_schema = 'public' 
        AND hypertable_name = 'vote_decryption_failures'
    ) THEN
        PERFORM add_retention_policy('vote_decryption_failures', INTERVAL '1 year');
    END IF;
END $$; 