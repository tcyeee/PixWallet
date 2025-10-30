use crate::{
    models::{history::History, network::SolanaNetwork},
    repository::history_repo::HistoryRepository,
    service::notice::{self, show, NoticeType},
};
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_sdk::pubkey::Pubkey;

pub fn history_update(
    history_list: &Vec<History>,
    public_key: &str,
    network: SolanaNetwork,
) -> Result<(), String> {
    show(NoticeType::Info, "正在同步Solana网络...");

    let client: RpcClient = SolanaNetwork::get_rpc_client(network);
    let pubkey: Pubkey = get_public_key_by_str(&public_key)?;
    let signatures = client
        .get_signatures_for_address_with_config(
            &pubkey,
            GetConfirmedSignaturesForAddress2Config {
                limit: Some(100), // 默认最多1000
                before: None,
                until: None,
                commitment: None,
            },
        )
        .map_err(|e| e.to_string())?;

    if signatures.is_empty() {
        return Ok(());
    };

    // 计算数据库中,最近的Block时间(可能为空)
    let mut last_block_time: i64 = 0_i64;
    if !history_list.is_empty() {
        last_block_time = match history_list.get(0) {
            Some(item) => item.block_time.unwrap_or(0),
            None => 0_i64,
        };
    }

    let mut new_history: Vec<History> = Vec::new();
    for sig in signatures {
        let current_block_time = sig.block_time.unwrap_or(0_i64);
        if current_block_time > last_block_time {
            let history = History::parse_from_signature(sig, &public_key)?;
            new_history.push(history);
        }
    }

    if !new_history.is_empty() {
        let repo = HistoryRepository::new();
        repo.insert_batch(new_history.clone())?;

        // Notice
        notice::msg(notice::MsgType::RefreshHistory, new_history);
    }

    Ok(())
}

pub fn get_public_key_by_str(public_key_str: &str) -> Result<Pubkey, String> {
    public_key_str
        .parse()
        .map_err(|e| format!("无效的公钥 ({}): {}", public_key_str, e))
}
