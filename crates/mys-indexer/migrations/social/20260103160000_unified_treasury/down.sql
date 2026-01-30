-- Revert unified treasury migration
-- This restores the ecosystem_treasury column to spt_exchange_config and removes the separate table

-- Re-add ecosystem_treasury column to spt_exchange_config table
ALTER TABLE spt_exchange_config ADD COLUMN IF NOT EXISTS ecosystem_treasury TEXT;

-- If ecosystem_treasury table has data, populate the latest treasury address into spt_exchange_config records
-- Update all existing config records with the most recent treasury address
UPDATE spt_exchange_config
SET ecosystem_treasury = (
    SELECT treasury_address
    FROM ecosystem_treasury
    ORDER BY time DESC
    LIMIT 1
)
WHERE ecosystem_treasury IS NULL
AND EXISTS (SELECT 1 FROM ecosystem_treasury);

-- Set a default value for any remaining NULL values (fallback)
UPDATE spt_exchange_config
SET ecosystem_treasury = ''
WHERE ecosystem_treasury IS NULL;

-- Make the column NOT NULL after populating values
ALTER TABLE spt_exchange_config ALTER COLUMN ecosystem_treasury SET NOT NULL;

-- Drop the ecosystem_treasury table (this will also drop the hypertable)
DROP TABLE IF EXISTS ecosystem_treasury CASCADE;

