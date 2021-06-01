use diesel::pg::PgConnection;
use std::time::SystemTime;

use crate::core::response;
use crate::models::groups;
use crate::models::permissions;
use crate::models::tokens;
use crate::models::user_groups;
use crate::models::users;
use crate::models::uuid::Uuid;
use std::ops::Add;

pub fn permission(conn: &PgConnection) -> Result<permissions::Permission, response::Error> {
    permissions::PermissionForm {
        name: fakeit::password::generate(true, true, false, 5).add("_WRITE"),
        created_at: SystemTime::now(),
        deleted_at: None,
    }
    .insert(conn)
}

pub fn group(conn: &PgConnection) -> Result<groups::Group, response::Error> {
    groups::GroupForm {
        name: fakeit::password::generate(true, true, false, 10),
        description: Some("Random group".to_string()),
        created_at: SystemTime::now(),
        deleted_at: None,
    }
    .insert(conn)
}

pub fn user(conn: &PgConnection) -> Result<users::User, response::Error> {
    users::UserForm {
        internal_permissions: crate::core::internal_permissions::Permissions::max(),
        eth_address: Some(fakeit::password::generate(true, true, false, 32)),
        signature: Some(fakeit::password::generate(true, true, false, 127)),
        created_at: SystemTime::now(),
        deleted_at: None,
    }
    .insert(conn)
}

pub fn user_group(
    conn: &PgConnection,
    u_id: Uuid,
    g_id: Uuid,
    p_id: Uuid,
) -> Result<user_groups::UserGroup, response::Error> {
    user_groups::UserGroupForm {
        user_id: u_id,
        group_id: g_id,
        permission_id: p_id,
        created_at: SystemTime::now(),
        deleted_at: None,
    }
    .insert(conn)
}

pub fn user_journey(conn: &PgConnection) -> Result<users::User, response::Error> {
    let p = permission(conn)?;
    let g = group(conn)?;
    let u = user(conn)?;
    user_group(conn, u.id, g.id, p.id)?;

    Ok(u)
}

pub fn auth_token(conn: &PgConnection, u: users::User) -> tokens::Token {
    tokens::TokenForm {
        token_type: tokens::AUTH_TYPE.to_string(),
        user_id: u.id,
        created_at: SystemTime::now(),
        expires_at: SystemTime::now(),
    }
    .insert(conn)
}
