-- Migration: Drop insurance module tables
-- Version: 20251231000000

-- Drop triggers and functions
DROP TRIGGER IF EXISTS set_insurance_config_time ON insurance_config;
DROP FUNCTION IF EXISTS update_insurance_config_time();

DROP TRIGGER IF EXISTS set_insurance_vault_transactions_time ON insurance_vault_transactions;
DROP FUNCTION IF EXISTS update_insurance_vault_transactions_time();

DROP TRIGGER IF EXISTS set_insurance_policy_events_time ON insurance_policy_events;
DROP FUNCTION IF EXISTS update_insurance_policy_events_time();

DROP TRIGGER IF EXISTS set_insurance_market_exposures_time ON insurance_market_exposures;
DROP FUNCTION IF EXISTS update_insurance_market_exposures_time();

DROP TRIGGER IF EXISTS set_insurance_user_exposures_time ON insurance_user_exposures;
DROP FUNCTION IF EXISTS update_insurance_user_exposures_time();

-- Drop tables (order matters - drop hypertables first, then regular tables)
DROP TABLE IF EXISTS insurance_user_exposures CASCADE;
DROP TABLE IF EXISTS insurance_market_exposures CASCADE;
DROP TABLE IF EXISTS insurance_policy_events CASCADE;
DROP TABLE IF EXISTS insurance_vault_transactions CASCADE;
DROP TABLE IF EXISTS insurance_events CASCADE;
DROP TABLE IF EXISTS insurance_policies CASCADE;
DROP TABLE IF EXISTS insurance_vaults CASCADE;
DROP TABLE IF EXISTS insurance_config CASCADE;

