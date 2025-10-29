use crate::models::{
    dto::TransferParams, history::History, network::SolanaNetwork, wallet::Wallet,
};
use crate::repository::history_repo::HistoryRepository;
use crate::repository::wallet_repo::WalletRepository;
use crate::service::notice::msg;
use crate::service::notice::MsgType;
use crate::service::rpc::history_update;

#[tauri::command]
pub fn query_wallet() -> Vec<Wallet> {
    let repo = WalletRepository::new();
    Wallet::query_all(&repo)
}

#[tauri::command]
pub fn create_wallet(network: Option<SolanaNetwork>) -> Result<Wallet, String> {
    let repo = WalletRepository::new();
    let wallet = Wallet::new(&repo, network)?;
    wallet.insert(&repo).map_err(|e| e.to_string())?;
    Ok(wallet)
}

#[tauri::command]
pub fn change_alias(public_key: &str, new_alias: &str) -> Vec<Wallet> {
    let repo = WalletRepository::new();
    let mut wallet = Wallet::query_by_public_key(&repo, public_key);
    wallet.alias = Some(new_alias.to_string());
    wallet.update(&repo);
    Wallet::query_all(&repo)
}

#[tauri::command]
pub fn delete_wallet(public_key: &str) -> Result<(), String> {
    let repo = WalletRepository::new();
    Wallet::query_by_public_key(&repo, public_key).del(&repo)
}

// 异步刷新余额
#[tauri::command]
pub async fn refresh_balance() -> Result<(), String> {
    let repo = WalletRepository::new();
    // 用户的全部账户
    let wallets = Wallet::query_all(&repo);
    // 多线程刷新余额
    let wallets = Wallet::refresh_wallet(wallets)?;

    // 挨个更新余额变动的账户
    wallets
        .iter()
        .for_each(|x: &Wallet| x.clone().update(&repo));
    msg(MsgType::BalanceRefreshEnd, ());
    Ok(())
}

#[tauri::command]
pub async fn transfer(params: TransferParams) -> Result<(), String> {
    let repo = WalletRepository::new();

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
pub async fn account_history(public_key: String) -> Result<Vec<History>, String> {
    let repo = HistoryRepository::new();
    let list = repo.list(&public_key);
    let list_clone = list.clone();
    tauri::async_runtime::spawn_blocking(move || {
        match history_update(&list_clone, &public_key, SolanaNetwork::Devnet) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("{}", e)
            }
        };
    });

    Ok(list)
}
