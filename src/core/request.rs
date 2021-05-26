use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Login {
    pub(crate) eth_address: String,
    pub(crate) signature: String,
}