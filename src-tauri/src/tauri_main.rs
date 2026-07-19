#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
    )]

use leafmaster::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| {
            // 启动任务调度器后台线程
            std::thread::spawn(|| {
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(1));

                    let pending_tasks = SCHEDULER.get_pending_tasks();

                    for task in pending_tasks {
                        log_info!("TaskExecutor", "执行任务 #{}: 发送 '{}' 给 {}",
                            task.id, task.content, task.recipient);

                        // TODO: 调用微信API发送消息
                        // 这里需要实现微信消息发送逻辑

                        SCHEDULER.increment_execute_count(task.id);
                    }
                }
            });

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
        ])
        .run(tauri::generate_context!())
        .expect("启动失败");
}