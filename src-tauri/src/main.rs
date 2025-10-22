// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod service;

use db::{create_tables, establish_connection};
use std::path::Path;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    // ✅ 数据库路径
    let db_path: &'static str = "wallet.db";
    let first_run: bool = !Path::new(db_path).exists();

    // ✅ 建立数据库连接
    let conn: rusqlite::Connection = establish_connection().expect("Failed to connect to database");

    // ✅ 首次运行时初始化表结构
    if first_run {
        create_tables(&conn).expect("Failed to initialize database tables");
        println!("✅ Database initialized successfully!");
    } else {
        println!("📁 Database already exists, skipping initialization.");
    }

    // ✅ 使用 Mutex 封装 Connection，供前端 handler 使用
    let conn_state: Mutex<_> = Mutex::new(conn);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // ✅ 注册数据库到全局状态（State）
        .manage(conn_state)
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
