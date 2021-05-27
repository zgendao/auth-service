-- Your SQL goes here
CREATE TABLE tokens (
    token UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    created_at TIMESTAMP,
    expires_at TIMESTAMP
);