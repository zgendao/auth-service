use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::core::internal_permissions;
use crate::models::tokens;
use crate::models::user_groups;
use crate::models::users;
use crate::models::uuid;
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub user_id: String,
    pub groups: HashMap<String, Group>,
    pub internal_permissions: Vec<String>,
    pub eth_address: String,
    pub token: Token,
}

impl User {
    pub fn new() -> User {
        User {
            user_id: "".to_string(),
            groups: HashMap::<String, Group>::new(),
            internal_permissions: Vec::<String>::new(),
            eth_address: "".to_string(),
            token: Token::default(),
        }
    }

    pub fn build(&mut self, conn: &PgConnection, user_id: uuid::Uuid) {
        let ug = user_groups::UserGroup::get_by_user_id(user_id, &conn).unwrap();
        let u = users::User::get_by_id(user_id, conn).unwrap();
        self.build_groups(conn, ug);
        self.eth_address = u.eth_address.unwrap();
        self.user_id = u.id.to_string();
        self.internal_permissions =
            internal_permissions::Permissions::from(u.internal_permissions).to_vec();
    }

    pub fn parse(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn build_groups(&mut self, conn: &PgConnection, ug: Vec<user_groups::UserGroup>) {
        use crate::models::groups;
        use crate::models::permissions;

        for ug_elem in ug.iter() {
            let g = groups::Group::get_by_id(ug_elem.group_id, conn).unwrap();
            let p = permissions::Permission::get_by_id(ug_elem.permission_id, conn).unwrap();
            if let std::collections::hash_map::Entry::Vacant(e) =
                self.groups.entry(g.id.to_string())
            {
                let mut permissions = HashMap::<String, Permission>::new();
                permissions.insert(p.id.to_string(), Permission { name: p.name });
                e.insert(Group {
                    name: g.name,
                    permissions,
                });
            } else {
                let g_mut = self.groups.get_mut(&g.id.to_string()).unwrap();
                g_mut
                    .permissions
                    .insert(p.id.to_string(), Permission { name: p.name });
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub permissions: HashMap<String, Permission>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Token {
    pub token: String,
    pub expires_at: String,
    pub valid: bool,
}

impl Token {
    pub fn new_auth(conn: &PgConnection, user_id: uuid::Uuid) -> Token {
        let token = Token::save_token(
            conn,
            user_id,
            tokens::AUTH_TYPE.to_string(),
            SystemTime::now().add(Duration::new(86_400, 0)),
        );
        let dt = DateTime::<Utc>::from(token.expires_at);

        Token {
            token: token.token.to_string(),
            expires_at: dt.to_rfc3339(),
            valid: true,
        }
    }

    pub fn new_register(conn: &PgConnection, user_id: uuid::Uuid) -> Token {
        let token = Token::save_token(
            conn,
            user_id,
            tokens::REGISTER_TYPE.to_string(),
            SystemTime::now().add(Duration::new(86_400, 0)),
        );
        let dt = DateTime::<Utc>::from(token.expires_at);

        Token {
            token: token.token.to_string(),
            expires_at: dt.to_rfc3339(),
            valid: true,
        }
    }

    pub fn new_long(conn: &PgConnection, user_id: uuid::Uuid) -> Token {
        let token_expires_at = SystemTime::now().add(Duration::new(31_556_952, 0));
        let token = Token::save_token(
            conn,
            user_id,
            tokens::LONG_TYPE.to_string(),
            token_expires_at,
        );
        let dt = DateTime::<Utc>::from(token_expires_at);

        Token {
            token: token.token.to_string(),
            expires_at: dt.to_rfc3339(),
            valid: true,
        }
    }

    pub fn parse(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn save_token(
        conn: &PgConnection,
        user_id: uuid::Uuid,
        t: String,
        expires_at: SystemTime,
    ) -> tokens::Token {
        tokens::TokenForm {
            token_type: t,
            user_id,
            created_at: SystemTime::now(),
            expires_at,
        }
        .insert(conn)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub msg: String,
    pub reason_code: String,
}

impl Error {
    pub fn new(err: String) -> Error {
        Error {
            msg: err,
            reason_code: "INTERNAL_ERROR".to_string(),
        }
    }

    pub fn parse(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
