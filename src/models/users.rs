#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::core::response::Error;
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

    pub fn get_by_eth_address(p_eth_address: &str, conn: &PgConnection) -> Result<Self, String> {
        use crate::models::schema::users::dsl::*;
        users
            .filter(eth_address.eq(p_eth_address))
            .first::<Self>(conn)
            .map_or_else(|_| Err("User doesn't exist".to_string()), |user| Ok(user))
    }

    pub fn update(&self, conn: &PgConnection) -> Result<(), String> {
        use crate::models::schema::users::dsl::*;
        diesel::update(users.filter(id.eq(self.id)))
            .set((
                internal_permissions.eq(self.internal_permissions),
                signature.eq(self.signature.as_ref()),
            ))
            .get_result::<User>(conn)
            .map_err(|_| "error inserting user")?;
        Ok(())
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
    pub fn insert(self, conn: &PgConnection) -> Result<User, Error> {
        let u = UserForm {
            internal_permissions: self.internal_permissions,
            eth_address: self.eth_address,
            signature: self.signature,
            created_at: SystemTime::now(),
            deleted_at: None,
        };
        let result = diesel::insert_into(users::table).values(u).get_result(conn);

        result.map_err(|e| Error::new(format!("user form error: {}", e)))
    }
}

// @TODO tests
