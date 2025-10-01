-- Migration Down: Remove min_offer_amount from Profiles
-- Version: 20250330000002

-- ============================================================================
-- 1. REMOVE INDEXES
-- ============================================================================

-- Drop indexes created for min_offer_amount
DROP INDEX IF EXISTS idx_profiles_min_offer_amount;
DROP INDEX IF EXISTS idx_profiles_owner_min_offer;

-- ============================================================================
-- 2. REMOVE MIN_OFFER_AMOUNT FIELD
-- ============================================================================

-- Drop the min_offer_amount column
ALTER TABLE profiles DROP COLUMN IF EXISTS min_offer_amount; 