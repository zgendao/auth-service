use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LoginSuccess {
    pub(crate) groups: HashMap<String, Group>,
    pub(crate) internal_permissions: Vec<String>,
    pub(crate) eth_address: String,
    pub(crate) token: Token,
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

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Token {
    pub(crate) token: String,
    pub(crate) expires_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LoginFailed {
    pub(crate) msg: String,
    pub(crate) reason_code: String,
}
