use crate::models::{message_type::MsgType, network::SolanaNetwork};
use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use std::time::Instant;
use tauri::{AppHandle, Emitter};
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

pub fn start_monitor(app: AppHandle) {
    tokio::spawn(async move {
        let client: RpcClient = SolanaNetwork::get_rpc_client(SolanaNetwork::Devnet);
        loop {
            check(&client, &app);
            sleep(Duration::from_secs(5)).await;
        }
    });
}

pub fn check(client: &RpcClient, app: &AppHandle) {
    let start = Instant::now();
    let health: Result<(), ClientError> = client.get_health();
    let elapsed = start.elapsed();
    match health {
        Ok(_) => {
            if elapsed.as_millis() <= 1000 {
                send(app, NetworkStatus::Good(elapsed.as_millis()));
            } else {
                send(app, NetworkStatus::Poor(elapsed.as_millis()));
            }
        }
        Err(_) => {
            send(app, NetworkStatus::Lost(elapsed.as_millis()));
        }
    }
}

fn send(app: &AppHandle, status: NetworkStatus) {
    app.emit(MsgType::Ping.name(), status.assemble())
        .inspect_err(|e| eprintln!("[NETWORK_CHECK] Failed to emit 'PING': {}", e))
        .ok();
}
