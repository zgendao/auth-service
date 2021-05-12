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
    user_groups (id) {
        id -> Uuid,
        user_id -> Uuid,
        group_id -> Uuid,
        permission_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
        internal_permissions -> Int8,
        email -> Nullable<Text>,
        email_verified -> Bool,
        eth_address -> Nullable<Text>,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    groups,
    permissions,
    user_groups,
    users,
);
