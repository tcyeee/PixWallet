use crate::models::network::SolanaNetwork;
use crate::models::wallet::WalletInfo;

// 创建新的钱包
#[tauri::command]
pub fn create_wallet(network: Option<SolanaNetwork>) -> Result<Vec<WalletInfo>, String> {
    WalletInfo::create_new_wallet(network)
}

// 查询钱包信息
#[tauri::command]
pub fn query_wallet() -> Result<Vec<WalletInfo>, String> {
    WalletInfo::load_from_file()
}

#[tauri::command]
pub fn change_alias(public_key: &str, new_alias: &str) -> Result<Vec<WalletInfo>, String> {
    let wallet: WalletInfo = WalletInfo::query_by_public_key(public_key)?;
    wallet.set_alias(new_alias)
}

#[tauri::command]
pub fn delete_wallet(public_key: &str) -> Result<Vec<WalletInfo>, String> {
    let wallet: WalletInfo = WalletInfo::query_by_public_key(public_key)?;
    wallet.delete_wallet()
}

#[tauri::command]
pub fn refresh_balance() -> Result<Vec<WalletInfo>, String> {
    WalletInfo::refresh_balance()
}
