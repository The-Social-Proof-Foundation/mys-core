-- Add blocked_count column to profiles with default 0 and backfill based on existing state
ALTER TABLE profiles
    ADD COLUMN IF NOT EXISTS blocked_count INTEGER NOT NULL DEFAULT 0;

-- Backfill accurate counts from current blocked_profiles state
UPDATE profiles p
SET blocked_count = COALESCE(sub.cnt, 0)
FROM (
    SELECT blocker_address, COUNT(*)::int AS cnt
    FROM blocked_profiles
    GROUP BY blocker_address
) AS sub
WHERE p.owner_address = sub.blocker_address;

