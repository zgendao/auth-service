use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use crate::core::internal_permissions;
use crate::models::tokens;
use crate::models::user_groups;
use crate::models::users;
use crate::models::uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct User {
    pub(crate) user_id: String,
    pub(crate) groups: HashMap<String, Group>,
    pub(crate) internal_permissions: Vec<String>,
    pub(crate) eth_address: String,
    pub(crate) token: Token,
}

impl User {
    pub(crate) fn new() -> User {
        User {
            user_id: "".to_string(),
            groups: HashMap::<String, Group>::new(),
            internal_permissions: Vec::<String>::new(),
            eth_address: "".to_string(),
            token: Token::default(),
        }
    }

    pub(crate) fn build(&mut self, conn: &PgConnection, user_id: uuid::Uuid) {
        let ug = user_groups::UserGroup::get_by_user_id(user_id, &conn).unwrap();
        let u = users::User::get_by_id(user_id, conn).unwrap();
        self.build_groups(conn, ug);
        self.token = Token::new_auth(conn, user_id);
        self.eth_address = u.eth_address.unwrap();
        self.user_id = u.id.0.to_string();
        self.internal_permissions =
            internal_permissions::Permissions::from(u.internal_permissions).to_vec();
    }

    pub(crate) fn parse(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn build_groups(&mut self, conn: &PgConnection, ug: Vec<user_groups::UserGroup>) {
        use crate::models::groups;
        use crate::models::permissions;

        for ug_elem in ug.iter() {
            let g = groups::Group::get_by_id(ug_elem.group_id, conn).unwrap();
            let p = permissions::Permission::get_by_id(ug_elem.permission_id, conn).unwrap();
            if self.groups.contains_key(&*g.id.0.to_string()) {
                let g_mut = self.groups.get_mut(&*g.id.0.to_string()).unwrap();
                g_mut
                    .permissions
                    .insert(p.id.0.to_string(), Permission { name: p.name });
            } else {
                let mut permissions = HashMap::<String, Permission>::new();
                permissions.insert(p.id.0.to_string(), Permission { name: p.name });
                self.groups.insert(
                    g.id.0.to_string(),
                    Group {
                        name: g.name,
                        permissions,
                    },
                );
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Group {
    pub(crate) name: String,
    pub(crate) permissions: HashMap<String, Permission>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Permission {
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Token {
    pub(crate) token: String,
    pub(crate) expires_at: String,
}

impl Token {
    pub(crate) fn new_auth(conn: &PgConnection, user_id: uuid::Uuid) -> Token {
        let token = Token::save_token(conn, user_id, tokens::AUTH_TYPE.to_string());
        let dt = DateTime::<Utc>::from(token.expires_at);

        Token {
            token: token.token.0.to_string(),
            expires_at: dt.to_rfc3339(),
        }
    }

    pub(crate) fn new_register(conn: &PgConnection, user_id: uuid::Uuid) -> Token {
        let token = Token::save_token(conn, user_id, tokens::REGISTER_TYPE.to_string());
        let dt = DateTime::<Utc>::from(token.expires_at);

        Token {
            token: token.token.0.to_string(),
            expires_at: dt.to_rfc3339(),
        }
    }

    pub(crate) fn parse(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn save_token(conn: &PgConnection, user_id: uuid::Uuid, t: String) -> tokens::Token {
        let token = tokens::TokenForm {
            token_type: t,
            user_id,
            created_at: SystemTime::now(),
            expires_at: SystemTime::now(),
        }
        .insert(conn);
        token
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Error {
    pub(crate) msg: String,
    pub(crate) reason_code: String,
}

impl Error {
    pub(crate) fn new(err: String) -> Error {
        Error {
            msg: err,
            reason_code: "INTERNAL_ERROR".to_string(),
        }
    }

    pub(crate) fn parse(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
