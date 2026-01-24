-- Migration: Remove version column from post_config table
-- Version: 20260124010622

-- Remove version column from post_config table
ALTER TABLE post_config DROP COLUMN IF EXISTS version;
