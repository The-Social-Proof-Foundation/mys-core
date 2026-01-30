-- Drop trigger and function
DROP TRIGGER IF EXISTS migrate_counts_on_profile_transfer ON profiles;
DROP FUNCTION IF EXISTS migrate_profile_counts_to_wallet();
