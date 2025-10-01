-- Create indexer_state table to track event processing progress
CREATE TABLE IF NOT EXISTS indexer_state (
    indexer_name VARCHAR(255) PRIMARY KEY,
    last_processed_version BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index for faster lookups
CREATE INDEX idx_indexer_state_updated_at ON indexer_state(updated_at);

-- Insert initial record for event_indexer
INSERT INTO indexer_state (indexer_name, last_processed_version, updated_at)
VALUES ('event_indexer', 0, CURRENT_TIMESTAMP)
ON CONFLICT (indexer_name) DO NOTHING;

-- Your SQL goes here
