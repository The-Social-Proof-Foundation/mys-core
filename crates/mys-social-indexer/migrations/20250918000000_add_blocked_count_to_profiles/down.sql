-- Remove blocked_count column from profiles
ALTER TABLE profiles
    DROP COLUMN IF EXISTS blocked_count;

