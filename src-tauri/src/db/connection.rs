use super::migrations::create_tables;
use rusqlite::{Connection, Result};
use std::path::Path;
use std::sync::Mutex;

pub fn establish_connection() -> Result<Mutex<Connection>, rusqlite::Error> {
    // ✅ 数据库路径
    let db_path: &'static str = "wallet.db";
    let first_run: bool = !Path::new(db_path).exists();

    // ✅ 建立数据库连接
    let conn: Connection = Connection::open("wallet.db")?;

    // ✅ 首次运行时初始化表结构
    if first_run {
        create_tables(&conn).expect("Failed to initialize database tables");
        println!("✅ Database initialized successfully!");
    } else {
        println!("📁 Database already exists, skipping initialization.");
    }

    // ✅ 使用 Mutex 封装 Connection，供前端 handler 使用
    let conn_state: Mutex<_> = Mutex::new(conn);

    Ok(conn_state)
}
