//! MWXDump UI Tauri Library
//! 
//! 这是 MWXDump UI 应用程序的 Tauri 后端库，提供与前端交互的命令。

use mwxdump_core::{
    ProcessDetector, WechatProcessInfo,
    models::{Contact, Message, ChatRoom, Session},
    logs::{init_tracing_with_config, LogConfig},
    Result,
};
use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Mutex;
use std::path::PathBuf;

/// 应用程序状态
#[derive(Default)]
pub struct AppState {
    pub current_process: Mutex<Option<WechatProcessInfo>>,
}

/// 进程信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfoResponse {
    pub pid: u32,
    pub name: String,
    pub version: String,
    pub path: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

impl From<WechatProcessInfo> for ProcessInfoResponse {
    fn from(info: WechatProcessInfo) -> Self {
        Self {
            pid: info.pid,
            name: info.name,
            version: format!("{:?}", info.version), // 使用 Debug 格式
            path: info.path.to_string_lossy().to_string(), // 转换 PathBuf 为 String
        }
    }
}

/// 初始化应用程序
fn init_app() -> Result<()> {
    // 使用 core 中的统一日志系统
    let log_config = LogConfig::console();
    init_tracing_with_config(&log_config)?;

    Ok(())
}

pub fn run() -> Result<()> {
    // 初始化应用程序
    if let Err(e) = init_app() {
        eprintln!("应用程序初始化失败: {}", e);
        std::process::exit(1);
    }

    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");    
     Ok(())
}