#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::models::schema::users;
use crate::models::uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub internal_permissions: i8,
    pub email: Option<String>,
    pub email_verified: bool,
    pub eth_address: Option<String>,
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

    pub fn get_by_username(p_username: String, conn: &PgConnection) -> Result<User, String> {
        use crate::models::schema::users::dsl::*;
        users
            .filter(username.eq(p_username))
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
    pub username: String,
    pub password: String,
    pub internal_permissions: i8,
    pub email: Option<String>,
    pub email_verified: bool,
    pub eth_address: Option<String>,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl UserForm {
    pub fn insert(&self, conn: &PgConnection) -> User {
        // TODO overwrite created_at
        // TODO overwrite deleted_at
        diesel::insert_into(users::table)
            .values(self)
            .get_result(conn)
            .expect("error inserting user")
    }
}

// @TODO tests