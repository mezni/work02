-- Create outbox_events table for reliable event publishing
CREATE TABLE IF NOT EXISTS outbox_events (
    event_id VARCHAR(32) PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    aggregate_id VARCHAR(32) NOT NULL,
    payload JSONB NOT NULL,
    published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    published_at TIMESTAMP,
    
    -- Indexes for performance
    CONSTRAINT chk_event_type CHECK (event_type IN (
        'UserCreated',
        'UserUpdated',
        'UserDeleted',
        'UserVerified',
        'UserDeactivated',
        'UserReactivated'
    ))
);

-- Index for unpublished events (used by background processor)
CREATE INDEX idx_outbox_unpublished ON outbox_events(published, created_at) 
WHERE published = FALSE;

-- Index for aggregate lookups
CREATE INDEX idx_outbox_aggregate ON outbox_events(aggregate_id, created_at DESC);

-- Index for cleanup of old published events
CREATE INDEX idx_outbox_cleanup ON outbox_events(published, published_at) 
WHERE published = TRUE;