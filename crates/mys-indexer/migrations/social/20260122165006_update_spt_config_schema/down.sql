-- Revert spt_exchange_config schema changes
-- This migration removes the separate trading and reservation fee columns
-- Note: Diesel migrations run in transactions automatically, so BEGIN/COMMIT are not needed

-- ============================================================================
-- 1. REMOVE NEW COLUMNS
-- ============================================================================

ALTER TABLE spt_exchange_config 
DROP COLUMN IF EXISTS trading_creator_fee_bps,
DROP COLUMN IF EXISTS trading_platform_fee_bps,
DROP COLUMN IF EXISTS trading_treasury_fee_bps,
DROP COLUMN IF EXISTS reservation_creator_fee_bps,
DROP COLUMN IF EXISTS reservation_platform_fee_bps,
DROP COLUMN IF EXISTS reservation_treasury_fee_bps,
DROP COLUMN IF EXISTS max_reservers_per_pool;
