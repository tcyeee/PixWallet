use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferParams {
    pub from: String,
    pub to: String,
    pub amount: f32,
}
