use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferParams {
    pub paying: String,
    pub receiving: String,
    pub amount: u64,
}
