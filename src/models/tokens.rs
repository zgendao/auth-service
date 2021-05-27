#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::models::schema::tokens;
use crate::models::uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Debug, Clone)]
#[table_name = "tokens"]
pub struct Token {
    pub token: Uuid,
    pub user_id: Uuid,
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
}

impl Token {
    pub fn get_by_id(p_id: Uuid, conn: &PgConnection) -> Result<Permission, String> {
        use crate::models::schema::permissions::dsl::*;
        permissions
            .filter(id.eq(p_id))
            .first::<Permission>(conn)
            .map_or_else(
                |_| Err("Permission doesn't exist".to_string()),
                |permission| Ok(permission),
            )
    }

    pub fn get_by_name(p_name: String, conn: &PgConnection) -> Result<Permission, String> {
        use crate::models::schema::permissions::dsl::*;
        permissions
            .filter(name.eq(p_name))
            .first::<Permission>(conn)
            .map_or_else(
                |_| Err("Permission doesn't exist".to_string()),
                |permission| Ok(permission),
            )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Insertable)]
#[table_name = "permissions"]
pub struct PermissionForm {
    pub name: String,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl PermissionForm {
    pub fn insert(&self, conn: &PgConnection) -> Permission {
        // TODO overwrite created_at
        // TODO overwrite deleted_at
        diesel::insert_into(permissions::table)
            .values(self)
            .get_result(conn)
            .expect("error inserting permission")
    }
}

// @TODO tests
