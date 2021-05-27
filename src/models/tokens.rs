#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

use crate::models::schema::tokens;
use crate::models::uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Debug, Clone)]
#[table_name = "tokens"]
pub struct Token {
    pub token: Uuid,
    pub user_id: Uuid,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

impl Token {
    pub fn get_by_token(p_token: Uuid, conn: &PgConnection) -> Result<Token, String> {
        use crate::models::schema::tokens::dsl::*;
        tokens
            .filter(token.eq(p_token))
            .first::<Token>(conn)
            .map_or_else(
                |_| Err("Token doesn't exist".to_string()),
                |t| Ok(t),
            )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Insertable)]
#[table_name = "tokens"]
pub struct TokenForm {
    pub user_id: Uuid,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

impl TokenForm {
    pub fn insert(&self, conn: &PgConnection) -> Token {
        let t = TokenForm {
            user_id: self.user_id,
            created_at: SystemTime::now(),
            expires_at: SystemTime::now().checked_add(Duration::new(10800, 0)).unwrap(),
        };
        diesel::insert_into(tokens::table)
            .values(t)
            .get_result(conn)
            .expect("error inserting tokan")
    }
}

// @TODO tests
