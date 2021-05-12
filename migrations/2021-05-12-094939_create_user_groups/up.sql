-- Your SQL goes here
CREATE TABLE user_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    group_id UUID NOT NULL,
    permission_id UUID NOT NULL,
    created_at TIMESTAMP,
    deleted_at TIMESTAMP
);