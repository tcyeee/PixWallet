use crate::models::{
    dto::TransferParams, history::History, message_type::MsgType, network::SolanaNetwork,
    wallet::Wallet,
};
use crate::repository::history_repo::HistoryRepository;
use crate::repository::wallet_repo::WalletRepository;
use crate::service::rpc::history_update;
use {
    rusqlite::Connection,
    std::sync::Arc,
    std::sync::Mutex,
    tauri::{AppHandle, Emitter, State},
    tokio::task,
};

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

/**
 * 查询钱包动账历史
 * 1.分别查询本地历史,和线上历史,拿到本地历史以后立即返回.
 * 2.优先显示本地历史,等待线上历史查询结束后,将其合并
 *
 * #合并步骤
 * 拿到本地List最新的一条的时间,在线上数据中,拿到所有大于改时间的信息
 * 将更新数据存储到数据库,同时异步通知到前端.
 */
#[tauri::command]
pub async fn account_history(
    conn_state: State<'_, Arc<Mutex<Connection>>>,
    public_key: String,
) -> Result<Vec<History>, String> {
    // 拿到本地历史
    let conn_state = conn_state.inner().clone();
    let public_key_clone = public_key.clone();
    let last_list = task::spawn_blocking(move || -> Result<Vec<History>, String> {
        let conn = conn_state
            .lock()
            .map_err(|e| format!("锁数据库失败: {}", e))?;
        let repo = HistoryRepository::new(&conn);
        Ok(repo.list(&public_key_clone))
    })
    .await
    .map_err(|e| e.to_string())??;

    // 查询线上历史
    history_update(&last_list, public_key, SolanaNetwork::Devnet).await;

    Ok(last_list)
}
