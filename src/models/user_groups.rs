#![allow(proc_macro_derive_resolution_fallback)]
use diesel::{pg::PgConnection, prelude::*, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::models::schema::user_groups;
use crate::models::uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Debug, Clone)]
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
        user_groups
            .filter(id.eq(p_id))
            .first::<UserGroup>(conn)
            .map_or_else(
                |_| Err("UserGroup doesn't exist".to_string()),
                |user_group| Ok(user_group),
            )
    }

    pub fn get_by_user_id(p_user_id: Uuid, conn: &PgConnection) -> Result<Vec<UserGroup>, String> {
        use crate::models::schema::user_groups::dsl::*;
        user_groups
            .filter(user_id.eq(p_user_id))
            .load::<UserGroup>(conn)
            .map_or_else(
                |_| Err("UserGroup doesn't exist".to_string()),
                |user_group| Ok(user_group),
            )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Insertable)]
#[table_name = "user_groups"]
pub struct UserGroupForm {
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

#[cfg(test)]
mod tests {
    use diesel::{pg::PgConnection, prelude::*};
    use std::time::SystemTime;

    use crate::models::user_groups::{UserGroup, UserGroupForm};
    use crate::models::uuid::Uuid;

    const TEST_DATABASE_URL: &str =
        "postgresql://root@127.0.0.1:26257/auth_service?sslmode=disable";

    #[test]
    fn test_group_insert() {
        let user_id = Uuid::from(uuid::Uuid::new_v4());
        let group_id = Uuid::from(uuid::Uuid::new_v4());
        let mut ug = UserGroupForm {
            user_id,
            group_id,
            permission_id: Uuid::from(uuid::Uuid::new_v4()),
            created_at: Some(SystemTime::now()),
            deleted_at: None,
        };
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));

        ug.insert(&conn);
        ug.permission_id = Uuid::from(uuid::Uuid::new_v4());
        ug.insert(&conn);
        ug.permission_id = Uuid::from(uuid::Uuid::new_v4());
        ug.insert(&conn);

        let result = UserGroup::get_by_user_id(user_id, &conn);
        match result {
            Ok(v) => {
                if v.len() != 3 {
                    panic!("Vector length should be 3 instead of {}", v.len());
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        };
    }
}
