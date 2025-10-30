use crate::service::notice;
use crate::try_notice;
use crate::{
    models::network::SolanaNetwork,
    repository::wallet_repo::WalletRepository,
    service::notice::{msg, show, MsgType, NoticeType},
};
use {
    bs58,
    serde::{Deserialize, Serialize},
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
    std::{
        sync::{Arc, Mutex},
        thread,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
    pub network: SolanaNetwork,
    pub balance: Option<u64>,
    pub alias: Option<String>,
    pub updated_at: Option<i64>,
    pub created_at: Option<i64>,
}

impl Wallet {
    pub fn insert(&self, repo: &WalletRepository) -> Result<(), rusqlite::Error> {
        repo.insert(&self);
        Ok(())
    }

    pub fn query_by_public_key(repo: &WalletRepository, public_key: &str) -> Self {
        repo.get_by_pub_key(public_key)
    }

    pub fn del(&self, repo: &WalletRepository) -> Result<(), String> {
        if self.query_balance()? != 0 {
            return Err("余额不为0,禁止删除".to_string());
        }
        repo.del(&self.public_key);
        Ok(())
    }

    pub fn pubkey(&self) -> Result<Pubkey, String> {
        self.public_key
            .parse()
            .map_err(|e| format!("无效的公钥 ({}): {}", self.public_key, e))
    }

    pub fn query_balance(&self) -> Result<u64, String> {
        let balance = SolanaNetwork::get_rpc_client(self.network)
            .get_balance(&self.pubkey()?)
            .map_err(|e| format!("查询余额失败: {}", e))?;
        Ok(balance)
    }

    pub fn new(repo: &WalletRepository, network: Option<SolanaNetwork>) -> Result<Self, String> {
        // 读取已有钱包,如果已经有5个,则不允许创建新钱包
        let existing_wallets: Vec<Wallet> = repo.all();

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
        let _client: RpcClient = SolanaNetwork::get_rpc_client(network);

        let wallet_info: Wallet = Wallet {
            public_key,
            private_key,
            network,
            balance: Some(0), // 新创建的钱包余额为 0
            alias: None,      // 初始没有别名
            updated_at: Some(0),
            created_at: Some(0),
        };
        Ok(wallet_info)
    }

    /**
     * 异步查询钱包余额
     * 查询完成以后,将会返回所有有变动的钱包信息
     */
    pub fn refresh_wallets(wallets: Vec<Wallet>) -> Result<Vec<Wallet>, String> {
        let results = Arc::new(Mutex::new(Vec::new()));

        let handles: Vec<_> = wallets
            .into_iter()
            .map(|wallet| {
                let results = Arc::clone(&results);
                thread::spawn(move || {
                    let name = wallet
                        .alias
                        .clone()
                        .unwrap_or(format!("{}...", &wallet.public_key[0..10]));

                    let content = format!("账户{:?}更新完成..", name);
                    let balance = match wallet.query_balance() {
                        Ok(x) => {
                            println!("[DEBUG] 账户: {} 查询完毕", wallet.public_key);
                            show(NoticeType::Success, &content);
                            x
                        }
                        Err(_) => {
                            let content = format!("账户{:?}更新失败..", name);
                            show(NoticeType::Error, &content);
                            return;
                        }
                    };

                    if wallet.balance.unwrap_or(0) != balance {
                        let mut w = wallet;
                        w.balance = Some(balance);
                        msg(MsgType::BalanceChange, &w);
                        results.lock().unwrap().push(w);
                    }
                })
            })
            .collect();

        // 等待所有线程结束
        for handle in handles {
            let _ = handle.join();
        }

        Arc::try_unwrap(results)
            .unwrap()
            .into_inner()
            .map_err(|e| e.to_string())
    }

    // 刷新单个账户余额,同时通知到前端
    pub fn refresh_balance(&mut self) {
        let new_balance = match self.query_balance() {
            Ok(x) => x,
            Err(e) => {
                notice::show(NoticeType::Error, &e);
                return;
            }
        };

        if self.balance.unwrap_or(0) == new_balance {
            return;
        }

        self.balance = Some(new_balance);
        let repo = WalletRepository::new();
        repo.update(self.clone());
    }
}
