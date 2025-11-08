// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod repository;
mod service;
mod utils;

use crate::{db::connection::establish_connection, service::notice::APP_HANDLE};
use std::env;
use tauri::Manager;

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
async fn main() {
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
            service::wallet::transfer_detail,
        ])
        .setup(|app| {
            // 初始化数据库
            let path = app.path().app_data_dir().unwrap();
            establish_connection(path);
            // 全局数据库连接 & 全局通知调用
            APP_HANDLE.set(app.handle().clone()).unwrap();
            // 开启网络监控
            service::network_monitor::start_monitor();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
