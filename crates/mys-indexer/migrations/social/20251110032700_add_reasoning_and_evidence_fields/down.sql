DROP INDEX IF EXISTS idx_spot_resolutions_reasoning;
DROP INDEX IF EXISTS idx_poc_analysis_reasoning;
DROP INDEX IF EXISTS idx_platform_events_reasoning;

ALTER TABLE spot_resolutions 
DROP COLUMN IF EXISTS reasoning,
DROP COLUMN IF EXISTS evidence_urls;

ALTER TABLE poc_analysis_results 
DROP COLUMN IF EXISTS reasoning,
DROP COLUMN IF EXISTS evidence_urls;

ALTER TABLE platform_events 
DROP COLUMN IF EXISTS reasoning;

