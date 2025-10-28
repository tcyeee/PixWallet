use crate::models::dto::TransferParams;
use crate::models::wallet::Wallet;
use crate::models::{message_type::MsgType, network::SolanaNetwork};
use crate::repository::wallet_repo::WalletRepository;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub fn query_wallet(conn_state: State<'_, Mutex<Connection>>) -> Vec<Wallet> {
    let conn = conn_state.lock().unwrap();
    let repo = WalletRepository::new(&conn);
    Wallet::query_all(&repo)
}

#[tauri::command]
pub fn create_wallet(
    conn_state: State<'_, Mutex<Connection>>,
    network: Option<SolanaNetwork>,
) -> Result<Wallet, String> {
    let conn = conn_state.lock().unwrap();
    let repo = WalletRepository::new(&conn);

    let wallet = Wallet::new(&repo, network)?;
    wallet.insert(&repo).map_err(|e| e.to_string())?;
    Ok(wallet)
}

#[tauri::command]
pub fn change_alias(
    conn_state: State<'_, Mutex<Connection>>,
    public_key: &str,
    new_alias: &str,
) -> Vec<Wallet> {
    let conn = conn_state.lock().unwrap();
    let repo = WalletRepository::new(&conn);
    let mut wallet = Wallet::query_by_public_key(&repo, public_key);
    wallet.alias = Some(new_alias.to_string());
    wallet.update(&repo);
    Wallet::query_all(&repo)
}

#[tauri::command]
pub fn delete_wallet(
    conn_state: State<'_, Mutex<Connection>>,
    public_key: &str,
) -> Result<(), String> {
    let conn = conn_state.lock().unwrap();
    let repo = WalletRepository::new(&conn);
    Wallet::query_by_public_key(&repo, public_key).del(&repo)
}

// 异步刷新余额
#[tauri::command]
pub async fn refresh_balance(
    conn_state: State<'_, Mutex<Connection>>,
    app: AppHandle,
) -> Result<(), String> {
    let conn = conn_state.lock().unwrap();
    let repo = WalletRepository::new(&conn);
    // 用户的全部账户
    let wallets = Wallet::query_all(&repo);
    // 多线程刷新余额
    let wallets = Wallet::refresh_wallet(wallets)?;
    // 挨个更新余额变动的账户
    wallets.iter().for_each(|x: &Wallet| {
        x.clone().update(&repo);
        // 通知前端某个账户更新
        app.emit(MsgType::BalanceChange.name(), x).unwrap();
        ()
    });
    // 通知前端所有的查询均已结束
    app.emit(MsgType::BalanceRefreshEnd.name(), ()).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn transfer(
    conn_state: State<'_, Mutex<Connection>>,
    params: TransferParams,
) -> Result<(), String> {
    let conn = conn_state.lock().unwrap();
    let repo = WalletRepository::new(&conn);

    let wallet = Wallet::query_by_public_key(&repo, &params.from);
    let receiving = Wallet::query_by_public_key(&repo, &params.to);
    let receiving_public_key = receiving.pubkey()?;
    wallet.transfer(receiving_public_key, params.amount)?;
    Ok(())
}

#[tauri::command]
pub async fn account_history(
    conn_state: State<'_, Mutex<Connection>>,
    public_key: &str,
) -> Result<(), String> {
    let conn = conn_state.lock().unwrap();
    let repo = WalletRepository::new(&conn);
    let wallet = Wallet::query_by_public_key(&repo, public_key);
    wallet.history()
}
