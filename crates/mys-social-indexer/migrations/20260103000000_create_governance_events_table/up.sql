-- Create governance_events table for tracking governance events
CREATE TABLE IF NOT EXISTS governance_events (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR NOT NULL,
    registry_type SMALLINT NOT NULL,
    event_data JSONB NOT NULL,
    event_id VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    anonymous_voting_related BOOLEAN
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_governance_events_registry_type ON governance_events(registry_type);
CREATE INDEX IF NOT EXISTS idx_governance_events_event_type ON governance_events(event_type);
CREATE INDEX IF NOT EXISTS idx_governance_events_created_at ON governance_events(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_governance_events_event_id ON governance_events(event_id);
CREATE INDEX IF NOT EXISTS idx_governance_events_anonymous ON governance_events(anonymous_voting_related, created_at DESC) 
    WHERE anonymous_voting_related IS NOT NULL;

