table! {
    groups (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    permissions (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    tokens (token) {
        token -> Uuid,
        token_type -> Text,
        user_id -> Uuid,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

table! {
    user_groups (id) {
        id -> Uuid,
        user_id -> Uuid,
        group_id -> Uuid,
        permission_id -> Uuid,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        internal_permissions -> Int8,
        eth_address -> Nullable<Text>,
        signature -> Nullable<Text>,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    groups,
    permissions,
    tokens,
    user_groups,
    users,
);
