-- Migration: Fix mydata revenue views and unified_revenue constraint
-- Version: 20251107015016
-- Purpose: Update unified_revenue CHECK constraint and ensure all views use 'mydata' instead of 'my_ip'

-- ============================================================================
-- 1. UPDATE UNIFIED_REVENUE CHECK CONSTRAINT
-- ============================================================================

-- Find and drop the old CHECK constraint (PostgreSQL auto-generates names)
DO $$
DECLARE
    constraint_name TEXT;
BEGIN
    -- Find the CHECK constraint on revenue_source column
    SELECT conname INTO constraint_name
    FROM pg_constraint
    WHERE conrelid = 'unified_revenue'::regclass
        AND contype = 'c'
        AND conname LIKE '%revenue_source%';
    
    -- Drop the constraint if found
    IF constraint_name IS NOT NULL THEN
        EXECUTE format('ALTER TABLE unified_revenue DROP CONSTRAINT %I', constraint_name);
    END IF;
END $$;

-- Add new CHECK constraint allowing 'mydata' instead of 'my_ip'
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'unified_revenue_revenue_source_check'
        AND conrelid = 'unified_revenue'::regclass
    ) THEN
        ALTER TABLE unified_revenue ADD CONSTRAINT unified_revenue_revenue_source_check 
            CHECK (revenue_source IN ('subscription', 'mydata', 'spt', 'tips', 'posts'));
    END IF;
END $$;

-- Update any existing 'my_ip' values to 'mydata'
UPDATE unified_revenue 
SET revenue_source = 'mydata' 
WHERE revenue_source = 'my_ip';

-- ============================================================================
-- 2. RECREATE ALL REVENUE VIEWS WITH CORRECT COLUMN NAMES
-- ============================================================================

-- Drop existing views that need to be updated
DROP VIEW IF EXISTS revenue_dashboard_24h CASCADE;
DROP VIEW IF EXISTS platform_revenue_summary CASCADE;
DROP VIEW IF EXISTS spt_creator_revenue_summary CASCADE;

-- Recreate revenue_dashboard_24h view (doesn't need mydata column, but ensure it's correct)
CREATE OR REPLACE VIEW revenue_dashboard_24h AS
SELECT 
    revenue_source,
    SUM(amount) AS total_revenue_24h,
    COUNT(*) AS total_transactions_24h,
    COUNT(DISTINCT creator_address) AS unique_creators_24h,
    COUNT(DISTINCT payer_address) AS unique_payers_24h,
    MAX(amount) AS largest_transaction_24h,
    AVG(amount) AS avg_transaction_amount
FROM unified_revenue
WHERE time >= NOW() - INTERVAL '24 hours'
GROUP BY revenue_source
ORDER BY total_revenue_24h DESC;

COMMENT ON VIEW revenue_dashboard_24h IS 'Real-time dashboard metrics using direct unified_revenue queries (24-hour summary)';

-- Recreate platform_revenue_summary view with mydata column
CREATE OR REPLACE VIEW platform_revenue_summary AS
SELECT 
    platform_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END) AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT creator_address) AS total_creators,
    COUNT(DISTINCT payer_address) AS total_payers,
    AVG(amount) AS avg_transaction_amount,
    COUNT(DISTINCT DATE_TRUNC('month', time)) AS active_months,
    DATE_TRUNC('month', MAX(time))::DATE AS last_active_month
FROM unified_revenue
WHERE platform_address IS NOT NULL
    AND time >= DATE_TRUNC('month', NOW() - INTERVAL '12 months')
GROUP BY platform_address
ORDER BY total_revenue DESC;

COMMENT ON VIEW platform_revenue_summary IS 'Platform revenue analytics using direct unified_revenue queries (12-month summary)';

-- Recreate spt_creator_revenue_summary view with mydata column
CREATE OR REPLACE VIEW spt_creator_revenue_summary AS
SELECT 
    creator_address,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN revenue_source = 'subscription' THEN amount ELSE 0 END) AS total_subscription_revenue,
    SUM(CASE WHEN revenue_source = 'mydata' THEN amount ELSE 0 END) AS total_mydata_revenue,
    SUM(CASE WHEN revenue_source = 'spt' THEN amount ELSE 0 END) AS total_spt_revenue,
    SUM(CASE WHEN revenue_source = 'tips' THEN amount ELSE 0 END) AS total_tips_revenue,
    COUNT(*) AS total_transactions,
    COUNT(DISTINCT payer_address) AS total_unique_payers,
    MAX(amount) AS largest_single_transaction,
    COUNT(DISTINCT DATE(time)) AS active_days,
    MAX(time) AS last_revenue_date
FROM unified_revenue
WHERE time >= NOW() - INTERVAL '30 days'
GROUP BY creator_address
ORDER BY total_revenue DESC;

COMMENT ON VIEW spt_creator_revenue_summary IS 'Creator revenue leaderboard using direct unified_revenue queries (30-day summary)';

