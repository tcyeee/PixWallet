#[derive(Debug)]
pub enum MsgType {
    Wallets,
    Balance,
}

impl MsgType {
    pub fn name(&self) -> &'static str {
        match self {
            MsgType::Wallets => "WALLETS",
            MsgType::Balance => "BALANCE",
        }
    }
}
