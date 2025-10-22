use crate::models::network::SolanaNetwork;
use crate::models::wallet::WalletInfo;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;
use tauri::{AppHandle, Emitter};

// 查询钱包信息
#[tauri::command]
pub fn query_wallet(conn_state: State<'_, Mutex<Connection>>) -> Result<Vec<WalletInfo>, String> {
    let conn = conn_state.lock().unwrap();
    WalletInfo::query_all(&conn)
}

// 创建新的钱包
#[tauri::command]
pub fn create_wallet(
    conn_state: State<'_, Mutex<Connection>>,
    network: Option<SolanaNetwork>,
) -> Result<Vec<WalletInfo>, String> {
    let conn = conn_state.lock().unwrap();
    let wallet = WalletInfo::new(network)?;
    wallet.insert(&conn).map_err(|e| e.to_string())?;
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

// 异步刷新余额
#[tauri::command]
pub fn refresh_balance(app: AppHandle) {
    std::thread::spawn(move || {
        // TODO 添加全局错误弹窗
        let wallets = WalletInfo::refresh_balance().unwrap();
        app.emit("refresh_balance", wallets).unwrap();
    });
}
