-- Migration: Remove active_badge_id from Profiles
-- Version: 20251111031105
-- Purpose: Rollback - Remove active_badge_id field from profiles table

-- ============================================================================
-- 1. REMOVE INDEX
-- ============================================================================

DROP INDEX IF EXISTS idx_profiles_active_badge_id;

-- ============================================================================
-- 2. REMOVE COLUMN
-- ============================================================================

ALTER TABLE profiles DROP COLUMN IF EXISTS active_badge_id;
