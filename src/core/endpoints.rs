use diesel::pg::PgConnection;

use crate::core::request;
use crate::core::response;
use crate::models::user_groups;
use crate::models::users;
use crate::models::tokens;
use crate::models::uuid::Uuid;
use std::str::FromStr;

pub(crate) fn login(conn: &PgConnection, login: request::Login) -> String {
    match users::User::get_by_eth_address(login.eth_address.clone(), conn) {
        Ok(u) => {
            let mut user = response::User::new();
            user.build(conn, u.id);
            user.parse()
        }
        Err(err) => {
            response::LoginFailed::new(err).parse()
        }
    }
}

pub(crate) fn introspection(conn: &PgConnection, token: String) -> String {
    let t = tokens::Token::get_by_token(Uuid::from(token), conn).unwrap();
    let mut user = response::User::new();
    user.build(conn, t.user_id);
    user.parse()
}

pub(crate) fn register_token(conn: &PgConnection, token: String) -> response::Token {
    response::Token::default()
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
    fn test_introspection() {
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));
        let seed_user = seed::user(&conn);
        let seed_token = seed::auth_token(&conn, seed_user);

        let user = endpoints::introspection(&conn, seed_token.token.0.to_string());
        println!("{}", user);
    }

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
