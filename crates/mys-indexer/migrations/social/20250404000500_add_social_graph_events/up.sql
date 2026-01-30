-- Create table to log social graph events
-- Note: follower_id and following_id columns may be dropped by a later migration (20250404000600_fix_social_graph_schema)
-- So we create them conditionally and make foreign keys conditional as well
CREATE TABLE IF NOT EXISTS social_graph_events (
    id SERIAL PRIMARY KEY,
    event_type TEXT NOT NULL, -- 'follow' or 'unfollow'
    follower_id INTEGER, -- May be dropped by later migration, so nullable initially
    following_id INTEGER, -- May be dropped by later migration, so nullable initially
    follower_address TEXT NOT NULL,
    following_address TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    blockchain_tx_hash TEXT, -- Optional blockchain transaction hash
    raw_event_data JSONB -- Raw event data from blockchain
);

-- Conditionally add follower_id column if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_events' 
        AND column_name = 'follower_id'
    ) THEN
        ALTER TABLE social_graph_events ADD COLUMN follower_id INTEGER;
    END IF;
END $$;

-- Conditionally add following_id column if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_events' 
        AND column_name = 'following_id'
    ) THEN
        ALTER TABLE social_graph_events ADD COLUMN following_id INTEGER;
    END IF;
END $$;

-- Conditionally add foreign key constraint for follower_id
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_events' 
        AND column_name = 'follower_id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' 
        AND column_name = 'id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'social_graph_events_follower_id_fkey'
        AND conrelid = 'social_graph_events'::regclass
    ) THEN
        ALTER TABLE social_graph_events 
        ADD CONSTRAINT social_graph_events_follower_id_fkey 
        FOREIGN KEY (follower_id) REFERENCES profiles(id) ON DELETE CASCADE;
    END IF;
END $$;

-- Conditionally add foreign key constraint for following_id
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_events' 
        AND column_name = 'following_id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' 
        AND column_name = 'id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'social_graph_events_following_id_fkey'
        AND conrelid = 'social_graph_events'::regclass
    ) THEN
        ALTER TABLE social_graph_events 
        ADD CONSTRAINT social_graph_events_following_id_fkey 
        FOREIGN KEY (following_id) REFERENCES profiles(id) ON DELETE CASCADE;
    END IF;
END $$;

-- Add indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_social_graph_events_event_type ON social_graph_events(event_type);
CREATE INDEX IF NOT EXISTS idx_social_graph_events_created_at ON social_graph_events(created_at);

-- Conditionally create indexes on follower_id and following_id only if columns exist
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_events' 
        AND column_name = 'follower_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE tablename = 'social_graph_events' 
        AND indexname = 'idx_social_graph_events_follower_id'
    ) THEN
        CREATE INDEX idx_social_graph_events_follower_id ON social_graph_events(follower_id);
    END IF;
END $$;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'social_graph_events' 
        AND column_name = 'following_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE tablename = 'social_graph_events' 
        AND indexname = 'idx_social_graph_events_following_id'
    ) THEN
        CREATE INDEX idx_social_graph_events_following_id ON social_graph_events(following_id);
    END IF;
END $$;

-- Add comment to describe the purpose of the table
COMMENT ON TABLE social_graph_events IS 'Records all follow/unfollow events for audit and history';