-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    internal_permissions INT NOT NULL,
    eth_address STRING,
    signature STRING,
    created_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP
);