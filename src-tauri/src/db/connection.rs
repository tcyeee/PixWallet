use super::schema::TABLES;
use once_cell::sync::OnceCell;
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

pub static DB_CONN: OnceCell<Arc<Mutex<Connection>>> = OnceCell::new();

pub fn establish_connection() {
    // 初始化数据库
    let conn: Connection = Connection::open("wallet.db").unwrap();
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
