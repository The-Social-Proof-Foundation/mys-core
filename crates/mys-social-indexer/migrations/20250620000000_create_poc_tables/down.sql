-- ROLLBACK PROOF OF CREATIVITY (POC) SYSTEM
-- This migration removes all PoC-related tables and components

-- ============================================================================
-- 1. DROP CONTINUOUS AGGREGATES
-- ============================================================================
DROP MATERIALIZED VIEW IF EXISTS poc_hourly_stats CASCADE;
DROP MATERIALIZED VIEW IF EXISTS poc_daily_stats CASCADE;

-- ============================================================================
-- 2. DROP POC TABLES (IN REVERSE ORDER)
-- ============================================================================
DROP TABLE IF EXISTS poc_configuration CASCADE;
DROP TABLE IF EXISTS poc_dispute_votes CASCADE;
DROP TABLE IF EXISTS poc_disputes CASCADE;
DROP TABLE IF EXISTS poc_analysis_results CASCADE;
DROP TABLE IF EXISTS poc_revenue_redirections CASCADE;
DROP TABLE IF EXISTS poc_badges CASCADE;

-- ============================================================================
-- 3. DROP TRIGGERS AND FUNCTIONS
-- ============================================================================
DROP FUNCTION IF EXISTS validate_poc_dispute_reference() CASCADE;
DROP FUNCTION IF EXISTS validate_poc_original_post_reference() CASCADE;
DROP FUNCTION IF EXISTS validate_poc_post_reference() CASCADE;
DROP FUNCTION IF EXISTS update_poc_vote_time() CASCADE;
DROP FUNCTION IF EXISTS update_poc_dispute_time() CASCADE;
DROP FUNCTION IF EXISTS update_poc_analysis_time() CASCADE;
DROP FUNCTION IF EXISTS update_poc_redirection_time() CASCADE;
DROP FUNCTION IF EXISTS update_poc_badge_time() CASCADE;

-- ============================================================================
-- 4. REMOVE POC FIELDS FROM POSTS TABLE
-- ============================================================================
DO $$
BEGIN
    -- Remove poc_badge_id column if it exists
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'poc_badge_id'
    ) THEN
        ALTER TABLE posts DROP COLUMN poc_badge_id;
    END IF;
    
    -- Remove revenue_redirect_to column if it exists
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'revenue_redirect_to'
    ) THEN
        ALTER TABLE posts DROP COLUMN revenue_redirect_to;
    END IF;
    
    -- Remove revenue_redirect_percentage column if it exists
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'revenue_redirect_percentage'
    ) THEN
        ALTER TABLE posts DROP COLUMN revenue_redirect_percentage;
    END IF;
END
$$; 