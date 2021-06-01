#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::core::response::Error;
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
    pub fn get_by_id(p_id: Uuid, conn: &PgConnection) -> Result<Group, String> {
        use crate::models::schema::groups::dsl::*;
        groups.filter(id.eq(p_id)).first::<Group>(conn).map_or_else(
            |_| Err("Group doesn't exist".to_string()),
            |group| Ok(group),
        )
    }

    pub fn get_by_name(p_name: &str, conn: &PgConnection) -> Result<Group, String> {
        use crate::models::schema::groups::dsl::*;
        groups
            .filter(name.eq(p_name))
            .first::<Group>(conn)
            .map_or_else(
                |_| Err("Group doesn't exist".to_string()),
                |group| Ok(group),
            )
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
    pub fn insert(self, conn: &PgConnection) -> Result<Group, Error> {
        let g = GroupForm {
            name: self.name,
            description: self.description,
            created_at: SystemTime::now(),
            deleted_at: None,
        };
        let result = diesel::insert_into(groups::table)
            .values(g)
            .get_result(conn);

        result.map_err(|e| Error::new(format!("group form error: {}", e)))
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
        let group = g.insert(&conn).unwrap();
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
