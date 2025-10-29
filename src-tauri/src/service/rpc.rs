use crate::models::{history::History, network::SolanaNetwork};
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_sdk::pubkey::Pubkey;

pub fn history_update(_history_list: &Vec<History>, public_key: &str, network: SolanaNetwork) {
    let client: RpcClient = SolanaNetwork::get_rpc_client(network);
    let pubkey: Pubkey = get_public_key_by_str(&public_key).unwrap();

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
        .unwrap();

    println!("开始检查账户:{:?}的100条记录", public_key);
    for sig in signatures {
        println!("Signature: {}, Slot: {}", sig.signature, sig.slot);
    }
}

pub fn get_public_key_by_str(public_key_str: &str) -> Result<Pubkey, String> {
    public_key_str
        .parse()
        .map_err(|e| format!("无效的公钥 ({}): {}", public_key_str, e))
}

// pub fn history(&self, repo: &WalletRepository) -> Result<Vec<History>, String> {
// let client: RpcClient = SolanaNetwork::get_rpc_client(self.network);
// let pubkey = &self.pubkey()?;
// let signatures: Vec<
//     solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature,
// > = client
//     .get_signatures_for_address_with_config(
//         pubkey,
//         GetConfirmedSignaturesForAddress2Config {
//             limit: Some(100), // 默认最多1000
//             before: None,
//             until: None,
//             commitment: None,
//         },
//     )
//     .unwrap();

// println!("开始检查账户:{:?}的100条记录", &self.alias);
// for sig in signatures {
//     println!("Signature: {}, Slot: {}", sig.signature, sig.slot);
// }
// Ok(())
// }
