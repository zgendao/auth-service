use diesel::pg::PgConnection;
use std::env;

use crate::core::internal_permissions;
use crate::core::request;
use crate::core::response;
use crate::models::groups;
use crate::models::permissions;
use crate::models::tokens;
use crate::models::user_groups;
use crate::models::users;
use crate::models::uuid::Uuid;
use std::time::SystemTime;

pub fn login(conn: &PgConnection, login: request::Login) -> String {
    match login_base(conn, login) {
        Ok(l) => l.parse(),
        Err(e) => e.parse(),
    }
}

fn login_base(
    conn: &PgConnection,
    login: request::Login,
) -> Result<response::User, response::Error> {
    match users::User::get_by_eth_address(login.eth_address, conn) {
        Ok(u) => {
            let mut user = response::User::new();
            user.build(conn, u.id);
            Ok(user)
        }
        Err(err) => Err(response::Error::new(err)),
    }
}

pub fn introspection(conn: &PgConnection, token: &str) -> String {
    match introspection_base(conn, token) {
        Ok(u) => u.parse(),
        Err(e) => e.parse(),
    }
}

fn introspection_base(conn: &PgConnection, token: &str) -> Result<response::User, response::Error> {
    let t = match tokens::Token::get_by_token(Uuid::from(token), conn) {
        Ok(t) => t,
        Err(e) => return Err(response::Error::new(e)),
    };
    let mut user = response::User::new();
    user.build(conn, t.user_id);
    Ok(user)
}

pub fn register_token(conn: &PgConnection, token: &str) -> String {
    match register_token_base(conn, token) {
        Ok(t) => t.parse(),
        Err(e) => e.parse(),
    }
}

fn register_token_base(
    conn: &PgConnection,
    token: &str,
) -> Result<response::Token, response::Error> {
    let user = introspection_base(conn, token)?;
    if user
        .internal_permissions
        .iter()
        .any(|item| item == internal_permissions::MANAGE_USERS)
    {
        return Ok(response::Token::new_register(
            conn,
            Uuid::from(user.user_id),
        ));
    }
    Err(response::Error::new("forbidden (MANAGE_USERS)".to_string()))
}

pub fn register(conn: &PgConnection, register: request::Register) -> String {
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

    let eth_address = register.eth_address.as_str();
    let mut u = users::UserForm {
        internal_permissions: 0,
        eth_address: Some(eth_address.to_string()),
        signature: Some(register.signature),
        created_at: SystemTime::now(),
        deleted_at: None,
    };

    if admin_account == eth_address {
        u.internal_permissions = internal_permissions::Permissions::max();
    }

    let saved_u = u.insert(conn);
    let mut response_u = response::User::new();
    response_u.build(conn, saved_u.id);
    Ok(response_u)
}

pub fn create_group(conn: &PgConnection, group: request::Group, token: &str) -> String {
    match create_group_base(conn, group, token) {
        Ok(g) => serde_json::to_string(&g).unwrap(),
        Err(e) => e.parse(),
    }
}

fn create_group_base(
    conn: &PgConnection,
    group: request::Group,
    token: &str,
) -> Result<groups::Group, response::Error> {
    let user = introspection_base(conn, token)?;
    if user
        .internal_permissions
        .iter()
        .any(|item| item == internal_permissions::MANAGE_GROUPS)
    {
        let g = groups::GroupForm {
            name: group.name,
            description: Some(group.description),
            created_at: SystemTime::now(),
            deleted_at: None,
        }
        .insert(conn);
        return Ok(g);
    }
    Err(response::Error::new(
        "forbidden (MANAGE_GROUPS)".to_string(),
    ))
}

pub fn create_permission(
    conn: &PgConnection,
    permission: request::Permission,
    token: &str,
) -> String {
    match create_permission_base(conn, permission, token) {
        Ok(p) => serde_json::to_string(&p).unwrap(),
        Err(e) => e.parse(),
    }
}

fn create_permission_base(
    conn: &PgConnection,
    permission: request::Permission,
    token: &str,
) -> Result<permissions::Permission, response::Error> {
    let user = introspection_base(conn, token)?;
    if user
        .internal_permissions
        .iter()
        .any(|item| item == internal_permissions::MANAGE_PERMISSIONS)
    {
        let p = permissions::PermissionForm {
            name: permission.name,
            created_at: SystemTime::now(),
            deleted_at: None,
        }
        .insert(conn);
        return Ok(p);
    }
    Err(response::Error::new(
        "forbidden (MANAGE_PERMISSIONS)".to_string(),
    ))
}

pub fn add_user_group(conn: &PgConnection, user_group: request::UserGroup, token: &str) -> String {
    match add_user_group_base(conn, user_group, token) {
        Ok(ug) => serde_json::to_string(&ug).unwrap(),
        Err(e) => e.parse(),
    }
}

