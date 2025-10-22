use crate::models::network::SolanaNetwork;
use crate::models::wallet::WalletInfo;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

// 查询钱包信息
#[tauri::command]
pub fn query_wallet(conn_state: State<'_, Mutex<Connection>>) -> Result<Vec<WalletInfo>, String> {
    let conn = conn_state.lock().unwrap();
    WalletInfo::query_all(&conn)
}

#[tauri::command]
pub fn create_wallet(
    conn_state: State<'_, Mutex<Connection>>,
    network: Option<SolanaNetwork>,
) -> Result<Vec<WalletInfo>, String> {
    let conn = conn_state.lock().unwrap();
    let wallet = WalletInfo::new(&conn, network)?;
    wallet.insert(&conn).map_err(|e| e.to_string())?;

    WalletInfo::query_all(&conn)
}

#[tauri::command]
pub fn change_alias(
    conn_state: State<'_, Mutex<Connection>>,
    public_key: &str,
    new_alias: &str,
) -> Result<Vec<WalletInfo>, String> {
    let conn = conn_state.lock().unwrap();
    let mut wallet = WalletInfo::query_by_public_key(&conn, public_key)?;
    wallet.alias = Some(new_alias.to_string());
    match wallet.update(&conn) {
        Ok(_) => {}
        Err(e) => return Err(e),
    };
    WalletInfo::query_all(&conn)
}

#[tauri::command]
pub fn delete_wallet(
    conn_state: State<'_, Mutex<Connection>>,
    public_key: &str,
) -> Result<Vec<WalletInfo>, String> {
    let conn = conn_state.lock().unwrap();
    match WalletInfo::query_by_public_key(&conn, public_key)?.del(&conn) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    WalletInfo::query_all(&conn)
}

// 异步刷新余额
#[tauri::command]
pub fn refresh_balance(conn_state: State<'_, Mutex<Connection>>, app: AppHandle) {
    let conn = conn_state.lock().unwrap();
    WalletInfo::refresh_all_balance(&conn).iter().for_each(|x| {
        app.emit("refresh_balance", x).unwrap();
    });
}
