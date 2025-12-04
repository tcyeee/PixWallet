use crate::models::history::HistoryQuery;
use crate::models::history::PaginatedHistory;
use crate::models::{dto::TransferParams, network::SolanaNetwork, wallet::Wallet};
use crate::repository::history_repo::HistoryRepository;
use crate::repository::wallet_repo::WalletRepository;
use crate::service::notice::MsgType;
use crate::service::notice::{msg, show, NoticeType};
use crate::service::rpc::{self, history_update};

#[tauri::command]
pub fn query_wallet() -> Vec<Wallet> {
    let repo = WalletRepository::new();
    repo.all()
}

#[tauri::command]
pub async fn create_wallet(network: Option<SolanaNetwork>) -> Result<Wallet, String> {
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
    repo.update(wallet);
    repo.all()
}

#[tauri::command]
pub async fn delete_wallet(public_key: &str) -> Result<(), String> {
    let repo = WalletRepository::new();
    Wallet::query_by_public_key(&repo, public_key).del(&repo)
}

// 异步刷新余额
#[tauri::command]
pub async fn refresh_balance() -> Result<(), String> {
    let repo = WalletRepository::new();
    // 用户的全部账户
    let wallets = repo.all();
    // 多线程刷新余额
    let wallets = Wallet::refresh_wallets(wallets)?;

    // 挨个更新余额变动的账户
    wallets.iter().for_each(|x: &Wallet| repo.update(x.clone()));

    msg(MsgType::BalanceRefreshEnd, ());
    Ok(())
}

#[tauri::command]
pub async fn transfer(params: TransferParams) -> Result<(), String> {
    let repo = WalletRepository::new();
    let wallet = Wallet::query_by_public_key(&repo, &params.from);
    rpc::transfer(wallet, params.to, params.amount);
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
pub async fn account_history(query: HistoryQuery) -> Result<PaginatedHistory, String> {
    let repo = HistoryRepository::new();
    let list = repo.list(&query.public_key, query.page, query.page_size);
    let total = repo.count(&query.public_key).map_err(|e| e.to_string())?;

    // 如果页码大于1,则无需查询线上历史。
    if query.page > 1 {
        return Ok(PaginatedHistory { total, list });
    }

    // 获取总数
    let list_clone = list.clone();
    tauri::async_runtime::spawn_blocking(move || {
        match history_update(&list_clone, &query.public_key, SolanaNetwork::Devnet) {
            Ok(()) => show(NoticeType::Success, "同步完成"),
            Err(e) => {
                show(NoticeType::Error, &format!("网络同步失败:{}", e));
                eprintln!("{}", e)
            }
        };
    });

    Ok(PaginatedHistory { total, list })
}

#[tauri::command]
pub async fn transfer_detail(_signature: &str) -> Result<(), String> {
    // rpc::transfer_detail(signature)
    Ok(())
}