fn add_user_group_base(
    conn: &PgConnection,
    user_group: request::UserGroup,
    token: &str,
) -> Result<user_groups::UserGroup, response::Error> {
    let user = introspection_base(conn, token)?;
    if user
        .internal_permissions
        .iter()
        .any(|item| item == internal_permissions::MANAGE_USERS)
    {
        let u = users::User::get_by_eth_address(user_group.eth_address, conn).unwrap();
        let g = groups::Group::get_by_name(user_group.group_name, conn).unwrap();
        let p = permissions::Permission::get_by_name(user_group.permission_name, conn).unwrap();
        let ug = user_groups::UserGroupForm {
            user_id: u.id,
            group_id: g.id,
            permission_id: p.id,
            created_at: SystemTime::now(),
            deleted_at: None,
        }
        .insert(conn);
        return Ok(ug);
    }
    Err(response::Error::new("forbidden (MANAGE_USERS)".to_string()))
}

pub fn add_user_internal_permission(
    conn: &PgConnection,
    uip: request::UserInternalPermission,
    token: &str,
) -> String {
    match add_user_internal_permission_base(conn, uip, token) {
        Ok(u) => serde_json::to_string(&u).unwrap(),
        Err(e) => e.parse(),
    }
}

fn add_user_internal_permission_base(
    conn: &PgConnection,
    uip: request::UserInternalPermission,
    token: &str,
) -> Result<response::User, response::Error> {
    let user = introspection_base(conn, token)?;
    if user
        .internal_permissions
        .contains(&internal_permissions::SET_INTERNAL_PERMISSIONS.to_string())
    {
        let mut u = users::User::get_by_eth_address(uip.eth_address, conn).unwrap();
        let mut ip_vec = internal_permissions::Permissions::from(u.internal_permissions).to_vec();
        // TODO validate internal_permission
        ip_vec.push(uip.internal_permission);
        u.internal_permissions = internal_permissions::Permissions::from_vec(ip_vec).to_number();
        let new_u = u.update(conn).unwrap();

        let mut response_u = response::User::new();
        response_u.build(conn, new_u.id);
        return Ok(response_u);
    }
    Err(response::Error::new(
        "forbidden (SET_INTERNAL_PERMISSIONS)".to_string(),
    ))
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

        let rt = endpoints::register_token_base(&conn, u.token.clone().token.as_str()).unwrap();

        let r = endpoints::register_base(
            &conn,
            request::Register {
                eth_address: fakeit::password::generate(true, true, false, 32),
                signature: fakeit::password::generate(true, true, false, 127),
                register_token: rt.token,
            },
        )
        .unwrap();

        let g = endpoints::create_group_base(
            &conn,
            request::Group {
                name: "test_amazing_group".to_string(),
                description: "This is an amazing group".to_string(),
            },
            u.token.clone().token.as_str(),
        )
        .unwrap();

        let p1 = endpoints::create_permission_base(
            &conn,
            request::Permission {
                name: "service_WRITE".to_string(),
            },
            u.token.clone().token.as_str(),
        )
        .unwrap();
        let p2 = endpoints::create_permission_base(
            &conn,
            request::Permission {
                name: "service_READ".to_string(),
            },
            u.token.clone().token.as_str(),
        )
        .unwrap();
        let p3 = endpoints::create_permission_base(
            &conn,
            request::Permission {
                name: "service_DELETE".to_string(),
            },
            u.token.clone().token.as_str(),
        )
        .unwrap();
        let name = g.name.as_str();
        endpoints::add_user_group_base(
            &conn,
            request::UserGroup {
                eth_address: r.eth_address.clone(),
                group_name: name.to_string(),
                permission_name: p1.name,
            },
            u.token.clone().token.as_str(),
        )
        .unwrap();
        endpoints::add_user_group_base(
            &conn,
            request::UserGroup {
                eth_address: r.eth_address.clone(),
                group_name: name.to_string(),
                permission_name: p2.name,
            },
            u.token.clone().token.as_str(),
        )
        .unwrap();
        endpoints::add_user_group_base(
            &conn,
            request::UserGroup {
                eth_address: r.eth_address.clone(),
                group_name: name.to_string(),
                permission_name: p3.name,
            },
            u.token.clone().token.as_str(),
        )
        .unwrap();

        endpoints::add_user_internal_permission_base(
            &conn,
            request::UserInternalPermission {
                eth_address: r.eth_address.clone(),
                internal_permission: "manage_groups".to_string(),
            },
            u.token.clone().token.as_str(),
        )
        .unwrap();
    }

    #[test]
    fn test_introspection() {
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));
        let seed_user = seed::user_journey(&conn);
        let seed_token = seed::auth_token(&conn, seed_user);

        let user = endpoints::introspection(&conn, seed_token.token.to_string().as_str());
        assert_eq!(724, user.len());
        println!("{}", user.len());
    }

    #[test]
    fn test_register_token() {
        let conn = PgConnection::establish(&TEST_DATABASE_URL)
            .expect(&format!("Error connecting to {}", TEST_DATABASE_URL));
        let seed_user = seed::user_journey(&conn);
        let seed_token = seed::auth_token(&conn, seed_user);

        let token = endpoints::register_token(&conn, seed_token.token.to_string().as_str());
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
