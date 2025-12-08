-- Add reasoning and evidence_urls fields to spot_resolutions table
ALTER TABLE spot_resolutions 
ADD COLUMN IF NOT EXISTS reasoning TEXT NOT NULL DEFAULT '',
ADD COLUMN IF NOT EXISTS evidence_urls JSONB NOT NULL DEFAULT '[]'::jsonb;

-- Remove defaults after adding columns (for future inserts)
ALTER TABLE spot_resolutions 
ALTER COLUMN reasoning DROP DEFAULT,
ALTER COLUMN evidence_urls DROP DEFAULT;

-- Add reasoning and evidence_urls fields to poc_analysis_results table
ALTER TABLE poc_analysis_results 
ADD COLUMN IF NOT EXISTS reasoning TEXT NULL,
ADD COLUMN IF NOT EXISTS evidence_urls JSONB NULL;

-- Add reasoning field to platform_events table
ALTER TABLE platform_events 
ADD COLUMN IF NOT EXISTS reasoning TEXT NULL;

-- Create indexes for querying
CREATE INDEX IF NOT EXISTS idx_spot_resolutions_reasoning ON spot_resolutions USING gin(to_tsvector('english', reasoning)) WHERE reasoning != '';
CREATE INDEX IF NOT EXISTS idx_poc_analysis_reasoning ON poc_analysis_results USING gin(to_tsvector('english', reasoning)) WHERE reasoning IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_platform_events_reasoning ON platform_events USING gin(to_tsvector('english', reasoning)) WHERE reasoning IS NOT NULL;

