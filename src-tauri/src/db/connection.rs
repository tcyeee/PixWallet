use super::schema::TABLES;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub fn establish_connection() -> Result<Mutex<Connection>, rusqlite::Error> {
    // 初始化数据库
    let conn: Connection = Connection::open("wallet.db")?;
    init_tables(&conn).expect("Failed to initialize database tables");
    println!("✅ Database initialized successfully!");

    // 使用 Mutex 封装 Connection，供前端 handler 使用
    let conn_state: Mutex<_> = Mutex::new(conn);
    Ok(conn_state)
}

pub fn init_tables(conn: &Connection) -> Result<(), String> {
    for sql in TABLES {
        conn.execute(sql, []).map_err(|e| e.to_string())?;
    }
    Ok(())
}
