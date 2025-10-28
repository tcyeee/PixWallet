pub const CREATE_WALLET_INFO_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS wallet (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        public_key TEXT NOT NULL UNIQUE,
        private_key TEXT NOT NULL,
        network TEXT NOT NULL,
        balance INTEGER,
        alias TEXT,
        last_update INTEGER
    );
";
