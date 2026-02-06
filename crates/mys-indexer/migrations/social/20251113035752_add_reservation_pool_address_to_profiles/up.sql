-- Add reservation_pool_address column to profiles table
ALTER TABLE profiles
    ADD COLUMN IF NOT EXISTS reservation_pool_address VARCHAR;

-- Add index for faster lookups by reservation pool address
CREATE INDEX IF NOT EXISTS idx_profiles_reservation_pool_address 
    ON profiles(reservation_pool_address) 
    WHERE reservation_pool_address IS NOT NULL;

