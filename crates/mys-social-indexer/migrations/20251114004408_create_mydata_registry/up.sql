-- Migration: Create mydata_registry table
-- Version: 20251114004408
-- Purpose: Create table to track MyData IP ID to owner mappings for registry events

-- ============================================================================
-- CREATE MYDATA REGISTRY TABLE
-- ============================================================================

-- MyData Registry table - tracks IP ID to owner mappings
-- This is a regular table (not hypertable) since it's a registry mapping
CREATE TABLE IF NOT EXISTS mydata_registry (
    ip_id TEXT NOT NULL PRIMARY KEY,
    owner TEXT NOT NULL,
    registered_at BIGINT NOT NULL,
    unregistered_at BIGINT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- Create indexes for efficient lookups
CREATE INDEX IF NOT EXISTS idx_mydata_registry_owner ON mydata_registry (owner);
CREATE INDEX IF NOT EXISTS idx_mydata_registry_active ON mydata_registry (is_active) WHERE is_active = true;
CREATE INDEX IF NOT EXISTS idx_mydata_registry_registered_at ON mydata_registry (registered_at DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_registry_transaction_id ON mydata_registry (transaction_id);

-- ============================================================================
-- ADD TABLE COMMENTS
-- ============================================================================

COMMENT ON TABLE mydata_registry IS 'Tracks MyData IP ID to owner mappings. Records registration and unregistration events from the blockchain.';
COMMENT ON COLUMN mydata_registry.ip_id IS 'The IP ID (address) of the MyData object';
COMMENT ON COLUMN mydata_registry.owner IS 'The owner address of the MyData';
COMMENT ON COLUMN mydata_registry.registered_at IS 'Unix timestamp in milliseconds when the MyData was registered';
COMMENT ON COLUMN mydata_registry.unregistered_at IS 'Unix timestamp in milliseconds when the MyData was unregistered (NULL if still active)';
COMMENT ON COLUMN mydata_registry.is_active IS 'Whether the MyData is currently registered (true) or unregistered (false)';
COMMENT ON COLUMN mydata_registry.time IS 'PostgreSQL timestamp for the record (derived from registered_at)';
COMMENT ON COLUMN mydata_registry.transaction_id IS 'Transaction ID from the blockchain event';

