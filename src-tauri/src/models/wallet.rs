use crate::models::network::SolanaNetwork;
use bs58;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::transfer;
use std::{
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletInfo {
    pub public_key: String,
    pub private_key: String,
    pub network: SolanaNetwork,
    pub balance: Option<u64>,
    pub alias: Option<String>,
}

impl WalletInfo {
    pub fn transfer(&self, recipient: Pubkey, amount: f32) -> Result<(), String> {
        println!("==================[START]==================");
        let connection: RpcClient = SolanaNetwork::get_rpc_client(self.network);
        let sender: Keypair = self.keypair();
        let transfer_amount = (amount * LAMPORTS_PER_SOL as f32) as u64;

        let transfer_instruction = transfer(&sender.pubkey(), &recipient, transfer_amount);
        let mut transaction =
            Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));
        let blockhash = connection
            .get_latest_blockhash()
            .map_err(|e| e.to_string())?;
        transaction.sign(&[&sender], blockhash);

        // Send the transaction to the network
        let transaction_signature = connection
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| e.to_string())?;

        println!("=================");
        println!("Transaction Signature: {}", transaction_signature);
        println!("==================[END]==================");
        Ok(())
    }

    pub fn insert(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "insert into wallet(public_key, private_key, network, balance, alias) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![self.public_key,self.private_key,self.network.to_string(),self.balance,self.alias],
        )?;
        Ok(())
    }

    pub fn query_by_public_key(conn: &Connection, public_key: &str) -> Result<Self, String> {
        conn.query_row(
            "select public_key, private_key, network, balance, alias from wallet where public_key = ?1 limit 1",
            params![public_key], |row|{
                let network_str: String = row.get(2)?;
                Ok(WalletInfo{
                    public_key: row.get(0)?,
                    private_key: row.get(1)?,
                    network: SolanaNetwork::from_str(&network_str) ,
                    balance: row.get(3)?,
                    alias: row.get(4)?,
                })
            }
        )
        .map_err(|_| "未找到对应钱包".to_string())
    }

    pub fn del(&self, conn: &Connection) -> Result<(), String> {
        if self.query_balance()? != 0 {
            return Err("余额不为0,禁止删除".to_string());
        }

        conn.execute(
            "DELETE FROM wallet WHERE public_key = ?1",
            params![&self.public_key],
        )
        .map(|_| ())
        .map_err(|_| "删除失败".to_string())
    }

    pub fn update(&self, conn: &Connection) {
        let info = conn
            .execute(
                "update wallet set alias = ?1, balance = ?2 where public_key = ?3",
                params![self.alias, self.balance, self.public_key],
            )
            .map_err(|_| "更新出错".to_string());

        match info {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
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

    pub fn new(conn: &Connection, network: Option<SolanaNetwork>) -> Result<Self, String> {
        // 读取已有钱包,如果已经有5个,则不允许创建新钱包
        let existing_wallets: Vec<WalletInfo> =
            Self::query_all(&conn).map_err(|e| e.to_string())?;

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

        let wallet_info: WalletInfo = WalletInfo {
            public_key,
            private_key,
            network,
            balance: Some(0), // 新创建的钱包余额为 0
            alias: None,      // 初始没有别名
        };
        Ok(wallet_info)
    }

    pub fn query_all(conn: &Connection) -> Result<Vec<Self>, String> {
        let mut stmt = conn
            .prepare(
                "
            select public_key, private_key, network, balance, alias
            from wallet
            limit 10
        ",
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                let network_str: String = row.get(2)?;
                Ok(WalletInfo {
                    public_key: row.get(0)?,
                    private_key: row.get(1)?,
                    network: SolanaNetwork::from_str(&network_str),
                    balance: row.get(3)?,
                    alias: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let wallets: Vec<WalletInfo> = rows
            .map(|r| r.map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?;

        Ok(wallets)
    }

    /**
     * 异步查询钱包余额
     * 查询完成以后,将会返回所有有变动的钱包信息
     */
    pub fn refresh_wallet(wallets: Vec<WalletInfo>) -> Result<Vec<WalletInfo>, String> {
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
