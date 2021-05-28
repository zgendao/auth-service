use diesel::pg::PgConnection;
use std::time::SystemTime;

use crate::models::groups;
use crate::models::permissions;
use crate::models::user_groups;
use crate::models::users;
use crate::models::tokens;

pub(crate) struct UserResult {
    pub(crate) p: permissions::Permission,
    pub(crate) g: groups::Group,
    pub(crate) u: users::User,
    pub(crate) ug: user_groups::UserGroup,
}

pub(crate) fn user(conn: &PgConnection) -> UserResult {
    let pf = permissions::PermissionForm {
        name: "NGEN_WRITE".to_string(),
        created_at: SystemTime::now(),
        deleted_at: None,
    };
    let p = pf.insert(conn);

    let gf = groups::GroupForm {
        name: "neurogenesis".to_string(),
        description: Some("Allow to modify neurogenesis".to_string()),
        created_at: SystemTime::now(),
        deleted_at: None,
    };
    let g = gf.insert(conn);

    let uf = users::UserForm {
        internal_permissions: 12,
        eth_address: Some(fakeit::password::generate(true, true, false, 32)),
        signature: Some(fakeit::password::generate(true, true, false, 127)),
        created_at: SystemTime::now(),
        deleted_at: None,
    };
    let u = uf.insert(conn);

    let ugf = user_groups::UserGroupForm {
        user_id: u.clone().id,
        group_id: g.clone().id,
        permission_id: p.clone().id,
        created_at: SystemTime::now(),
        deleted_at: None,
    };
    let ug = ugf.insert(conn);

    UserResult { p, g, u, ug }
}

pub(crate) fn auth_token(conn: &PgConnection, u: UserResult) -> tokens::Token {
    tokens::TokenForm{
        token_type: tokens::AUTH_TYPE.to_string(),
        user_id: u.u.id,
        created_at: SystemTime::now(),
        expires_at: SystemTime::now()
    }.insert(conn)
}
