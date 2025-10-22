use super::schema::CREATE_WALLET_INFO_TABLE;
use rusqlite::Connection;

pub fn create_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(&format!("{}\n", CREATE_WALLET_INFO_TABLE))?;
    Ok(())
}
