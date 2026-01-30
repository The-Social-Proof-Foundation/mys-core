-- DROP VIEWS AND CONTINUOUS AGGREGATES
DROP VIEW IF EXISTS governance_stats;
DROP VIEW IF EXISTS delegate_performance;
DROP VIEW IF EXISTS proposal_voting_status;
DROP VIEW IF EXISTS voting_activity;

DROP MATERIALIZED VIEW IF EXISTS delegate_ratings_daily;
DROP MATERIALIZED VIEW IF EXISTS delegate_voting_hourly;
DROP MATERIALIZED VIEW IF EXISTS community_voting_hourly;
DROP MATERIALIZED VIEW IF EXISTS rewards_daily;

-- REMOVE ENTRIES FROM TRACKING TABLE
DELETE FROM continuous_aggregate_refresh_status 
WHERE view_name IN ('delegate_ratings_daily', 'delegate_voting_hourly', 'community_voting_hourly', 'rewards_daily');

-- DROP TRIGGERS
DROP TRIGGER IF EXISTS set_registry_time ON governance_registries;
DROP TRIGGER IF EXISTS set_delegate_time ON delegates;
DROP TRIGGER IF EXISTS set_nominee_time ON nominated_delegates;
DROP TRIGGER IF EXISTS set_proposal_time ON proposals;
DROP TRIGGER IF EXISTS set_rating_time ON delegate_ratings;
DROP TRIGGER IF EXISTS set_delegate_vote_time ON delegate_votes;
DROP TRIGGER IF EXISTS check_proposal_delegate_vote ON delegate_votes;
DROP TRIGGER IF EXISTS set_community_vote_time ON community_votes;
DROP TRIGGER IF EXISTS check_proposal_community_vote ON community_votes;
DROP TRIGGER IF EXISTS set_distribution_time ON reward_distributions;
DROP TRIGGER IF EXISTS check_proposal_reward ON reward_distributions;

-- DROP TRIGGER FUNCTIONS
DROP FUNCTION IF EXISTS update_registry_time();
DROP FUNCTION IF EXISTS update_delegate_time();
DROP FUNCTION IF EXISTS update_nominee_time();
DROP FUNCTION IF EXISTS update_proposal_time();
DROP FUNCTION IF EXISTS update_rating_time();
DROP FUNCTION IF EXISTS update_delegate_vote_time();
DROP FUNCTION IF EXISTS validate_proposal_delegate_vote();
DROP FUNCTION IF EXISTS update_community_vote_time();
DROP FUNCTION IF EXISTS validate_proposal_community_vote();
DROP FUNCTION IF EXISTS update_distribution_time();
DROP FUNCTION IF EXISTS validate_proposal_reward();

-- DROP ACTIVITY TABLES FIRST (due to dependencies)
DROP TABLE IF EXISTS reward_distributions;
DROP TABLE IF EXISTS community_votes;
DROP TABLE IF EXISTS delegate_votes;
DROP TABLE IF EXISTS delegate_ratings;

-- DROP ENTITY TABLES
DROP TABLE IF EXISTS proposals;
DROP TABLE IF EXISTS nominated_delegates;
DROP TABLE IF EXISTS delegates;
DROP TABLE IF EXISTS governance_registries; 