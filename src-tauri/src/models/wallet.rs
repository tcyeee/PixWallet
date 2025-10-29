use crate::models::{history::History, network::SolanaNetwork};
use crate::repository::wallet_repo::WalletRepository;
use {
    bs58,
    serde::{Deserialize, Serialize},
    solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    solana_system_interface::instruction::transfer,
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
    pub fn transfer(&self, recipient: Pubkey, amount: f32) -> Result<(), String> {
        println!("==================[START]==================");
        let client: RpcClient = SolanaNetwork::get_rpc_client(self.network);
        let sender: Keypair = self.keypair();
        let transfer_amount = (amount * LAMPORTS_PER_SOL as f32) as u64;

        let transfer_instruction = transfer(&sender.pubkey(), &recipient, transfer_amount);
        let mut transaction =
            Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));
        let blockhash = client.get_latest_blockhash().map_err(|e| e.to_string())?;
        transaction.sign(&[&sender], blockhash);

        // Send the transaction to the network
        let transaction_signature = client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| e.to_string())?;

        println!("=================");
        println!("Transaction Signature: {}", transaction_signature);
        println!("==================[END]==================");
        Ok(())
    }

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

    pub fn update(self, repo: &WalletRepository) {
        repo.update(self);
    }

    pub fn pubkey(&self) -> Result<Pubkey, String> {
        self.public_key
            .parse()
            .map_err(|e| format!("无效的公钥 ({}): {}", self.public_key, e))
    }

    /* 通过私钥, 获取用于转账的密钥对 */
    pub fn keypair(&self) -> Keypair {
        Keypair::from_base58_string(&self.private_key)
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

    pub fn query_all(repo: &WalletRepository) -> Vec<Self> {
        repo.all()
    }

    /**
     * 异步查询钱包余额
     * 查询完成以后,将会返回所有有变动的钱包信息
     */
    pub fn refresh_wallet(wallets: Vec<Wallet>) -> Result<Vec<Wallet>, String> {
        let results = Arc::new(Mutex::new(Vec::new()));

        let handles: Vec<_> = wallets
            .into_iter()
            .map(|wallet| {
                let results = Arc::clone(&results);
                thread::spawn(move || {
                    println!("[DEBUG] 正在查询账户: {}", wallet.public_key);
                    let balance = wallet.query_balance().unwrap_or_default();
                    println!("[DEBUG] 账户: {} 查询完毕", wallet.public_key);
                    if wallet.balance != Some(balance) {
                        let mut w = wallet;
                        w.balance = Some(balance);
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
}
