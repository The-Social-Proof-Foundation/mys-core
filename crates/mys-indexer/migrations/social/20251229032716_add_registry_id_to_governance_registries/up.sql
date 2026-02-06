-- Migration: Add registry_id column to governance_registries table
-- Version: 20251229032716
-- Purpose: Add registry_id field to store the Object ID of governance registries

-- ============================================================================
-- 1. ADD registry_id COLUMN
-- ============================================================================

-- Add registry_id column to governance_registries table
ALTER TABLE governance_registries ADD COLUMN registry_id VARCHAR NOT NULL;

-- ============================================================================
-- 2. CREATE INDEX ON registry_id
-- ============================================================================

-- Create index on registry_id for efficient lookups
CREATE INDEX idx_governance_registries_registry_id ON governance_registries(registry_id);

