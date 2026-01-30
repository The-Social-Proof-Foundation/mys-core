-- Migration: Drop spot_config table
-- Version: 20251115000000

DROP TRIGGER IF EXISTS set_spot_config_time ON spot_config;
DROP FUNCTION IF EXISTS update_spot_config_time();
DROP TABLE IF EXISTS spot_config;

