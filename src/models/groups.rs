#![allow(proc_macro_derive_resolution_fallback)]
use diesel::insert_into;
use diesel::prelude::*;
use std::time::SystemTime;

use crate::schema::groups;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "groups"]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "groups"]
pub struct GroupForm<'a> {
    name: &'a str,
    description: Option<&'a str>,
}

pub fn insert_default_values(conn: &PgConnection) -> QueryResult<usize> {
    use schema::groups::dsl::*;

    insert_into(groups).default_values().execute(conn)
}
