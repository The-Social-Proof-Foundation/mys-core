-- Migration: Remove post prediction markets functionality
-- Version: 20251229020150
-- Purpose: Remove post prediction markets tables and columns

-- ============================================================================
-- 1. DROP POST PREDICTION CONFIG TABLE
-- ============================================================================

-- Drop the view first
DROP VIEW IF EXISTS latest_post_prediction_config CASCADE;

-- Drop the trigger
DROP TRIGGER IF EXISTS set_post_prediction_config_time ON post_prediction_config;

-- Drop the function
DROP FUNCTION IF EXISTS update_post_prediction_config_time() CASCADE;

-- Drop the table
DROP TABLE IF EXISTS post_prediction_config CASCADE;

-- ============================================================================
-- 2. REMOVE PREDICTION COLUMNS FROM POST_CONFIG TABLE
-- ============================================================================

-- Remove prediction-related columns from post_config table
ALTER TABLE post_config DROP COLUMN IF EXISTS predictions_enabled;
ALTER TABLE post_config DROP COLUMN IF EXISTS prediction_fee_bps;
ALTER TABLE post_config DROP COLUMN IF EXISTS prediction_treasury;
ALTER TABLE post_config DROP COLUMN IF EXISTS max_prediction_options;

