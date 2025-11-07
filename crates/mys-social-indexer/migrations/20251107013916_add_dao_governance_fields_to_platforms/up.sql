-- Add DAO governance fields to platforms table
ALTER TABLE platforms
ADD COLUMN wants_dao_governance BOOLEAN,
ADD COLUMN governance_registry_id TEXT,
ADD COLUMN delegate_count BIGINT,
ADD COLUMN delegate_term_epochs BIGINT,
ADD COLUMN max_votes_per_user BIGINT,
ADD COLUMN min_on_chain_age_days BIGINT,
ADD COLUMN proposal_submission_cost BIGINT,
ADD COLUMN quadratic_base_cost BIGINT,
ADD COLUMN quorum_votes BIGINT,
ADD COLUMN voting_period_epochs BIGINT,
ADD COLUMN treasury BIGINT,
ADD COLUMN version BIGINT;

-- Note: shutdown_date already exists in the schema but wasn't being populated from events
-- This migration adds all the missing DAO/governance fields

