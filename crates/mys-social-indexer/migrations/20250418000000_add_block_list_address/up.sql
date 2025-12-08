ALTER TABLE profiles
ADD COLUMN IF NOT EXISTS block_list_address TEXT;

-- Add index for faster lookups
CREATE INDEX IF NOT EXISTS idx_profiles_block_list_address ON profiles (block_list_address);

-- Comment for the new column
COMMENT ON COLUMN profiles.block_list_address IS 'The Blockchain address of the profile''s BlockList object';