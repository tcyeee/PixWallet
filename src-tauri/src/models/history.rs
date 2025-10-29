use chrono::Utc;
use serde::{Deserialize, Serialize};
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_sdk::clock::Slot;
use std::{fmt, str::FromStr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct History {
    pub public_key: String,
    pub signature: String,
    pub slot: Slot,
    pub err: Option<String>,
    pub memo: Option<String>,
    pub block_time: Option<i64>,
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

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Processed => "Processed",
            Status::Confirmed => "Confirmed",
            Status::Finalized => "Finalized",
        };
        write!(f, "{}", s)
    }
}

impl History {
    pub fn parse_from_signature(
        sign: RpcConfirmedTransactionStatusWithSignature,
        public_key: &str,
    ) -> Result<Self, String> {
        let status_str = format!("{:?}", sign.confirmation_status);
        let status = Status::from_str(&status_str)?;
        let history = History {
            public_key: public_key.to_string(),
            signature: sign.signature,
            slot: sign.slot,
            err: sign.err.map(|e| format!("{:?}", e)),
            memo: sign.memo,
            block_time: sign.block_time,
            confirmation_status: Some(status),
            remark: None,
            created_at: Utc::now().timestamp_millis(),
        };
        Ok(history)
    }
}
