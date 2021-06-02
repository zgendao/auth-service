use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub eth_address: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub eth_address: String,
    pub signature: String,
    pub register_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroup {
    pub eth_address: String,
    pub group_name: String,
    pub permission_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInternalPermission {
    pub eth_address: String,
    pub internal_permission: String,
}
