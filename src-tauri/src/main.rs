// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod service;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            service::greet::say,
            service::wallet::create_wallet,
            service::wallet::query_wallet,
            service::wallet::change_alias,
            service::wallet::delete_wallet,
            service::wallet::refresh_balance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
