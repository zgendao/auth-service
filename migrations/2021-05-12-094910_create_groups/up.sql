-- Your SQL goes here
CREATE TABLE groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name STRING NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP,
);