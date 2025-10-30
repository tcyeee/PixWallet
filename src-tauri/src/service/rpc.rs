use crate::try_notice;
use crate::{
    models::{history::History, network::SolanaNetwork, wallet::Wallet},
    repository::history_repo::HistoryRepository,
    service::notice::{self, show, NoticeType},
};
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
};

pub fn transfer(payer: Wallet, receiver_public_key: String, amount: f32) {
    // [校验] 如果收款账户无法解析则提示
    let receiver_result: Result<Pubkey, String> = receiver_public_key
        .parse()
        .map_err(|e| format!("公钥校验失败: {}", e));
    let receiver: Pubkey = try_notice!(receiver_result);
    let sender: Keypair = Keypair::from_base58_string(&payer.private_key);
    let transfer_amount = (amount * LAMPORTS_PER_SOL as f32) as u64;

    let client: RpcClient = SolanaNetwork::get_rpc_client(payer.network);
    let transfer_instruction = solana_system_interface::instruction::transfer(
        &sender.pubkey(),
        &receiver,
        transfer_amount,
    );

    println!("[DEBUG] 开始转账..");
    let blockhash_result = client.get_latest_blockhash().map_err(|e| e.to_string());
    let blockhash = try_notice!(blockhash_result);
    println!("[DEBUG] Transfer 获取到最新区块:{}..", blockhash);

    let mut transaction =
        Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));
    transaction.sign(&[&sender], blockhash);
    println!("[DEBUG] Transfer 构建转账命令完成..");

    println!("[DEBUG] Transfer 开始上传交易数据..");
    let signature_result = client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| e.to_string());
    let signature = try_notice!(signature_result);
    println!("[DEBUG] Transfer 交易数据上传完成..");
    println!("[DEBUG] Transfer 交易完成, 签名:{}", signature);

    println!("[DEBUG] Transfer 更新支付账户余额..");
    println!("[DEBUG] Transfer 更新交易记录..");
}

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
