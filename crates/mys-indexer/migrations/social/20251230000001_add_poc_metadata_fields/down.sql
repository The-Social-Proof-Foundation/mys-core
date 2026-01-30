-- Migration: Remove PoC metadata fields from posts table
-- Version: 20251230000001
-- Purpose: Reverse the addition of PoC metadata fields

-- ============================================================================
-- 1. DROP INDEXES
-- ============================================================================

DROP INDEX idx_posts_poc_similarity_score;
DROP INDEX idx_posts_poc_media_type;
DROP INDEX idx_posts_poc_oracle_address;
DROP INDEX idx_posts_poc_analyzed_at;
DROP INDEX idx_posts_poc_analysis;

-- ============================================================================
-- 2. REMOVE COLUMNS
-- ============================================================================

ALTER TABLE posts DROP COLUMN poc_reasoning;
ALTER TABLE posts DROP COLUMN poc_evidence_urls;
ALTER TABLE posts DROP COLUMN poc_similarity_score;
ALTER TABLE posts DROP COLUMN poc_media_type;
ALTER TABLE posts DROP COLUMN poc_oracle_address;
ALTER TABLE posts DROP COLUMN poc_analyzed_at;
