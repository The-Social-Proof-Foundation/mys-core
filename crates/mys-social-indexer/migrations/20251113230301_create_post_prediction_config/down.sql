-- Migration: Drop post_prediction_config table
-- Version: 20251113230301
-- Purpose: Rollback - Remove post_prediction_config table and related objects

-- ============================================================================
-- 1. DROP VIEWS
-- ============================================================================

-- Drop the view first
DROP VIEW IF EXISTS latest_post_prediction_config CASCADE;

-- ============================================================================
-- 2. DROP TABLE
-- ============================================================================

-- Drop the trigger function
DROP FUNCTION IF EXISTS update_post_prediction_config_time() CASCADE;

-- Drop the table (will cascade to indexes and constraints)
DROP TABLE IF EXISTS post_prediction_config CASCADE;

