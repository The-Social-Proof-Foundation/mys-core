-- Rename my_ip_id column to mydata_id in posts table
-- This aligns with the my_ip -> mydata rename that was done in other tables

DO $$
BEGIN
    -- Check if my_ip_id exists and mydata_id doesn't exist
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'my_ip_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'posts' AND column_name = 'mydata_id'
    ) THEN
        ALTER TABLE posts RENAME COLUMN my_ip_id TO mydata_id;
    END IF;
END
$$;

