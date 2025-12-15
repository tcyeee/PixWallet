
use std::{ 
    result::Result, sync::{Arc, Mutex}
};

use crate::{
    db::connection::DB_CONN,
    models::token_price::TokenPrice
};
use rusqlite::Connection;
 

pub struct TokenPriceRepository {
    conn: Arc<Mutex<Connection>>,
}

impl TokenPriceRepository {
    pub fn new() -> Self {
        let conn = DB_CONN.get().expect("数据库未初始化").clone(); // 拿到 Arc<Mutex<Connection>>
        Self { conn }
    }

    fn get_conn(&'_ self) -> Result<std::sync::MutexGuard<'_,rusqlite::Connection>, Box<dyn std::error::Error>> {
        match self.conn.lock() {
               Ok(guard) => Ok(guard),
               Err(e) => Err(format!("获取数据库连接失败: {}", e).into())
        }
    }
   
   pub fn get_multi(&self, symbols: &[String]) -> Vec<TokenPrice> {
    if symbols.is_empty() {
        return vec![];
    }

    let conn = match self.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            println!("获取数据库连接异常{}", e);
            return vec![];
        }
    };

    // 动态构建占位符 (?, ?, ?)
    let placeholders = symbols
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");

    let sql = format!(
        "SELECT symbol, usd, expo, updated_at 
         FROM token_price 
         WHERE symbol IN ({})",
        placeholders
    );

    let mut stmt = match conn.prepare(&sql) {
          Ok(stmt) => stmt,
          Err(e) => {
              println!("sql 异常{}", e);
              return vec![];
          }
    };

    let rows = stmt
        .query_map(rusqlite::params_from_iter(symbols.iter()), |row| {
            Ok(TokenPrice {
                symbol: row.get(0)?,
                usd: row.get(1)?,
                expo: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })
        .unwrap();

    rows.filter_map(|row| row.ok()).collect()
}



pub fn save_all(&self, prices: &[TokenPrice]) -> Result<(),  Box<dyn std::error::Error>> {
      // 获取连接
    let mut conn = self.conn.lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    
    // 开始事务
    let tx = conn.transaction()?;
    
    {
    // 关键：prepare 返回 Result，需要用 ? 解包
        let mut stmt = tx.prepare(
            "INSERT INTO token_price (symbol, usd, expo, updated_at)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(symbol) DO UPDATE SET 
                usd = excluded.usd,
                expo = excluded.expo,
                updated_at = excluded.updated_at"
        )?;  // 注意这个 ? 号！
        
        // 现在 stmt 是 Statement 类型，可以调用 execute
        for price in prices {
            stmt.execute(rusqlite::params![
                &price.symbol,
                price.usd,
                price.expo,
                price.updated_at
            ])?;  // 这个 execute 是 Statement 的方法
        }
    }
    
    
    tx.commit()?;
    
    println!("✅ 成功保存 {} 个价格", prices.len());
    
    Ok(())
}

}
