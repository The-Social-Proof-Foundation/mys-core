-- Drop trigger and function
DROP TRIGGER IF EXISTS migrate_counts_on_profile_create ON profiles;
DROP FUNCTION IF EXISTS migrate_wallet_counts_to_profile();
