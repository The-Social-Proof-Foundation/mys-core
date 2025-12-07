-- Migration: Add min_offer_amount to Profiles
-- Version: 20250330000002
-- Purpose: Add min_offer_amount field to track minimum MYSO token offer amounts for profile sales

-- ============================================================================
-- 1. ADD MIN_OFFER_AMOUNT FIELD
-- ============================================================================

-- Add min_offer_amount field to track minimum offer amounts for profile sales
-- NULL means no minimum offer amount is set (profile not explicitly for sale)
-- BIGINT to match u64 from Move contract (up to 18 quintillion tokens)
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS min_offer_amount BIGINT NULL;

-- ============================================================================
-- 2. INDEXING FOR PERFORMANCE
-- ============================================================================

-- Add index for querying profiles by min_offer_amount (for marketplace views)
CREATE INDEX IF NOT EXISTS idx_profiles_min_offer_amount ON profiles (min_offer_amount) WHERE min_offer_amount IS NOT NULL;

-- Add composite index for filtering profiles for sale by owner
CREATE INDEX IF NOT EXISTS idx_profiles_owner_min_offer ON profiles (owner_address, min_offer_amount) WHERE min_offer_amount IS NOT NULL;

-- ============================================================================
-- 3. DOCUMENTATION
-- ============================================================================

-- Comment the column for documentation
COMMENT ON COLUMN profiles.min_offer_amount IS 'Minimum MYSO token amount the profile owner will accept as an offer for profile sale. NULL means no minimum is set and profile is not explicitly for sale.'; 