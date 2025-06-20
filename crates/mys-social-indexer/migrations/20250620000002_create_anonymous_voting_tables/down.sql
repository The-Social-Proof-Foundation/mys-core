-- ANONYMOUS VOTING SYSTEM ROLLBACK
-- Reverse all changes from the anonymous voting migration

-- ============================================================================
-- 1. DROP RETENTION POLICIES
-- ============================================================================

-- Remove retention policies
SELECT remove_retention_policy('anonymous_votes', if_exists => true);
SELECT remove_retention_policy('vote_decryption_failures', if_exists => true);

-- ============================================================================
-- 2. DROP CONTINUOUS AGGREGATES
-- ============================================================================

-- Remove continuous aggregate policy
SELECT remove_continuous_aggregate_policy('anonymous_voting_daily_stats', if_exists => true);

-- Drop continuous aggregate
DROP MATERIALIZED VIEW IF EXISTS anonymous_voting_daily_stats;

-- ============================================================================
-- 3. REMOVE GOVERNANCE EVENTS TABLE CHANGES
-- ============================================================================

-- Drop anonymous voting index
DROP INDEX IF EXISTS idx_governance_events_anonymous;

-- Remove anonymous voting column
ALTER TABLE governance_events DROP COLUMN IF EXISTS anonymous_voting_related;

-- ============================================================================
-- 4. DROP VOTE DECRYPTION FAILURES TABLE
-- ============================================================================

-- Drop compression policy
SELECT remove_compression_policy('vote_decryption_failures', if_exists => true);

-- Drop triggers and functions
DROP TRIGGER IF EXISTS set_decryption_failure_time ON vote_decryption_failures;
DROP FUNCTION IF EXISTS update_decryption_failure_time();

-- Drop table (this will automatically drop the hypertable)
DROP TABLE IF EXISTS vote_decryption_failures;

-- ============================================================================
-- 5. DROP ANONYMOUS VOTES TABLE
-- ============================================================================

-- Drop compression policy
SELECT remove_compression_policy('anonymous_votes', if_exists => true);

-- Drop triggers and functions
DROP TRIGGER IF EXISTS check_anonymous_vote_proposal ON anonymous_votes;
DROP TRIGGER IF EXISTS set_anonymous_vote_time ON anonymous_votes;
DROP FUNCTION IF EXISTS validate_anonymous_vote_proposal();
DROP FUNCTION IF EXISTS update_anonymous_vote_time();

-- Drop table (this will automatically drop the hypertable)
DROP TABLE IF EXISTS anonymous_votes;

-- ============================================================================
-- 6. REMOVE PROPOSALS TABLE CHANGES
-- ============================================================================

-- Drop indexes for anonymous voting fields
DROP INDEX IF EXISTS idx_proposals_pending_decryption;
DROP INDEX IF EXISTS idx_proposals_anonymous_votes;

-- Remove anonymous voting columns from proposals table
ALTER TABLE proposals DROP COLUMN IF EXISTS anonymous_decryption_completed_at;
ALTER TABLE proposals DROP COLUMN IF EXISTS pending_anonymous_decryption;
ALTER TABLE proposals DROP COLUMN IF EXISTS anonymous_voters_count;
ALTER TABLE proposals DROP COLUMN IF EXISTS anonymous_votes_against;
ALTER TABLE proposals DROP COLUMN IF EXISTS anonymous_votes_for; 