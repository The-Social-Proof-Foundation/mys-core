-- Remove profile_id column from governance tables
-- Address is the primary identifier for governance, not profile_id

-- Drop indexes that reference profile_id first
DROP INDEX IF EXISTS idx_delegates_profile_id;
DROP INDEX IF EXISTS idx_nominees_profile_id;

-- Drop profile_id column from delegates table
ALTER TABLE delegates 
    DROP COLUMN IF EXISTS profile_id;

-- Drop profile_id column from nominated_delegates table  
ALTER TABLE nominated_delegates
    DROP COLUMN IF EXISTS profile_id;

