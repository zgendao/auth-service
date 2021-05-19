#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::models::schema::user_groups;
use crate::models::uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Debug)]
#[table_name = "user_groups"]
pub struct UserGroup {
    pub id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub permission_id: Uuid,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl UserGroup {
    pub fn get_by_id(p_id: Uuid, conn: &PgConnection) -> Result<UserGroup, String> {
        use crate::models::schema::user_groups::dsl::*;
        user_groups.filter(id.eq(p_id)).first::<UserGroup>(conn).map_or_else(
            |_| Err("UserGroup doesn't exist".to_string()),
            |user_group| Ok(user_group),
        )
    }

    // TODO fix
    // pub fn get_by_user_id(p_user_id: Uuid, conn: &PgConnection) -> Result<Vec<UserGroup>, String> {
    //     use crate::models::schema::user_groups::dsl::*;
    //     user_groups.filter(user_id.eq(p_user_id)).load::<UserGroup>(conn).map_or_else(
    //         |_| Err("UserGroup doesn't exist".to_string()),
    //         |user_group| Ok(user_group),
    //     )
    // }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Insertable)]
#[table_name = "user_groups"]
pub struct UserGroupForm {
    pub id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub permission_id: Uuid,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl UserGroupForm {
    pub fn insert(&self, conn: &PgConnection) -> UserGroup {
        // TODO overwrite created_at
        // TODO overwrite deleted_at
        diesel::insert_into(user_groups::table)
            .values(self)
            .get_result(conn)
            .expect("error inserting user_groups")
    }
}

// TODO test