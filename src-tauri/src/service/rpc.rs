use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
 
use crate::try_notice;
use crate::{
    models::{history::History, network::SolanaNetwork, wallet::Wallet, token_price::*},
    repository::history_repo::HistoryRepository,
    repository::token_price_repo::TokenPriceRepository,
    service::notice::{self, show, NoticeType},
    utils::http_client::get_pyth_price,
};

use chrono::Local;
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
};

const CACHE_TTL: i64 = 300; // 5 分钟

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

    transfer_record("正在构建转账方法..");
    let blockhash_result = client.get_latest_blockhash().map_err(|e| e.to_string());
    let blockhash = try_notice!(blockhash_result);
    transfer_record(&format!("获取到最新区块:{}", blockhash));
    let mut transaction =
        Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));
    transaction.sign(&[&sender], blockhash);
    transfer_record("交易命令构建完成..");
    let fee_result = client
        .get_fee_for_message(transaction.message())
        .map_err(|e| e.to_string());
    let fee = try_notice!(fee_result);
    transfer_record(&format!(
        "预计手续费: {} lamports (~{} SOL)",
        fee,
        fee as f32 / LAMPORTS_PER_SOL as f32
    ));
    transfer_record("开始上传交易数据..");
    let signature_result = client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| e.to_string());
    let signature = try_notice!(signature_result);
    transfer_record("交易数据上传完成..");
    transfer_record(&format!("交易完成,签名:{}", signature));
    transfer_record("更新支付账户余额..");
    transfer_record("更新交易记录..");
    transfer_record("🎉🎉🎉交易成功!..");
    notice::show(NoticeType::Success, "恭喜,交易完成!");
    notice::msg(notice::MsgType::TransferEnd, &receiver_public_key);
}

fn transfer_record(content: &str) {
    thread::sleep(Duration::from_millis(150));

    let now = Local::now();
    let formatted = now.format("%H:%M:%S").to_string();
    let content = format!("{} {}", formatted, content);

    notice::msg(notice::MsgType::TransferInfo, &content);
    println!("[Transfer] {}", &content);
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

// pub fn transfer_detail(
//     signature: &str,
// ) -> Result<solana_transaction_status_client_types::EncodedConfirmedTransactionWithStatusMeta, String>
// {
//     println!("{}", signature);
//     let client: RpcClient = SolanaNetwork::get_rpc_client(SolanaNetwork::Devnet);
//     let sig = Signature::from_str(signature).map_err(|e| e.to_string())?;
//     client
//         .get_transaction(&sig, UiTransactionEncoding::Json)
//         .map_err(|e| e.to_string())
// }

pub async fn get_price(symbol: &str) -> Result<Vec<TokenPrice>, String> {
    let symbol_list: Vec<String> = symbol
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if symbol_list.is_empty() {
        return Err("Symbol list is empty".to_string());
    }

    println!("[DEBUG] 查询符号: {:?}", symbol_list);

    // ① 先查缓存
    let repo: TokenPriceRepository = TokenPriceRepository::new();
    let mut need_remote_query_token: Vec<String> = vec![];
    let now = now_sec();

    let local_price_list = repo.get_multi(&symbol_list);
    
    // 如果本地有数据，检查是否需要更新
    if !local_price_list.is_empty() {
        for local_price in &local_price_list {
            if now - local_price.updated_at > CACHE_TTL {
                need_remote_query_token.push(local_price.symbol.clone());
            }
        }
        
        if need_remote_query_token.is_empty() {
            println!("[INFO] 使用缓存数据，数量: {}", local_price_list.len());
            return Ok(local_price_list);
        }
        println!("[INFO] 需要更新 {} 个价格", need_remote_query_token.len());
    } else {
        println!("[INFO] 缓存中没有数据，全部远程查询");
        need_remote_query_token = symbol_list.clone();
    }

    // ② 外网获取价格（使用更安全的调用）
    println!("[INFO] 开始远程查询 Pyth 价格...");
    let token_price_opt = match get_pyth_price(&need_remote_query_token).await {
        Ok(prices) => {
            println!("[INFO] 远程查询成功，获取到 {} 个价格", prices.len());
            prices
        }
        Err(e) => {
            // 如果远程查询失败，但有缓存数据，则返回缓存数据
            if !local_price_list.is_empty() {
                eprintln!("[WARN] 远程查询失败，使用缓存数据: {}", e);
                return Ok(local_price_list);
            }
            return Err(format!("Failed to fetch price: {}", e));
        }
    };

    // ③ 如果没有价格就直接返回缓存或空
    if token_price_opt.is_empty() {
        eprintln!("[WARN] 远程查询返回空价格列表");
        if !local_price_list.is_empty() {
            return Ok(local_price_list);
        }
        return Ok(vec![]);
    }

    // ④ 保存到数据库
    println!("[INFO] 保存 {} 个价格到数据库", token_price_opt.len());
    println!("[INFO] 保存 {:?} 到数据库", token_price_opt);
    match repo.save_all(&token_price_opt) {
        Ok(_) => println!("数据库保存成功 data={:?}", token_price_opt),
        Err(e) => println!("数据库保存远程价格失败:data={:?}, e= {}",token_price_opt, e) 
    };

    // ⑤ 发送通知
    notice::msg(notice::MsgType::RefreshTokenPrice, &token_price_opt);

    Ok(token_price_opt)
}
fn now_sec() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
