#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::models::schema::permissions;
use crate::models::uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Debug)]
#[table_name = "permissions"]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl Permission {
    pub fn get_by_id(p_id: Uuid, conn: &PgConnection) -> Result<Self, String> {
        use crate::models::schema::permissions::dsl::*;
        permissions
            .filter(id.eq(p_id))
            .first::<Self>(conn)
            .map_or_else(
                |_| Err("Permission doesn't exist".to_string()),
                |permission| Ok(permission),
            )
    }

    pub fn get_by_name(p_name: String, conn: &PgConnection) -> Result<Self, String> {
        use crate::models::schema::permissions::dsl::*;
        permissions
            .filter(name.eq(p_name))
            .first::<Self>(conn)
            .map_or_else(
                |_| Err("Permission doesn't exist".to_string()),
                |permission| Ok(permission),
            )
    }
}

#[derive(Debug, PartialEq, Deserialize, Insertable)]
#[table_name = "permissions"]
pub struct PermissionForm {
    pub name: String,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl PermissionForm {
    pub fn insert(mut self, conn: &PgConnection) -> Permission {
        self.created_at = SystemTime::now();
        self.deleted_at = None;
        diesel::insert_into(permissions::table)
            .values(self)
            .get_result(conn)
            .expect("error inserting permission")
    }
}

// @TODO tests
