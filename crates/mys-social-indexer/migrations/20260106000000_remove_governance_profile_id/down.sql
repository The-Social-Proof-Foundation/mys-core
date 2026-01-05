-- Revert: Add profile_id column back (if needed)
-- Note: This will set profile_id to NULL for existing rows

-- Drop the view first
DROP VIEW IF EXISTS delegate_performance CASCADE;

ALTER TABLE delegates 
    ADD COLUMN IF NOT EXISTS profile_id TEXT;

ALTER TABLE nominated_delegates
    ADD COLUMN IF NOT EXISTS profile_id TEXT;

-- Recreate indexes if they existed
CREATE INDEX IF NOT EXISTS idx_delegates_profile_id ON delegates(profile_id, time);
CREATE INDEX IF NOT EXISTS idx_nominees_profile_id ON nominated_delegates(profile_id, time);

-- Recreate delegate_performance view with profile_id
CREATE OR REPLACE VIEW delegate_performance AS
SELECT
    d.address,
    d.profile_id,
    d.registry_type,
    d.upvotes,
    d.downvotes,
    d.proposals_reviewed,
    d.proposals_submitted,
    d.sided_winning_proposals,
    d.sided_losing_proposals,
    d.term_start,
    d.term_end,
    d.is_active,
    CASE 
        WHEN d.proposals_reviewed > 0 THEN 
            d.sided_winning_proposals::FLOAT / NULLIF(d.proposals_reviewed, 0) 
        ELSE NULL 
    END AS winning_rate,
    COUNT(DISTINCT dv.proposal_id) AS recent_votes,
    SUM(CASE WHEN dv.approve THEN 1 ELSE 0 END) AS recent_approvals,
    SUM(CASE WHEN NOT dv.approve THEN 1 ELSE 0 END) AS recent_rejections
FROM
    delegates d
LEFT JOIN
    delegate_votes dv ON d.address = dv.delegate_address 
                      AND dv.time > NOW() - INTERVAL '30 days'
GROUP BY
    d.id, d.address, d.profile_id, d.registry_type, d.upvotes, d.downvotes, 
    d.proposals_reviewed, d.proposals_submitted, d.sided_winning_proposals, 
    d.sided_losing_proposals, d.term_start, d.term_end, d.is_active;

