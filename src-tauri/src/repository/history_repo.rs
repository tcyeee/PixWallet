use std::str::FromStr;

use crate::models::history::{History, Status};
use rusqlite::Connection;

pub struct HistoryRepository<'a> {
    conn: &'a Connection,
}

impl<'a> HistoryRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn list(&self, public_key: &str) -> Vec<History> {
        let mut stmt = self
            .conn
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
                    public_key = ?1
                ORDER BY
                    created_at DESC
        ",
            )
            .unwrap();

        let rows = stmt
            .query_map([public_key], |row| {
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
}
