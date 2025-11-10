-- Rename mydata_id column back to my_ip_id in posts table

DO $$
BEGIN
    -- Check if mydata_id exists and my_ip_id doesn't exist
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'mydata_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'my_ip_id'
    ) THEN
        ALTER TABLE posts RENAME COLUMN mydata_id TO my_ip_id;
    END IF;
END
$$;

