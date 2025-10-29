use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::models::{history::History, network::SolanaNetwork};

pub async fn history_update(
    _history_list: &Vec<History>,
    public_key: String,
    network: SolanaNetwork,
) {
    let _client: RpcClient = SolanaNetwork::get_rpc_client(network);
    let _pubkey: Pubkey = match get_public_key_by_str(&public_key) {
        Ok(pk) => pk,
        Err(_) => {
            // 出错时通知前端
            // notify_frontend(e);
            return ();
        }
    };

    // 后续的转账代码

    // client
    //     .get_signatures_for_address_with_config(
    //         &public_key,
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
}

pub fn get_public_key_by_str(public_key_str: &String) -> Result<Pubkey, String> {
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
