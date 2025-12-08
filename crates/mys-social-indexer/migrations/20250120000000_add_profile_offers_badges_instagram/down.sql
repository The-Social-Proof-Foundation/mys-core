-- Drop profile badges table
DROP TABLE IF EXISTS profile_badges CASCADE;

-- Drop profile sale fees table
DROP TABLE IF EXISTS profile_sale_fees CASCADE;

-- Drop profile offers table
DROP TABLE IF EXISTS profile_offers CASCADE;

-- Remove instagram_username column from profiles
ALTER TABLE profiles DROP COLUMN IF EXISTS instagram_username;

