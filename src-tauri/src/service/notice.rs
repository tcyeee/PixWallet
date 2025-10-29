use {once_cell::sync::OnceCell, serde::Serialize, tauri::AppHandle, tauri::Emitter};

pub static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

#[derive(Debug)]
pub enum MsgType {
    _Error,
    Ping,
    BalanceRefreshEnd,
    BalanceChange,
}

impl MsgType {
    pub fn name(&self) -> &'static str {
        match self {
            MsgType::_Error => "ERROR",
            MsgType::Ping => "PING",
            MsgType::BalanceRefreshEnd => "BALANCE_REFRESH_END",
            MsgType::BalanceChange => "BALANCE_CHANGE",
        }
    }
}

pub fn notice<S: Serialize + Clone>(msg_type: MsgType, msg: S) {
    // todo
    if let Some(app) = APP_HANDLE.get() {
        app.emit(msg_type.name(), msg)
            .inspect_err(|e| eprintln!("[Notice] Failed to emit: {}", e))
            .ok();
    }
}
