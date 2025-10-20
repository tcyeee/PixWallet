use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use std::path::PathBuf;

use crate::models::network::SolanaNetwork;

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletInfo {
    pub public_key: String,
    pub private_key: String,
    pub network: SolanaNetwork,
    pub balance: Option<u64>,
    pub alias: Option<String>,
}

const WALLET_FILE_PATH: &str = ".solana-wallet/wallet.json";

impl WalletInfo {
    fn get_wallet_path() -> PathBuf {
        let home: String = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(WALLET_FILE_PATH)
    }

    // 获取 RPC 客户端
    fn get_rpc_client(network: SolanaNetwork) -> RpcClient {
        RpcClient::new(network.url().to_string())
    }

    /**
     * 存储钱包到钱包列表
     * 1. 拿到原有的钱包列表(最多不能超过5个)
     * 2. 更新钱包列表,新创建的钱包放在最上面.
     */
    pub fn save_to_file(&self) -> Result<(), String> {
        let wallet_path: PathBuf = Self::get_wallet_path();

        // 确保目录存在
        if let Some(parent) = wallet_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e: std::io::Error| format!("Failed to create directory: {}", e))?;
        }

        // 拿到原有的钱包列表(最多不能超过5个)
        let mut wallets: Vec<WalletInfo> = Self::load_from_file()?;
        if wallets.len() >= 5 {
            return Err("已达到最大钱包数量(5个), 无法创建新钱包。".to_string());
        }

        // 将新创建的钱包放在最前面
        wallets.insert(0, self.clone());

        // 序列化钱包信息为 JSON
        let json: String = serde_json::to_string_pretty(&wallets)
            .map_err(|e: serde_json::Error| format!("Failed to serialize wallet: {}", e))?;

        std::fs::write(&wallet_path, json)
            .map_err(|e: std::io::Error| format!("Failed to write wallet file: {}", e))?;

        Ok(())
    }

    pub fn refresh_balance() -> Result<Vec<Self>, String> {
        // 1. 获取钱包列表
        let mut wallets = Self::load_from_file()?;

        // 2. 使用多线程查询余额
        let handles: Vec<_> = wallets
            .iter()
            .map(|wallet| {
                let wallet_clone = wallet.clone();
                std::thread::spawn(move || {
                    let client = Self::get_rpc_client(wallet_clone.network);
                    match client.get_balance(&wallet_clone.public_key.parse().unwrap()) {
                        Ok(balance) => (wallet_clone.public_key.clone(), Some(balance)),
                        Err(_) => (wallet_clone.public_key.clone(), None),
                    }
                })
            })
            .collect();

        // 等待所有线程完成并收集结果
        let results: Vec<(String, Option<u64>)> = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();

        // 3. 更新钱包余额
        for wallet in wallets.iter_mut() {
            if let Some((_, balance)) = results
                .iter()
                .find(|(pubkey, _)| pubkey == &wallet.public_key)
            {
                wallet.balance = *balance;
            }
        }

        // 4. 保存更新后的钱包列表到文件
        let json = serde_json::to_string_pretty(&wallets)
            .map_err(|e| format!("Failed to serialize wallet: {}", e))?;

        let wallet_path = Self::get_wallet_path();
        std::fs::write(&wallet_path, json)
            .map_err(|e| format!("Failed to write wallet file: {}", e))?;

        Ok(wallets)
    }

    // 从文件加载钱包信息
    pub fn load_from_file() -> Result<Vec<Self>, String> {
        let wallet_path: PathBuf = Self::get_wallet_path();

        // 如果钱包文件不存在, 则返回空集合
        if !wallet_path.exists() {
            return Ok(Vec::new());
        }

        // 读取文件内容
        let json: String = std::fs::read_to_string(&wallet_path)
            .map_err(|e: std::io::Error| format!("无法读取钱包文件: {}", e))?;

        serde_json::from_str(&json).map_err(|e| format!("无法解析钱包文件: {}", e))
    }

    pub fn create_new_wallet(network: Option<SolanaNetwork>) -> Result<Vec<Self>, String> {
        // 读取已有钱包,如果已经有5个,则不允许创建新钱包
        let existing_wallets: Vec<WalletInfo> = WalletInfo::load_from_file()?;
        if existing_wallets.len() >= 5 {
            return Err("已达到最大钱包数量(5个), 无法创建新钱包。".to_string());
        }

        // 如果没有指定网络，默认使用 Devnet
        let network: SolanaNetwork = network.unwrap_or(SolanaNetwork::Devnet);
        // 生成新的密钥对
        let keypair: Keypair = Keypair::new();
        // 获取公钥
        let public_key: String = keypair.pubkey().to_string();
        // 获取私钥（转换为 base58 格式）
        let private_key: String = bs58::encode(keypair.to_bytes()).into_string();
        // 初始化 RPC 客户端（将来用于查询余额等操作）
        let _client: RpcClient = Self::get_rpc_client(network);

        let wallet_info: WalletInfo = WalletInfo {
            public_key,
            private_key,
            network,
            balance: Some(0), // 新创建的钱包余额为 0
            alias: None,      // 初始没有别名
        };

        // 保存钱包信息到本地文件
        wallet_info.save_to_file()?;

        WalletInfo::load_from_file()
    }

    pub fn query_by_public_key(public_key: &str) -> Result<Self, String> {
        let wallet_list = WalletInfo::load_from_file()?;
        for wallet in wallet_list {
            if wallet.public_key == public_key {
                return Ok(wallet);
            }
        }
        Err("未找到对应钱包".to_string())
    }

    pub fn set_alias(mut self, new_alias: &str) -> Result<Vec<Self>, String> {
        self.alias = Some(new_alias.to_string());

        // 读取所有钱包
        let mut wallets = Self::load_from_file()?;

        // 找到并更新对应钱包的别名
        for wallet in wallets.iter_mut() {
            if wallet.public_key == self.public_key {
                wallet.alias = Some(new_alias.to_string());
                break;
            }
        }

        // 将更新后的钱包列表保存回文件
        let json = serde_json::to_string_pretty(&wallets)
            .map_err(|e| format!("Failed to serialize wallet: {}", e))?;

        let wallet_path = Self::get_wallet_path();
        std::fs::write(&wallet_path, json)
            .map_err(|e| format!("Failed to write wallet file: {}", e))?;

        Ok(wallets)
    }

    pub fn delete_wallet(self) -> Result<Vec<Self>, String> {
        let mut wallets = Self::load_from_file()?;

        // 保留不匹配的钱包（即删除匹配的钱包）
        wallets.retain(|wallet| wallet.public_key != self.public_key);

        // 将更新后的钱包列表保存回文件
        let json = serde_json::to_string_pretty(&wallets)
            .map_err(|e| format!("Failed to serialize wallet: {}", e))?;

        let wallet_path = Self::get_wallet_path();
        std::fs::write(&wallet_path, json)
            .map_err(|e| format!("Failed to write wallet file: {}", e))?;

        Ok(wallets)
    }
}
