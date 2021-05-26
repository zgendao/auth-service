use diesel::pg::PgConnection;
use std::time::SystemTime;

use crate::models::groups;
use crate::models::permissions;
use crate::models::user_groups;
use crate::models::users;

const ETH_ADDRESS: &str = "bf29c02dc4b041895d34b1c63e14c20b";
const SIGNATURE: &str = "cd8f145c39a27190d6469cc57fad290eddaddb143ef8bddd5b77edd95711f1532764e0b980d68432ab4693eaf5ffbc66faacbb4f365e7ef1db9ddb1d93746f7b";

pub(crate) struct UserResult {
    pub(crate) p: permissions::Permission,
    pub(crate) g: groups::Group,
    pub(crate) u: users::User,
    pub(crate) ug: user_groups::UserGroup,
}

pub(crate) fn user(conn: &PgConnection) -> UserResult {
    let pf = permissions::PermissionForm {
        name: "".to_string(),
        created_at: SystemTime::now(),
        deleted_at: None,
    };
    let p = pf.insert(conn);

    let gf = groups::GroupForm {
        name: "NGEN_WRITE".to_string(),
        description: Some("Allow to modify neurogenesis".to_string()),
        created_at: SystemTime::now(),
        deleted_at: None,
    };
    let g = gf.insert(conn);

    let uf = users::UserForm {
        internal_permissions: 12,
        eth_address: Some(ETH_ADDRESS.to_string()),
        signature: Some(SIGNATURE.to_string()),
        created_at: SystemTime::now(),
        deleted_at: None,
    };
    let u = uf.insert(conn);

    let ugf = user_groups::UserGroupForm {
        user_id: u.clone().id,
        group_id: g.clone().id,
        permission_id: p.clone().id,
        created_at: Some(SystemTime::now()),
        deleted_at: None,
    };
    let ug = ugf.insert(conn);

    UserResult { p, g, u, ug }
}
