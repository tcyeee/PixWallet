use super::schema::TABLES;
use once_cell::sync::OnceCell;
use rusqlite::{Connection, Result};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub static DB_CONN: OnceCell<Arc<Mutex<Connection>>> = OnceCell::new();

pub fn establish_connection(path: PathBuf) {
    std::fs::create_dir_all(&path).expect("Failed to create directory");
    let db_path = path.join("wallet.db");
    println!("📁 Database full path: {}", db_path.display());

    // 初始化数据库
    let conn: Connection = Connection::open(&db_path).expect("Failed to open database");
    init_tables(&conn).expect("Failed to initialize database tables");
    println!("✅ Database initialized successfully!");

    // 设置全局 OnceCell
    let conn_state = Arc::new(Mutex::new(conn));
    DB_CONN
        .set(conn_state.clone())
        .expect("Database has been initialized");
}

pub fn init_tables(conn: &Connection) -> Result<(), String> {
    for sql in TABLES {
        conn.execute(sql, []).map_err(|e| e.to_string())?;
    }
    Ok(())
}
