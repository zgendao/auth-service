#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

use crate::models::schema::tokens;
use crate::models::uuid::Uuid;

pub const AUTH_TYPE: &str = "auth";
pub const REGISTER_TYPE: &str = "register";

#[derive(Queryable, AsChangeset, Serialize, Debug)]
#[table_name = "tokens"]
pub struct Token {
    pub token: Uuid,
    pub token_type: String,
    pub user_id: Uuid,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

impl Token {
    pub fn get_by_token(p_token: Uuid, conn: &PgConnection) -> Result<Self, String> {
        use crate::models::schema::tokens::dsl::*;
        tokens
            .filter(token.eq(p_token))
            .first::<Self>(conn)
            .map_or_else(|_| Err("Token doesn't exist".to_string()), |t| Ok(t))
    }

    pub fn delete(&self, conn: &PgConnection) -> Result<(), String> {
        use crate::models::schema::tokens::dsl::*;
        diesel::delete(tokens.filter(token.eq(self.token)))
            .execute(conn)
            .unwrap();
        Ok(())
    }
}

#[derive(Debug, PartialEq, Deserialize, Insertable)]
#[table_name = "tokens"]
pub struct TokenForm {
    pub token_type: String,
    pub user_id: Uuid,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

impl TokenForm {
    pub fn insert(mut self, conn: &PgConnection) -> Token {
        self.created_at = SystemTime::now();
        self.expires_at = SystemTime::now()
            .checked_add(Duration::new(10800, 0))
            .unwrap();
        diesel::insert_into(tokens::table)
            .values(self)
            .get_result(conn)
            .expect("error inserting tokan")
    }
}

// @TODO tests
