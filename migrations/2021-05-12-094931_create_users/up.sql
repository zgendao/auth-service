-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username STRING NOT NULL,
    password STRING NOT NULL,
    internal_permissions INT NOT NULL,
    email STRING,
    email_verified BOOL NOT NULL DEFAULT false,
    eth_address STRING,
    created_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP,
);