use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenPrice {
    pub symbol: String,
    pub usd: i64,        // 价格 * 1e6
    pub expo: i64, //小数位数
    pub updated_at: i64, // 时间戳
}