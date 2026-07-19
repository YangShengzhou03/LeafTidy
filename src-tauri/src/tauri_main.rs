#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            leafmaster::commands::get_wechat_status,
            leafmaster::commands::send_message,
            leafmaster::commands::send_file,
        ])
        .run(tauri::generate_context!())
        .expect("启动失败");
}