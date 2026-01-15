-- Migration: Revert insurance_config schema changes
-- Version: 20260114225121
-- Purpose: Revert insurance_config table to previous structure

-- ============================================================================
-- 1. ADD BACK TREASURY COLUMN
-- ============================================================================

-- Add back treasury column
ALTER TABLE insurance_config 
ADD COLUMN IF NOT EXISTS treasury TEXT NOT NULL DEFAULT '';

-- ============================================================================
-- 2. RENAME COLUMN BACK AND INVERT VALUES
-- ============================================================================

-- Add back paused column
ALTER TABLE insurance_config 
ADD COLUMN IF NOT EXISTS paused BOOLEAN NOT NULL DEFAULT FALSE;

-- Migrate data back: invert enable_flag values (enable_flag=true means paused=false)
UPDATE insurance_config 
SET paused = NOT enable_flag;

-- Drop the enable_flag column
ALTER TABLE insurance_config DROP COLUMN IF EXISTS enable_flag;
