-- Remove all backfilled data from wallet_social_graph
-- Note: This will remove all entries, not just backfilled ones
-- In practice, you may want to be more selective, but for rollback purposes this is safe
-- as the table will be repopulated by triggers going forward
DELETE FROM wallet_social_graph;
