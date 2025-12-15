-- Migration: Drop post_config table
-- Version: 20251215065555

DROP TRIGGER IF EXISTS set_post_config_time ON post_config;
DROP FUNCTION IF EXISTS update_post_config_time();
DROP TABLE IF EXISTS post_config CASCADE;

