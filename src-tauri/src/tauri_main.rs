#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
    )]

use leafmaster::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| {
            // 问题 H1 修复：启动后台任务执行循环
            leafmaster::task_executor::SCHEDULER.start_execution_loop();
            log_info!("TauriMain", "应用已启动，任务调度器已启动");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            leafmaster::commands::get_wechat_status,
            leafmaster::commands::send_message,
            leafmaster::commands::send_file,
            leafmaster::task_executor::schedule_task,
            leafmaster::task_executor::cancel_task,
            leafmaster::task_executor::get_pending_tasks_count,
            leafmaster::logger::get_log_files,
            leafmaster::logger::read_log_file,
            leafmaster::logger::delete_log_file,
            leafmaster::logger::delete_all_logs,
            leafmaster::logger::log_frontend,
        ])
        .run(tauri::generate_context!())
        .expect("启动失败");
}