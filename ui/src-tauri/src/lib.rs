//! MWXDump UI Tauri Library
//! 
//! 这是 MWXDump UI 应用程序的 Tauri 后端库，提供与前端交互的命令。

use mwxdump_core::{
    ProcessDetector, WechatProcessInfo,
    models::{Contact, Message, ChatRoom, Session},
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

/// Windows 进程检测器实现
#[cfg(windows)]
struct WindowsProcessDetector;

#[cfg(windows)]
impl ProcessDetector for WindowsProcessDetector {
    async fn detect_processes(&self) -> mwxdump_core::Result<Vec<WechatProcessInfo>> {
        // TODO: 实现 Windows 进程检测
        Ok(vec![])
    }

    async fn get_process_by_pid(&self, _pid: u32) -> mwxdump_core::Result<Option<WechatProcessInfo>> {
        // TODO: 实现根据 PID 获取进程
        Ok(None)
    }
}

/// 扫描微信进程
#[tauri::command]
pub async fn scan_wechat_processes() -> Result<Vec<ProcessInfoResponse>, String> {
    #[cfg(windows)]
    let detector = WindowsProcessDetector;
    
    #[cfg(not(windows))]
    return Err("当前平台不支持".to_string());
    
    #[cfg(windows)]
    match detector.detect_processes().await {
        Ok(processes) => {
            let responses: Vec<ProcessInfoResponse> = processes
                .into_iter()
                .map(ProcessInfoResponse::from)
                .collect();
            Ok(responses)
        }
        Err(e) => Err(format!("扫描微信进程失败: {}", e)),
    }
}

/// 选择微信进程
#[tauri::command]
pub async fn select_wechat_process(
    pid: u32,
    state: State<'_, AppState>,
) -> Result<ProcessInfoResponse, String> {
    #[cfg(windows)]
    let detector = WindowsProcessDetector;
    
    #[cfg(not(windows))]
    return Err("当前平台不支持".to_string());
    
    #[cfg(windows)]
    match detector.get_process_by_pid(pid).await {
        Ok(Some(process)) => {
            let response = ProcessInfoResponse::from(process.clone());
            *state.current_process.lock().unwrap() = Some(process);
            Ok(response)
        }
        Ok(None) => Err(format!("未找到 PID {} 的微信进程", pid)),
        Err(e) => Err(format!("获取进程信息失败: {}", e)),
    }
}

/// 获取当前选择的进程
#[tauri::command]
pub async fn get_current_process(
    state: State<'_, AppState>,
) -> Result<Option<ProcessInfoResponse>, String> {
    let current = state.current_process.lock().unwrap();
    Ok(current.as_ref().map(|p| ProcessInfoResponse::from(p.clone())))
}

/// 提取微信密钥
#[tauri::command]
pub async fn extract_wechat_key(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let current = state.current_process.lock().unwrap();
    match current.as_ref() {
        Some(_process) => {
            // TODO: 实现密钥提取逻辑
            Ok("dummy_key".to_string())
        }
        None => Err("请先选择微信进程".to_string()),
    }
}

/// 解密微信数据
#[tauri::command]
pub async fn decrypt_wechat_data(
    key: String,
    input_path: String,
    output_path: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let current = state.current_process.lock().unwrap();
    match current.as_ref() {
        Some(_process) => {
            // TODO: 实现数据解密逻辑
            Ok(format!("已使用密钥 {} 将 {} 解密到 {}", key, input_path, output_path))
        }
        None => Err("请先选择微信进程".to_string()),
    }
}

/// 获取联系人列表
#[tauri::command]
pub async fn get_contacts() -> Result<Vec<Contact>, String> {
    // TODO: 实现获取联系人逻辑
    Ok(vec![])
}

/// 获取消息列表
#[tauri::command]
pub async fn get_messages(contact_id: String) -> Result<Vec<Message>, String> {
    // TODO: 实现获取消息逻辑
    let _ = contact_id;
    Ok(vec![])
}

/// 获取群聊列表
#[tauri::command]
pub async fn get_chatrooms() -> Result<Vec<ChatRoom>, String> {
    // TODO: 实现获取群聊逻辑
    Ok(vec![])
}

/// 获取会话列表
#[tauri::command]
pub async fn get_sessions() -> Result<Vec<Session>, String> {
    // TODO: 实现获取会话逻辑
    Ok(vec![])
}

/// 初始化应用程序
pub fn init_app() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    Ok(())
}