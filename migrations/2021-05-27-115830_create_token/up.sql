-- Your SQL goes here
CREATE TABLE tokens (
    token UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_type TEXT NOT NULL,
    user_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL,
    expires_at TIMESTAMP NOT NULL
);