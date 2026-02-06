-- Revert voting_period_ms back to voting_period_epochs
-- Convert milliseconds back to epochs (assuming 1 epoch = 1 day = 86400000 ms)

-- Convert milliseconds back to epochs
UPDATE governance_registries 
SET voting_period_ms = voting_period_ms / 86400000 
WHERE voting_period_ms >= 86400000;

-- Rename column back
ALTER TABLE governance_registries 
RENAME COLUMN voting_period_ms TO voting_period_epochs;
