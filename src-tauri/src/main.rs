// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod repository;
mod service;
mod utils;

use crate::{db::connection::establish_connection, service::notice::APP_HANDLE};

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
async fn main() {
    establish_connection();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            service::wallet::create_wallet,
            service::wallet::query_wallet,
            service::wallet::change_alias,
            service::wallet::delete_wallet,
            service::wallet::refresh_balance,
            service::wallet::transfer,
            service::wallet::account_history,
        ])
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            // PING
            service::network_monitor::start_monitor();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
