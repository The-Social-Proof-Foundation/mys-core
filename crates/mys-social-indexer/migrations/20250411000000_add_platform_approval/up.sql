-- Add is_approved column to platforms table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'is_approved'
    ) THEN
        ALTER TABLE platforms ADD COLUMN is_approved BOOLEAN NOT NULL DEFAULT false;
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'approval_changed_at'
    ) THEN
        ALTER TABLE platforms ADD COLUMN approval_changed_at TIMESTAMP NULL;
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'platforms' AND column_name = 'approved_by'
    ) THEN
        ALTER TABLE platforms ADD COLUMN approved_by TEXT NULL;
    END IF;
END $$;