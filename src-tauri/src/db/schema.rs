pub const CREATE_WALLET_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS wallet (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        public_key TEXT NOT NULL UNIQUE,
        private_key TEXT NOT NULL,
        network TEXT NOT NULL,
        balance INTEGER DEFAULT 0,
        alias TEXT,
        created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
        updated_at INTEGER
    );
";

pub const CREATE_HISTORY_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS history (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        public_key TEXT NOT NULL,
        signature TEXT NOT NULL,
        slot INTEGER NOT NULL,
        err TEXT,
        memo TEXT,
        block_time INTEGER,
        confirmation_status TEXT,
        remark TEXT,
        created_at INTEGER DEFAULT (strftime('%s', 'now'))
    );
";

pub const  CREATE_TOKEN_PRICE_TABLE: &str = "
 CREATE TABLE IF NOT EXISTS token_price (
                symbol TEXT PRIMARY KEY,  --  设置为主键
                usd INTEGER NOT NULL,
                expo INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )
";
pub const TABLES: [&str; 3] = [CREATE_WALLET_TABLE, CREATE_HISTORY_TABLE, CREATE_TOKEN_PRICE_TABLE];
