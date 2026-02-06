-- Reverse migration: Restore binary YES/NO system

-- Step 1: Drop spot_bet_withdrawals table
DROP TABLE IF EXISTS spot_bet_withdrawals CASCADE;

-- Step 2: Add is_yes column back to social_proof_of_truth
ALTER TABLE social_proof_of_truth ADD COLUMN IF NOT EXISTS is_yes BOOLEAN;

-- Step 3: Migrate data back: option_id = 0 -> is_yes = true, option_id = 1 -> is_yes = false
UPDATE social_proof_of_truth 
SET is_yes = CASE 
    WHEN option_id = 0 THEN true 
    WHEN option_id = 1 THEN false 
    ELSE NULL 
END 
WHERE is_yes IS NULL;

-- Step 4: Drop option_id column from social_proof_of_truth
ALTER TABLE social_proof_of_truth DROP COLUMN IF EXISTS option_id;

-- Step 5: Add old escrow columns back to spot_records
ALTER TABLE spot_records ADD COLUMN IF NOT EXISTS total_yes_escrow BIGINT NOT NULL DEFAULT 0;
ALTER TABLE spot_records ADD COLUMN IF NOT EXISTS total_no_escrow BIGINT NOT NULL DEFAULT 0;

-- Step 6: Migrate data back from option_escrow JSONB
UPDATE spot_records 
SET 
    total_yes_escrow = COALESCE((option_escrow->>'0')::bigint, 0),
    total_no_escrow = COALESCE((option_escrow->>'1')::bigint, 0)
WHERE total_yes_escrow = 0 AND total_no_escrow = 0;

-- Step 7: Drop new columns from spot_records
ALTER TABLE spot_records DROP COLUMN IF EXISTS betting_options;
ALTER TABLE spot_records DROP COLUMN IF EXISTS option_escrow;
ALTER TABLE spot_records DROP COLUMN IF EXISTS resolution_window_epochs;
ALTER TABLE spot_records DROP COLUMN IF EXISTS max_resolution_window_epochs;

-- Step 8: Add is_yes column back to spot_bets
ALTER TABLE spot_bets ADD COLUMN IF NOT EXISTS is_yes BOOLEAN;

-- Step 9: Migrate data back: option_id = 0 -> is_yes = true, option_id = 1 -> is_yes = false
UPDATE spot_bets 
SET is_yes = CASE 
    WHEN option_id = 0 THEN true 
    WHEN option_id = 1 THEN false 
    ELSE NULL 
END 
WHERE is_yes IS NULL;

-- Step 10: Make is_yes NOT NULL
ALTER TABLE spot_bets ALTER COLUMN is_yes SET NOT NULL;

-- Step 11: Drop option_id column from spot_bets
ALTER TABLE spot_bets DROP COLUMN IF EXISTS option_id;

