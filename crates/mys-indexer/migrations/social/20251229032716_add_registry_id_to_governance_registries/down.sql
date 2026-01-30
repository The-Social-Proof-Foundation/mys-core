-- Reverse migration: Remove registry_id column from governance_registries table

-- ============================================================================
-- 1. DROP INDEX
-- ============================================================================

-- Drop the index on registry_id
DROP INDEX IF EXISTS idx_governance_registries_registry_id;

-- ============================================================================
-- 2. REMOVE registry_id COLUMN
-- ============================================================================

-- Remove registry_id column from governance_registries table
ALTER TABLE governance_registries DROP COLUMN IF EXISTS registry_id;

