#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            leafnova::commands::get_wechat_status,
            leafnova::commands::send_message,
            leafnova::commands::send_file,
        ])
        .run(tauri::generate_context!())
        .expect("启动失败");
}