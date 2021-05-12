-- Your SQL goes here
CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name STRING NOT NULL,
    created_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP,
);