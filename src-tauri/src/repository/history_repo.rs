use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::{
    db::connection::DB_CONN,
    models::history::{History, Status},
};
use rusqlite::{params, Connection};

pub struct HistoryRepository {
    conn: Arc<Mutex<Connection>>,
}

impl HistoryRepository {
    pub fn new() -> Self {
        let conn = DB_CONN.get().expect("数据库未初始化").clone(); // 拿到 Arc<Mutex<Connection>>
        Self { conn }
    }

    fn get_conn(&'_ self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().expect("锁数据库失败")
    }

    pub fn insert_batch(&self, list: Vec<History>) -> Result<(), String> {
        let mut conn = self.get_conn();
        let tx = conn.transaction().map_err(|e| e.to_string())?;

        for history in list {
            tx.execute(
                "
                INSERT INTO history (
                    public_key,
                    signature,
                    slot,
                    err,
                    memo,
                    block_time,
                    confirmation_status,
                    remark
                )
                VALUES
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                ",
                params![
                    history.public_key,
                    history.signature,
                    history.slot,
                    history.err,
                    history.memo,
                    history.block_time,
                    history.confirmation_status.map(|x| x.to_string()),
                    history.remark,
                ],
            )
            .map_err(|e| e.to_string())?;
        }
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn list(&self, public_key: &str, page: usize, page_size: usize) -> Vec<History> {
        let offset = (page - 1) * page_size;
        let conn = self.get_conn();
        let mut stmt = conn
            .prepare(
                "
                SELECT
                    public_key,
                    signature,
                    slot,
                    err,
                    memo,
                    block_time,
                    confirmation_status,
                    remark,
                    created_at
                FROM
                    history
                WHERE
                    public_key = ?
                ORDER BY
                    created_at DESC
                    LIMIT ? OFFSET ?
        ",
            )
            .unwrap();

        let rows = stmt
            .query_map((public_key, page_size, offset), |row| {
                let status_str: String = row.get(6)?;
                Ok(History {
                    public_key: row.get(0)?,
                    signature: row.get(1)?,
                    slot: row.get(2)?,
                    err: row.get(3)?,
                    memo: row.get(4)?,
                    block_time: row.get(5)?,
                    confirmation_status: Some(Status::from_str(&status_str).unwrap()),
                    remark: row.get(7)?,
                    created_at: row.get(8)?,
                })
            })
            .unwrap();
        rows.map(|r| r.unwrap()).collect()
    }

    pub fn count(&self, public_key: &str) -> Result<usize, rusqlite::Error> {
        let conn = self.get_conn();
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM history WHERE public_key = ? ")?;

        let count: i64 = stmt.query_row((public_key,), |row| row.get(0))?;
        Ok(count as usize)
    }
}
