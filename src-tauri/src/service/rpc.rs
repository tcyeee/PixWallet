use crate::{
    models::{history::History, network::SolanaNetwork},
    repository::history_repo::HistoryRepository,
};
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_sdk::pubkey::Pubkey;

pub fn history_update(
    history_list: &Vec<History>,
    public_key: &str,
    network: SolanaNetwork,
) -> Result<(), String> {
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

    // 找到最新的时间
    if !history_list.is_empty() {
        let first = match history_list.get(0) {
            Some(item) => item,
            None => return Ok(()),
        };

        // 同时signatures也要有值
        if signatures.is_empty() {
            return Ok(());
        };

        let mut new_history: Vec<History> = Vec::new();
        for sig in signatures {
            if sig.block_time > first.block_time {
                let history = History::parse_from_signature(sig, &first.public_key)?;
                new_history.push(history);
            }
        }

        if new_history.is_empty() {
            return Ok(());
        }

        // 批量添加
        let repo = HistoryRepository::new();
        repo.insert_batch(new_history)?;
    }

    Ok(())
}

pub fn get_public_key_by_str(public_key_str: &str) -> Result<Pubkey, String> {
    public_key_str
        .parse()
        .map_err(|e| format!("无效的公钥 ({}): {}", public_key_str, e))
}
