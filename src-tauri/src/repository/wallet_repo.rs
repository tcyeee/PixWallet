use crate::models::network::SolanaNetwork;
use crate::models::wallet::Wallet;
use rusqlite::{params, Connection};

pub struct WalletRepository<'a> {
    conn: &'a Connection,
}

impl<'a> WalletRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn insert(&self, wallet: &Wallet) {
        self.conn
            .execute(
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
        self.conn
            .query_row(
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
        self.conn
            .execute(
                "
                DELETE FROM wallet 
                WHERE public_key = ?1
                ",
                params![public_key],
            )
            .unwrap();
    }

    pub fn update(&self, wallet: Wallet) {
        self.conn
            .execute(
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
        let mut stmt = self
            .conn
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
