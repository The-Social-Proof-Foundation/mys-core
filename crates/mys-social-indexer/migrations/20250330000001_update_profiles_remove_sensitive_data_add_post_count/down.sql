-- Migration Down: Rollback Profiles Changes - Restore sensitive_data_updated_at and Remove post_count
-- Version: 20250330000001

-- ============================================================================
-- 1. REMOVE POST_COUNT FIELD AND RELATED OBJECTS
-- ============================================================================

-- Drop the constraint first
ALTER TABLE profiles DROP CONSTRAINT IF EXISTS chk_profiles_post_count_non_negative;

-- Drop indexes
DROP INDEX IF EXISTS idx_profiles_post_count;
DROP INDEX IF EXISTS idx_profiles_owner_post_count;

-- Drop the post_count column
ALTER TABLE profiles DROP COLUMN IF EXISTS post_count;

-- ============================================================================
-- 2. RESTORE SENSITIVE_DATA_UPDATED_AT FIELD
-- ============================================================================

-- Add back the sensitive_data_updated_at column
ALTER TABLE profiles ADD COLUMN sensitive_data_updated_at TIMESTAMP NULL; 