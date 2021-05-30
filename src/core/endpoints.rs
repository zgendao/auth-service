use diesel::pg::PgConnection;
use std::env;

use crate::core::internal_permissions;
use crate::core::request;
use crate::core::response;
use crate::core::response::{Error, User};
use crate::models::tokens;
use crate::models::tokens::Token;
use crate::models::users;
use crate::models::uuid::Uuid;
use std::env::VarError;
use std::time::SystemTime;

pub(crate) fn login(conn: &PgConnection, login: request::Login) -> String {
    match login_base(conn, login) {
        Ok(l) => l.parse(),
        Err(e) => e.parse(),
    }
}

fn login_base(
    conn: &PgConnection,
    login: request::Login,
) -> Result<response::User, response::Error> {
    match users::User::get_by_eth_address(login.eth_address.clone(), conn) {
        Ok(u) => {
            let mut user = response::User::new();
            user.build(conn, u.id);
            Ok(user)
        }
        Err(err) => Err(response::Error::new(err)),
    }
}

pub(crate) fn introspection(conn: &PgConnection, token: String) -> String {
    match introspection_base(conn, token) {
        Ok(u) => u.parse(),
        Err(e) => e.parse(),
    }
}

fn introspection_base(
    conn: &PgConnection,
    token: String,
) -> Result<response::User, response::Error> {
    let t = match tokens::Token::get_by_token(Uuid::from(token), conn) {
        Ok(t) => t,
        Err(e) => return Err(response::Error::new(e)),
    };
    let mut user = response::User::new();
    user.build(conn, t.user_id);
    Ok(user)
}

pub(crate) fn register_token(conn: &PgConnection, token: String) -> String {
    match register_token_base(conn, token) {
        Ok(t) => t.parse(),
        Err(e) => e.parse(),
    }
}

fn register_token_base(
    conn: &PgConnection,
    token: String,
) -> Result<response::Token, response::Error> {
    let user = introspection_base(conn, token)?;
    if user
        .internal_permissions
        .contains(&internal_permissions::CREATE_USER.to_string())
    {
        return Ok(response::Token::new_register(
            conn,
            Uuid::from(user.user_id),
        ));
    }
    Err(response::Error::new("forbidden (CREATE_USER)".to_string()))
}

pub(crate) fn register(conn: &PgConnection, register: request::Register) -> String {
    match register_base(conn, register) {
        Ok(u) => u.parse(),
        Err(e) => e.parse(),
    }
}

fn register_base(
    conn: &PgConnection,
    register: request::Register,
) -> Result<response::User, response::Error> {
    let admin_account = match env::var("ADMIN_ACCOUNT") {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };
    if admin_account != register.eth_address {
        let token = match tokens::Token::get_by_token(Uuid::from(register.register_token), conn) {
            Ok(t) => t,
            Err(_) => {
                return Err(response::Error::new("token not existing".to_string()));
            }
        };
        if token.token_type != tokens::REGISTER_TYPE {
            return Err(response::Error::new(
                "token must be registration token".to_string(),
            ));
        }
        if token.expires_at.lt(&SystemTime::now()) {
            return Err(response::Error::new("token expired".to_string()));
        }
        token.delete(conn).unwrap();
    }

    let mut u = users::UserForm {
        internal_permissions: 0,
        eth_address: Some(register.eth_address.clone()),
        signature: Some(register.signature),
        created_at: SystemTime::now(),
        deleted_at: None,
    };

    if admin_account == register.eth_address.clone() {
        u.internal_permissions = internal_permissions::Permissions::max();
    }

    let saved_u = u.insert(conn);
    let mut response_u = response::User::new();
    response_u.build(conn, saved_u.id);
    Ok(response_u)
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
    fn test_end_registration() {
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));
        let seed_user = seed::user_journey(&conn);

        let u = endpoints::login_base(
            &conn,
            request::Login {
                eth_address: seed_user.eth_address.unwrap(),
                signature: seed_user.signature.unwrap(),
            },
        )
        .unwrap();

        let rt = endpoints::register_token_base(&conn, u.token.token).unwrap();

        let r = endpoints::register_base(
            &conn,
            request::Register {
                eth_address: fakeit::password::generate(true, true, false, 32),
                signature: fakeit::password::generate(true, true, false, 127),
                register_token: rt.token,
            },
        )
        .unwrap();

        // CREATE group
        // CREATE 3 persmissions
        // add user to group with permissions
        // add internal permissions to user
    }

    #[test]
    fn test_introspection() {
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));
        let seed_user = seed::user_journey(&conn);
        let seed_token = seed::auth_token(&conn, seed_user);

        let user = endpoints::introspection(&conn, seed_token.token.0.to_string());
        println!("{}", user);
    }

    #[test]
    fn test_register_token() {
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));
        let seed_user = seed::user_journey(&conn);
        let seed_token = seed::auth_token(&conn, seed_user);

        let token = endpoints::register_token(&conn, seed_token.token.0.to_string());
        println!("{}", token);
    }

    #[test]
    fn test_login() {
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));

        let seed_result = seed::user_journey(&conn);

        let l = request::Login {
            eth_address: seed_result.eth_address.unwrap(),
            signature: seed_result.signature.unwrap(),
        };

        let result = endpoints::login(&conn, l);
        println!("{}", result);
    }
}
