use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Login {
    pub(crate) eth_address: String,
    pub(crate) signature: String,
}

pub(crate) struct Register {
    pub(crate) eth_address: String,
    pub(crate) signature: String,
    pub(crate) register_token: String,
}
