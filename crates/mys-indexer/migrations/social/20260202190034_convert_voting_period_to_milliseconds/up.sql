-- Convert voting_period_epochs to voting_period_ms in governance_registries table
-- This migration renames the column and converts existing epoch values to milliseconds
-- Assumes 1 epoch = 1 day = 86400000 milliseconds
-- Adjust multiplier if your network has different epoch duration

-- Rename column
ALTER TABLE governance_registries 
RENAME COLUMN voting_period_epochs TO voting_period_ms;

-- Convert existing epoch values to milliseconds
-- Only convert if value looks like an epoch (less than 1000, which would be unreasonable for milliseconds)
-- This prevents double-conversion if migration is run multiple times
UPDATE governance_registries 
SET voting_period_ms = voting_period_ms * 86400000 
WHERE voting_period_ms < 1000 AND voting_period_ms > 0;

-- Note: voting_start_time and voting_end_time in proposals table are already stored as i64
-- They will now correctly represent milliseconds instead of epochs after Move contract update
