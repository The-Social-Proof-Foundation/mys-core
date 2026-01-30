-- Migration: Drop mydata_registry table
-- Version: 20251114004408
-- Purpose: Rollback creation of mydata_registry table

-- Drop indexes first
DROP INDEX IF EXISTS idx_mydata_registry_transaction_id;
DROP INDEX IF EXISTS idx_mydata_registry_registered_at;
DROP INDEX IF EXISTS idx_mydata_registry_active;
DROP INDEX IF EXISTS idx_mydata_registry_owner;

-- Drop the table
DROP TABLE IF EXISTS mydata_registry;

