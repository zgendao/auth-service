table! {
    group (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Date,
    }
}

table! {
    permission (id) {
        id -> Varchar,
        name -> Varchar,
        created_at -> Date,
    }
}

table! {
    user (id) {
        id -> Varchar,
        username -> Varchar,
        password -> Varchar,
        internal_permissions -> Int64,
        email -> Varchar,
        email_verified -> Bool,
        eth_address -> Varchar,
        created_at -> Date,
    }
}
