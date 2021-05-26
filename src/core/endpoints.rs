use diesel::pg::PgConnection;
use serde_json;

use crate::core::request;
use crate::core::response::LoginFailed;
use crate::models::users;
use crate::models::user_groups;
use crate::utils::connection;
use crate::models::user_groups::UserGroup;

pub(crate) fn login(conn: &PgConnection, login: request::Login) -> String {
    match users::User::get_by_eth_address(login.eth_address, conn) {
        Ok(u) => {
            match user_groups::UserGroup::get_by_user_id(u.id, &conn) {
                Ok(uhg) => {
                    serde_json::to_string(&uhg).unwrap()
                }
                Err(err) => {
                    let l = LoginFailed {
                        msg: err,
                        reason_code: "INTERNAL_ERROR".to_string(),
                    };
                    serde_json::to_string(&l).unwrap()
                }
            }
        },
        Err(err) => {
            let l = LoginFailed {
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

    use crate::core::request;
    use crate::models::seed;
    use crate::core::endpoints;

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
