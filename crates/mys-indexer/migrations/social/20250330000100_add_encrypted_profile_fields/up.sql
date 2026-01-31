-- Add new fields to profiles table
DO $$ 
BEGIN
    BEGIN
        ALTER TABLE profiles ADD COLUMN cover_photo TEXT;
    EXCEPTION
        WHEN duplicate_column THEN NULL;
    END;
    
    BEGIN
        ALTER TABLE profiles ADD COLUMN profile_id TEXT;
    EXCEPTION
        WHEN duplicate_column THEN NULL;
    END;
    
    BEGIN
        ALTER TABLE profiles ADD COLUMN has_private_data BOOLEAN NOT NULL DEFAULT FALSE;
    EXCEPTION
        WHEN duplicate_column THEN NULL;
    END;
    
    BEGIN
        ALTER TABLE profiles ADD COLUMN private_data_updated_at TIMESTAMP;
    EXCEPTION
        WHEN duplicate_column THEN NULL;
    END;
END $$;

-- Create private_fields table for tracking which private fields exist
CREATE TABLE IF NOT EXISTS profile_private_fields (
    profile_id INTEGER PRIMARY KEY,
    has_birthdate BOOLEAN NOT NULL DEFAULT FALSE,
    has_current_location BOOLEAN NOT NULL DEFAULT FALSE,
    has_raised_location BOOLEAN NOT NULL DEFAULT FALSE,
    has_phone BOOLEAN NOT NULL DEFAULT FALSE,
    has_email BOOLEAN NOT NULL DEFAULT FALSE,
    has_gender BOOLEAN NOT NULL DEFAULT FALSE,
    has_political_view BOOLEAN NOT NULL DEFAULT FALSE,
    has_education BOOLEAN NOT NULL DEFAULT FALSE,
    has_primary_language BOOLEAN NOT NULL DEFAULT FALSE,
    has_relationship_status BOOLEAN NOT NULL DEFAULT FALSE,
    has_social_usernames BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create encrypted_data table
CREATE TABLE IF NOT EXISTS profile_encrypted_data (
    profile_id INTEGER PRIMARY KEY,
    encrypted_data BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Add foreign key constraints only if profiles table exists with id column
DO $$
BEGIN
    -- Add foreign key constraint to profile_private_fields
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profile_private_fields' AND column_name = 'profile_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'profile_private_fields_profile_id_fkey'
    ) THEN
        ALTER TABLE profile_private_fields
        ADD CONSTRAINT profile_private_fields_profile_id_fkey
        FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE;
    END IF;
    
    -- Add foreign key constraint to profile_encrypted_data
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' AND column_name = 'id'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profile_encrypted_data' AND column_name = 'profile_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'profile_encrypted_data_profile_id_fkey'
    ) THEN
        ALTER TABLE profile_encrypted_data
        ADD CONSTRAINT profile_encrypted_data_profile_id_fkey
        FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE;
    END IF;
END $$;