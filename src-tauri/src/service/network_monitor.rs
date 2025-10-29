use crate::models::network::SolanaNetwork;
use crate::service::notice::{msg, MsgType};
use solana_client::rpc_client::RpcClient;
use std::time::Instant;
use tokio::time::timeout;
use tokio::time::{sleep, Duration};

#[derive(serde::Serialize, Clone, Debug)]
pub enum NetworkStatus {
    Good(u128),
    Poor(u128),
    Lost(u128),
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct NetworkAssemble {
    status: &'static str,
    ping: u128,
}

impl NetworkStatus {
    pub fn assemble(&self) -> NetworkAssemble {
        match self {
            NetworkStatus::Good(ping) => NetworkAssemble {
                status: "GOOD",
                ping: *ping,
            },
            NetworkStatus::Poor(ping) => NetworkAssemble {
                status: "POOR",
                ping: *ping,
            },
            NetworkStatus::Lost(ping) => NetworkAssemble {
                status: "LOST",
                ping: *ping,
            },
        }
    }
}

pub fn start_monitor() {
    tokio::spawn(async move {
        let client: RpcClient = SolanaNetwork::get_rpc_client(SolanaNetwork::Devnet);
        loop {
            check(&client).await;
            sleep(Duration::from_secs(5)).await;
        }
    });
}

pub async fn check(client: &RpcClient) {
    let start = Instant::now();
    let result = timeout(Duration::from_secs(5), async { client.get_health() }).await;
    let elapsed = start.elapsed().as_millis();
    match result {
        Ok(health) => match health {
            Ok(_) => {
                if elapsed <= 1000 {
                    sends(NetworkStatus::Good(elapsed));
                } else {
                    sends(NetworkStatus::Poor(elapsed));
                }
            }
            Err(_) => sends(NetworkStatus::Lost(elapsed)),
        },
        Err(_) => sends(NetworkStatus::Lost(9999)),
    }
}

fn sends(status: NetworkStatus) {
    msg(MsgType::Ping, status.assemble());
}
