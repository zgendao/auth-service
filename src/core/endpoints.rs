use diesel::pg::PgConnection;

use crate::core::request;
use crate::core::response;
use crate::models::user_groups;
use crate::models::users;

pub(crate) fn login(conn: &PgConnection, login: request::Login) -> String {
    match users::User::get_by_eth_address(login.eth_address.clone(), conn) {
        Ok(u) => match user_groups::UserGroup::get_by_user_id(u.id, &conn) {
            Ok(ug) => {
                let mut result = response::LoginSuccess::new();
                result.build(conn, ug, login.eth_address.clone(), u.id);
                result.parse()
            }
            Err(err) => {
                response::LoginFailed::new(err).parse()
            }
        },
        Err(err) => {
            response::LoginFailed::new(err).parse()
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
