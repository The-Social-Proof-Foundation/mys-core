-- Remove social_proof_token_address column from profiles table
DROP INDEX IF EXISTS idx_profiles_social_proof_token_address;
ALTER TABLE profiles DROP COLUMN IF EXISTS social_proof_token_address;
