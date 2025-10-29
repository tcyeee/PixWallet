use {
    once_cell::sync::OnceCell,
    serde::{Deserialize, Serialize},
    tauri::AppHandle,
    tauri::Emitter,
};

pub static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

#[derive(Debug)]
pub enum MsgType {
    Ping,
    BalanceRefreshEnd,
    BalanceChange,
}

impl MsgType {
    pub fn name(&self) -> &'static str {
        match self {
            MsgType::Ping => "PING",
            MsgType::BalanceRefreshEnd => "BALANCE_REFRESH_END",
            MsgType::BalanceChange => "BALANCE_CHANGE",
        }
    }
}

pub fn msg<S: Serialize + Clone>(msg_type: MsgType, msg: S) {
    if let Some(app) = APP_HANDLE.get() {
        app.emit(msg_type.name(), msg)
            .inspect_err(|e| eprintln!("[Notice] Failed to emit: {}", e))
            .ok();
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum NoticeType {
    Success,
    _Warning,
    _Error,
}

impl NoticeType {
    pub fn name(&self) -> &'static str {
        match self {
            NoticeType::Success => "SUCCESS",
            NoticeType::_Warning => "WARNING",
            NoticeType::_Error => "ERROR",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ShowParams {
    level: String,
    content: String,
}

pub fn show(notice_type: NoticeType, info: &str) {
    let params = ShowParams {
        level: notice_type.name().to_string(),
        content: info.to_string(),
    };

    if let Some(app) = APP_HANDLE.get() {
        app.emit("NOTICE", params)
            .inspect_err(|e| eprintln!("[Notice] Failed to emit: {}", e))
            .ok();
    }
}
