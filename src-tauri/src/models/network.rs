use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
            SolanaNetwork::Local => "http://192.168.187.135:8899",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "mainnet" => SolanaNetwork::Mainnet,
            "testnet" => SolanaNetwork::Testnet,
            "devnet" => SolanaNetwork::Devnet,
            "local" => SolanaNetwork::Local,
            _ => SolanaNetwork::Local,
        }
    }

    // 获取 RPC 客户端
    pub fn get_rpc_client(network: SolanaNetwork) -> RpcClient {
        RpcClient::new(network.url().to_string())
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
