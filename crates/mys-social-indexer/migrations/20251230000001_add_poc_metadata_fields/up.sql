-- Migration: Add PoC metadata fields to posts table
-- Version: 20251230000001
-- Purpose: Add PoC analysis metadata fields to replace poc_id with detailed analysis information

-- ============================================================================
-- 1. ADD POC METADATA COLUMNS
-- ============================================================================

-- Add PoC metadata fields (replaces poc_id with detailed analysis data)
ALTER TABLE posts ADD COLUMN poc_reasoning TEXT NULL;
ALTER TABLE posts ADD COLUMN poc_evidence_urls JSONB NULL;
ALTER TABLE posts ADD COLUMN poc_similarity_score BIGINT NULL;
ALTER TABLE posts ADD COLUMN poc_media_type SMALLINT NULL;
ALTER TABLE posts ADD COLUMN poc_oracle_address TEXT NULL;
ALTER TABLE posts ADD COLUMN poc_analyzed_at BIGINT NULL;

-- ============================================================================
-- 2. CREATE INDEXES FOR POC METADATA
-- ============================================================================

-- Index for filtering by similarity score
CREATE INDEX idx_posts_poc_similarity_score ON posts(poc_similarity_score, time) WHERE poc_similarity_score IS NOT NULL;

-- Index for filtering by media type
CREATE INDEX idx_posts_poc_media_type ON posts(poc_media_type, time) WHERE poc_media_type IS NOT NULL;

-- Index for filtering by oracle address
CREATE INDEX idx_posts_poc_oracle_address ON posts(poc_oracle_address, time) WHERE poc_oracle_address IS NOT NULL;

-- Index for filtering by analysis timestamp
CREATE INDEX idx_posts_poc_analyzed_at ON posts(poc_analyzed_at, time) WHERE poc_analyzed_at IS NOT NULL;

-- Composite index for common PoC queries
CREATE INDEX idx_posts_poc_analysis ON posts(poc_media_type, poc_similarity_score, poc_analyzed_at, time) 
WHERE poc_media_type IS NOT NULL AND poc_similarity_score IS NOT NULL;

-- ============================================================================
-- 3. UPDATE DOCUMENTATION
-- ============================================================================

COMMENT ON COLUMN posts.poc_reasoning IS 'Oracle analysis reasoning for PoC verification';
COMMENT ON COLUMN posts.poc_evidence_urls IS 'URLs to evidence used in PoC analysis (JSON array)';
COMMENT ON COLUMN posts.poc_similarity_score IS 'Similarity score from PoC analysis (0-100 percentage)';
COMMENT ON COLUMN posts.poc_media_type IS 'Media type analyzed: 1=IMAGE, 2=VIDEO, 3=AUDIO';
COMMENT ON COLUMN posts.poc_oracle_address IS 'Address of oracle that performed PoC analysis';
COMMENT ON COLUMN posts.poc_analyzed_at IS 'Timestamp when PoC analysis was performed';
