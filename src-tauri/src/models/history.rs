use serde::{Deserialize, Serialize};
use solana_sdk::clock::{Slot, UnixTimestamp};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct History {
    pub public_key: String,
    pub signature: String,
    pub slot: Slot,
    pub err: Option<String>,
    pub memo: Option<String>,
    pub block_time: Option<UnixTimestamp>,
    pub confirmation_status: Option<Status>,
    pub remark: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Processed,
    Confirmed,
    Finalized,
}

impl FromStr for Status {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "processed" => Ok(Status::Processed),
            "confirmed" => Ok(Status::Confirmed),
            "finalized" => Ok(Status::Finalized),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}
