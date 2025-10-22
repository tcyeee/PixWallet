use rusqlite::{Connection, Result};

pub fn establish_connection() -> Result<Connection> {
    let conn = Connection::open("wallet.db")?;
    Ok(conn)
}
