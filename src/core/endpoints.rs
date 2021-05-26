use diesel::pg::PgConnection;
use serde_json;
use std::collections::HashMap;

use crate::core::request;
use crate::core::response;
use crate::models::groups;
use crate::models::permissions;
use crate::models::user_groups;
use crate::models::user_groups::UserGroup;
use crate::models::users;
use crate::utils::connection;

pub(crate) fn login(conn: &PgConnection, login: request::Login) -> String {
    match users::User::get_by_eth_address(login.eth_address, conn) {
        Ok(u) => {
            match user_groups::UserGroup::get_by_user_id(u.id, &conn) {
                Ok(ug) => {
                    let mut h = HashMap::<String, response::Group>::new();
                    for ug_elem in ug.iter() {
                        let g = groups::Group::get_by_id(ug_elem.group_id, &conn).unwrap();
                        let p = permissions::Permission::get_by_id(ug_elem.permission_id, &conn)
                            .unwrap();
                        if h.contains_key(&*g.id.0.to_string()) {
                            let g_mut = h.get_mut(&*g.id.0.to_string()).unwrap();
                            g_mut.permissions.insert(p.id.0.to_string(), response::Permission{ name: p.name });
                        } else {
                            let mut permissions = HashMap::<String, response::Permission>::new();
                            permissions
                                .insert(p.id.0.to_string(), response::Permission { name: p.name });
                            h.insert(
                                g.id.0.to_string(),
                                response::Group {
                                    name: g.name,
                                    permissions,
                                },
                            );
                        }
                    }
                    serde_json::to_string(&h).unwrap()
                }
                Err(err) => {
                    let l = response::LoginFailed {
                        msg: err,
                        reason_code: "INTERNAL_ERROR".to_string(),
                    };
                    serde_json::to_string(&l).unwrap()
                }
            }
        }
        Err(err) => {
            let l = response::LoginFailed {
                msg: err,
                reason_code: "INTERNAL_ERROR".to_string(),
            };
            serde_json::to_string(&l).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use diesel::{pg::PgConnection, prelude::*};

    use crate::core::endpoints;
    use crate::core::request;
    use crate::models::seed;

    const TEST_DATABASE_URL: &str =
        "postgresql://root@127.0.0.1:26257/auth_service?sslmode=disable";

    #[test]
    fn test_login() {
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));

        let seed_result = seed::user(&conn);

        let l = request::Login {
            eth_address: seed_result.u.eth_address.unwrap(),
            signature: seed_result.u.signature.unwrap(),
        };

        let result = endpoints::login(&conn, l);
        println!("{}", result);
    }
}
