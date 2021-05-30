#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::models::schema::users;
use crate::models::uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Debug, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub internal_permissions: i64,
    pub eth_address: Option<String>,
    pub signature: Option<String>,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl User {
    pub fn get_by_id(p_id: Uuid, conn: &PgConnection) -> Result<User, String> {
        use crate::models::schema::users::dsl::*;
        users
            .filter(id.eq(p_id))
            .first::<User>(conn)
            .map_or_else(|_| Err("User doesn't exist".to_string()), |user| Ok(user))
    }

    pub fn get_by_eth_address(p_eth_address: String, conn: &PgConnection) -> Result<User, String> {
        use crate::models::schema::users::dsl::*;
        users
            .filter(eth_address.eq(p_eth_address))
            .first::<User>(conn)
            .map_or_else(|_| Err("User doesn't exist".to_string()), |user| Ok(user))
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserForm {
    pub internal_permissions: i64,
    pub eth_address: Option<String>,
    pub signature: Option<String>,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl UserForm {
    pub fn insert(&self, conn: &PgConnection) -> User {
        let u = UserForm {
            internal_permissions: 0,
            eth_address: self.clone().eth_address,
            signature: self.clone().signature,
            created_at: SystemTime::now(),
            deleted_at: None,
        };
        diesel::insert_into(users::table)
            .values(u)
            .get_result(conn)
            .expect("error inserting user")
    }
}

// @TODO tests
