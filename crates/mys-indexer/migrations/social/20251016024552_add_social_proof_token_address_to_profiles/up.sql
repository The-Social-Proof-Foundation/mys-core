-- Add social_proof_token_address column to profiles table
ALTER TABLE profiles
    ADD COLUMN IF NOT EXISTS social_proof_token_address VARCHAR;

-- Add index for faster lookups by social proof token address
CREATE INDEX IF NOT EXISTS idx_profiles_social_proof_token_address 
    ON profiles(social_proof_token_address) 
    WHERE social_proof_token_address IS NOT NULL;
