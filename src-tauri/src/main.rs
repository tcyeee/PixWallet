// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod repository;
mod service;

use db::establish_connection;

use crate::service::notice::APP_HANDLE;

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
async fn main() {
    let conn_state = establish_connection().expect("Failed to connect to database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(conn_state)
        .invoke_handler(tauri::generate_handler![
            service::greet::say,
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
