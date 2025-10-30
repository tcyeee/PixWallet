use std::sync::{Arc, Mutex};

use crate::models::wallet::Wallet;
use crate::{db::connection::DB_CONN, models::network::SolanaNetwork};
use rusqlite::{params, Connection};

pub struct WalletRepository {
    conn: Arc<Mutex<Connection>>,
}

impl WalletRepository {
    pub fn new() -> Self {
        let conn = DB_CONN.get().expect("数据库未初始化").clone(); // 拿到 Arc<Mutex<Connection>>
        Self { conn }
    }

    fn get_conn(&'_ self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().expect("锁数据库失败")
    }

    pub fn insert(&self, wallet: &Wallet) {
        let conn = self.get_conn();
        conn.execute(
            "
                INSERT INTO wallet(
                    public_key, private_key, network, balance, alias, created_at, updated_at
                ) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                wallet.public_key,
                wallet.private_key,
                wallet.network.to_string(),
                wallet.balance,
                wallet.alias,
                wallet.created_at,
                wallet.updated_at,
            ],
        )
        .unwrap();
    }

    pub fn get_by_pub_key(&self, public_key: &str) -> Wallet {
        let conn = self.get_conn();
        conn.query_row(
            "
                SELECT 
                    public_key, private_key, network, balance, alias, created_at, updated_at
                FROM wallet 
                WHERE public_key = ?1 
                LIMIT 1
            ",
            params![public_key],
            |row| {
                let network_str: String = row.get(2)?;
                Ok(Wallet {
                    public_key: row.get(0)?,
                    private_key: row.get(1)?,
                    network: SolanaNetwork::from_str(&network_str),
                    balance: row.get(3)?,
                    alias: row.get(4)?,
                    updated_at: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )
        .unwrap()
    }

    pub fn del(&self, public_key: &str) {
        let conn = self.get_conn();
        conn.execute(
            "
                DELETE FROM wallet 
                WHERE public_key = ?1
                ",
            params![public_key],
        )
        .unwrap();
    }

    pub fn update(&self, wallet: Wallet) {
        let conn = self.get_conn();
        conn.execute(
            "
                UPDATE wallet SET 
                    alias = ?1, 
                    balance = ?2 
                WHERE
                    public_key = ?3
                ",
            params![wallet.alias, wallet.balance, wallet.public_key],
        )
        .unwrap();
    }

    pub fn all(&self) -> Vec<Wallet> {
        let conn = self.get_conn();
        let mut stmt = conn
            .prepare(
                "
        SELECT
            public_key, private_key, network, balance, alias, updated_at, created_at
        FROM wallet
        LIMIT 50
        ",
            )
            .unwrap();

        let rows = stmt
            .query_map([], |row| {
                let network_str: String = row.get(2)?;
                Ok(Wallet {
                    public_key: row.get(0)?,
                    private_key: row.get(1)?,
                    network: SolanaNetwork::from_str(&network_str),
                    balance: row.get(3)?,
                    alias: row.get(4)?,
                    updated_at: row.get(5)?,
                    created_at: row.get(6)?,
                })
            })
            .unwrap();

        rows.map(|r| r.unwrap()).collect()
    }
}
