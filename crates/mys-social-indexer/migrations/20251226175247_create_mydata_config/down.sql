-- Migration: Drop mydata_config table
-- Version: 20251226175247

DROP TRIGGER IF EXISTS set_mydata_config_time ON mydata_config;
DROP FUNCTION IF EXISTS update_mydata_config_time();
DROP TABLE IF EXISTS mydata_config CASCADE;

