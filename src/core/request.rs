use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Login {
    pub(crate) eth_address: String,
    pub(crate) signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Register {
    pub(crate) eth_address: String,
    pub(crate) signature: String,
    pub(crate) register_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Group {
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Permission {
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UserGroup {
    pub(crate) eth_address: String,
    pub(crate) group_name: String,
    pub(crate) permission_name: String,
}