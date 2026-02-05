-- Update spt_exchange_config schema to support separate trading and reservation fees
-- This migration adds new fields matching the updated Move contract structure
-- Note: Diesel migrations run in transactions automatically, so BEGIN/COMMIT are not needed

-- ============================================================================
-- 1. ADD NEW COLUMNS FOR SEPARATE TRADING AND RESERVATION FEES
-- ============================================================================

-- Add trading fee columns
ALTER TABLE spt_exchange_config 
ADD COLUMN IF NOT EXISTS trading_creator_fee_bps BIGINT,
ADD COLUMN IF NOT EXISTS trading_platform_fee_bps BIGINT,
ADD COLUMN IF NOT EXISTS trading_treasury_fee_bps BIGINT;

-- Add reservation fee columns
ALTER TABLE spt_exchange_config 
ADD COLUMN IF NOT EXISTS reservation_creator_fee_bps BIGINT,
ADD COLUMN IF NOT EXISTS reservation_platform_fee_bps BIGINT,
ADD COLUMN IF NOT EXISTS reservation_treasury_fee_bps BIGINT;

-- Add max_reservers_per_pool column
ALTER TABLE spt_exchange_config 
ADD COLUMN IF NOT EXISTS max_reservers_per_pool BIGINT;

-- ============================================================================
-- 2. BACKFILL EXISTING RECORDS WITH VALUES FROM OLD FIELDS
-- ============================================================================

-- For existing records, populate new trading fees from old unified fees
-- This maintains backward compatibility
UPDATE spt_exchange_config 
SET 
    trading_creator_fee_bps = creator_fee_bps,
    trading_platform_fee_bps = platform_fee_bps,
    trading_treasury_fee_bps = treasury_fee_bps,
    reservation_creator_fee_bps = creator_fee_bps,
    reservation_platform_fee_bps = platform_fee_bps,
    reservation_treasury_fee_bps = treasury_fee_bps,
    max_reservers_per_pool = 1000  -- Default value from Move contract
WHERE trading_creator_fee_bps IS NULL;

-- ============================================================================
-- 3. SET NOT NULL CONSTRAINTS AFTER BACKFILL
-- ============================================================================

-- Make new columns NOT NULL after backfill
ALTER TABLE spt_exchange_config 
ALTER COLUMN trading_creator_fee_bps SET NOT NULL,
ALTER COLUMN trading_platform_fee_bps SET NOT NULL,
ALTER COLUMN trading_treasury_fee_bps SET NOT NULL,
ALTER COLUMN reservation_creator_fee_bps SET NOT NULL,
ALTER COLUMN reservation_platform_fee_bps SET NOT NULL,
ALTER COLUMN reservation_treasury_fee_bps SET NOT NULL,
ALTER COLUMN max_reservers_per_pool SET NOT NULL;

-- Set default values for future inserts
ALTER TABLE spt_exchange_config 
ALTER COLUMN trading_creator_fee_bps SET DEFAULT 100,  -- 1.0% default from Move contract
ALTER COLUMN trading_platform_fee_bps SET DEFAULT 25,   -- 0.25% default
ALTER COLUMN trading_treasury_fee_bps SET DEFAULT 25,    -- 0.25% default
ALTER COLUMN reservation_creator_fee_bps SET DEFAULT 100, -- 1.0% default
ALTER COLUMN reservation_platform_fee_bps SET DEFAULT 25, -- 0.25% default
ALTER COLUMN reservation_treasury_fee_bps SET DEFAULT 25, -- 0.25% default
ALTER COLUMN max_reservers_per_pool SET DEFAULT 1000;     -- Default from Move contract
