// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mwxdump_ui_lib::{
    init_app, AppState,
    scan_wechat_processes, select_wechat_process, get_current_process,
    extract_wechat_key, decrypt_wechat_data,
    get_contacts, get_messages, get_chatrooms, get_sessions,
};

fn main() {
    // 初始化应用程序
    if let Err(e) = init_app() {
        eprintln!("应用程序初始化失败: {}", e);
        std::process::exit(1);
    }

    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_wechat_processes,
            select_wechat_process,
            get_current_process,
            extract_wechat_key,
            decrypt_wechat_data,
            get_contacts,
            get_messages,
            get_chatrooms,
            get_sessions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
