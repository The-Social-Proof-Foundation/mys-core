-- Drop triggers and function
DROP TRIGGER IF EXISTS update_blocked_count_on_block ON blocked_profiles;
DROP TRIGGER IF EXISTS update_blocked_count_on_unblock ON blocked_profiles;
DROP FUNCTION IF EXISTS update_blocked_count();
