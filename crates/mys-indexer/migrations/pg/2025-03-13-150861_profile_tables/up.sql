CREATE TABLE profiles (
    profile_id          BYTEA        PRIMARY KEY,
    owner              BYTEA        NOT NULL,
    display_name       TEXT         NOT NULL,
    bio                TEXT         NOT NULL,
    profile_picture    TEXT         NULL,
    created_at         BIGINT       NOT NULL,
    username_nft_id    BYTEA        NULL,
    tx_sequence_number BIGINT       NOT NULL,
    checkpoint_sequence_number BIGINT NOT NULL,
    timestamp_ms       BIGINT       NOT NULL
);

CREATE TABLE profile_events (
    tx_sequence_number   BIGINT       NOT NULL,
    event_sequence_number BIGINT      NOT NULL,
    profile_id           BYTEA        NOT NULL,
    event_type           TEXT         NOT NULL,
    owner                BYTEA        NOT NULL,
    timestamp_ms         BIGINT       NOT NULL,
    data                 JSONB        NOT NULL,
    PRIMARY KEY (tx_sequence_number, event_sequence_number)
);

CREATE INDEX idx_profiles_owner ON profiles(owner);
CREATE INDEX idx_profile_events_profile_id ON profile_events(profile_id);