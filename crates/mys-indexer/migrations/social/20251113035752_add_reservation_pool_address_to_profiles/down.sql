-- Remove index
DROP INDEX IF EXISTS idx_profiles_reservation_pool_address;

-- Remove reservation_pool_address column from profiles table
ALTER TABLE profiles
    DROP COLUMN IF EXISTS reservation_pool_address;

