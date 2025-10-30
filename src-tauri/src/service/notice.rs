use {
    once_cell::sync::OnceCell,
    serde::{Deserialize, Serialize},
    serde_json,
    tauri::{AppHandle, Emitter},
};

pub static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

#[derive(Debug, Serialize)]
pub enum MsgType {
    Ping,
    RefreshHistory,
    BalanceRefreshEnd,
    TransferEnd,
    TransferInfo,
    BalanceChange,
}

impl MsgType {
    pub fn name(&self) -> &'static str {
        match self {
            MsgType::Ping => "PING",
            MsgType::TransferEnd => "TRANSFER_END",
            MsgType::TransferInfo => "TRANSFER_INFO",
            MsgType::RefreshHistory => "REFRESH_HISTORY",
            MsgType::BalanceRefreshEnd => "BALANCE_REFRESH_END",
            MsgType::BalanceChange => "BALANCE_CHANGE",
        }
    }
}

pub fn msg<S: Serialize + Clone>(msg_type: MsgType, msg: S) {
    if let Some(app) = APP_HANDLE.get() {
        print(&msg_type, &msg);
        app.emit(msg_type.name(), msg)
            .inspect_err(|e| eprintln!("[Notice] Failed to emit: {}", e))
            .ok();
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum NoticeType {
    Info,
    Success,
    Warning,
    Error,
}

impl NoticeType {
    pub fn name(&self) -> &'static str {
        match self {
            NoticeType::Info => "INFO",
            NoticeType::Success => "SUCCESS",
            NoticeType::Warning => "WARNING",
            NoticeType::Error => "ERROR",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ShowParams {
    level: String,
    content: String,
}

pub fn show(notice_type: NoticeType, content: &str) {
    let params = ShowParams {
        level: notice_type.name().to_string(),
        content: content.to_string(),
    };

    if let Some(app) = APP_HANDLE.get() {
        app.emit("NOTICE", params)
            .inspect_err(|e| eprintln!("[Notice] Failed to emit: {}", e))
            .ok();
    }
}

pub fn print<S: Serialize + Clone>(msg_type: &MsgType, msg: &S) {
    match serde_json::to_string(&msg) {
        Ok(json) => println!("MSG:{:?} :: {}", msg_type, json),
        Err(_) => println!("无法序列化消息内容"),
    }
}
