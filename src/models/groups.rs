#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::models::schema::groups;
use crate::models::uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Debug)]
#[table_name = "groups"]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl Group {
    pub fn get_by_id(p_id: Uuid, conn: &PgConnection) -> Result<Self, String> {
        use crate::models::schema::groups::dsl::*;
        groups
            .filter(id.eq(p_id))
            .first::<Self>(conn)
            .map_or_else(|_| Err("Group doesn't exist".to_string()), Ok)
    }

    pub fn get_by_name(p_name: String, conn: &PgConnection) -> Result<Self, String> {
        use crate::models::schema::groups::dsl::*;
        groups
            .filter(name.eq(p_name))
            .first::<Self>(conn)
            .map_or_else(|_| Err("Group doesn't exist".to_string()), Ok)
    }
}

#[derive(Debug, PartialEq, Deserialize, Insertable)]
#[table_name = "groups"]
pub struct GroupForm {
    pub name: String,
    pub description: Option<String>,
    pub created_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
}

impl GroupForm {
    pub fn insert(mut self, conn: &PgConnection) -> Group {
        self.created_at = SystemTime::now();
        self.deleted_at = None;
        diesel::insert_into(groups::table)
            .values(self)
            .get_result(conn)
            .expect("error inserting group")
    }
}

#[cfg(test)]
mod tests {
    use super::Group;
    use super::GroupForm;
    use diesel::{pg::PgConnection, prelude::*};
    use std::time::SystemTime;

    const TEST_DATABASE_URL: &str =
        "postgresql://root@127.0.0.1:26257/auth_service?sslmode=disable";

    #[test]
    fn test_group_insert() {
        let g = GroupForm {
            name: "test".to_string(),
            description: Some("test description".to_string()),
            created_at: SystemTime::now(),
            deleted_at: None,
        };
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));
        let group = g.insert(&conn);
        println!("{:?}", group);
        match Group::get_by_id(group.id.clone(), &conn) {
            Ok(q_group) => {
                assert_eq!(q_group.id.to_string(), group.id.to_string());
                assert_eq!(q_group.name, group.name);
            }
            Err(err) => {
                panic!("{}", err)
            }
        };
    }
}
