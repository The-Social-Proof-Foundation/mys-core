-- Remove DAO governance fields from platforms table
ALTER TABLE platforms
DROP COLUMN IF EXISTS wants_dao_governance,
DROP COLUMN IF EXISTS governance_registry_id,
DROP COLUMN IF EXISTS delegate_count,
DROP COLUMN IF EXISTS delegate_term_epochs,
DROP COLUMN IF EXISTS max_votes_per_user,
DROP COLUMN IF EXISTS min_on_chain_age_days,
DROP COLUMN IF EXISTS proposal_submission_cost,
DROP COLUMN IF EXISTS quadratic_base_cost,
DROP COLUMN IF EXISTS quorum_votes,
DROP COLUMN IF EXISTS voting_period_epochs,
DROP COLUMN IF EXISTS treasury,
DROP COLUMN IF EXISTS version;

