use crate::models::wallet::WalletInfo;
use crate::models::{message_type::MsgType, network::SolanaNetwork};
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
) -> Result<WalletInfo, String> {
    let conn = conn_state.lock().unwrap();
    let wallet = WalletInfo::new(&conn, network)?;
    wallet.insert(&conn).map_err(|e| e.to_string())?;
    Ok(wallet)
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
    wallet.update(&conn);
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
pub async fn refresh_balance(
    conn_state: State<'_, Mutex<Connection>>,
    app: AppHandle,
) -> Result<(), String> {
    let conn = conn_state.lock().unwrap();
    // 用户的全部账户
    let wallets = WalletInfo::query_all(&conn)?;
    // 多线程刷新余额
    let wallets = WalletInfo::refresh_wallet(wallets)?;
    // 挨个更新余额变动的账户
    wallets.iter().for_each(|x| {
        x.update(&conn);
        // 通知前端某个账户更新
        app.emit(MsgType::BalanceChange.name(), x).unwrap();
        ()
    });
    // 通知前端所有的查询均已结束
    app.emit(MsgType::BalanceRefreshEnd.name(), ()).unwrap();
    Ok(())
}
