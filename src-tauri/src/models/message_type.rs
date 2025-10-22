#[derive(Debug)]
pub enum MsgType {
    BalanceRefreshEnd,
    BalanceChange,
}

impl MsgType {
    pub fn name(&self) -> &'static str {
        match self {
            MsgType::BalanceRefreshEnd => "BALANCE_REFRESH_END",
            MsgType::BalanceChange => "BALANCE_CHANGE",
        }
    }
}
