use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum SolanaNetwork {
    Mainnet,
    Testnet,
    Devnet,
    Local,
}

impl SolanaNetwork {
    pub fn url(&self) -> &str {
        match self {
            SolanaNetwork::Mainnet => "https://api.mainnet-beta.solana.com",
            SolanaNetwork::Testnet => "https://api.testnet.solana.com",
            SolanaNetwork::Devnet => "https://api.devnet.solana.com",
            SolanaNetwork::Local => "http://127.0.0.1:8899",
        }
    }
}

impl std::fmt::Display for SolanaNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SolanaNetwork::Mainnet => "mainnet",
                SolanaNetwork::Devnet => "devnet",
                SolanaNetwork::Testnet => "testnet",
                SolanaNetwork::Local => "local",
            }
        )
    }
}
