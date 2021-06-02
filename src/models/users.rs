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
    pub internal_permissions: i64,
    pub eth_address: Option<String>,
    pub signature: Option<String>,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl User {
    pub fn get_by_id(p_id: Uuid, conn: &PgConnection) -> Result<Self, String> {
        use crate::models::schema::users::dsl::*;
        users
            .filter(id.eq(p_id))
            .first::<Self>(conn)
            .map_or_else(|_| Err("User doesn't exist".to_string()), |user| Ok(user))
    }

    pub fn get_by_eth_address(p_eth_address: String, conn: &PgConnection) -> Result<Self, String> {
        use crate::models::schema::users::dsl::*;
        users
            .filter(eth_address.eq(p_eth_address))
            .first::<Self>(conn)
            .map_or_else(|_| Err("User doesn't exist".to_string()), |user| Ok(user))
    }

    pub fn update(self, conn: &PgConnection) -> Result<Self, String> {
        use crate::models::schema::users::dsl::*;
        let signature_opt = self.signature.clone();
        diesel::update(users.filter(id.eq(self.id)))
            .set((
                internal_permissions.eq(self.internal_permissions),
                signature.eq(&signature_opt),
            ))
            .get_result::<Self>(conn)
            .expect("error inserting user");
        Ok(self)
    }
}

#[derive(Debug, PartialEq, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserForm {
    pub internal_permissions: i64,
    pub eth_address: Option<String>,
    pub signature: Option<String>,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl UserForm {
    pub fn insert(mut self, conn: &PgConnection) -> User {
        self.created_at = SystemTime::now();
        self.deleted_at = None;
        diesel::insert_into(users::table)
            .values(self)
            .get_result(conn)
            .expect("error inserting user")
    }
}

// @TODO tests
