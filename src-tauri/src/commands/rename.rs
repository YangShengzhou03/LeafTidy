use tauri::State;

use crate::models::{RenameResult, RenameRule, TaskProgress};

use super::{run_batch_task, AppState};

#[tauri::command]
pub async fn batch_rename_async(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    paths: Vec<String>,
    target_dir: String,
    rule: RenameRule,
) -> Result<Vec<RenameResult>, String> {
    use tauri::Emitter;

    log::info!("批量重命名: {} 个文件 -> {target_dir}", paths.len());
    let start = std::time::Instant::now();
    let total = paths.len() as u64;
    let file_count = paths.len();

    let task_app = app.clone();
    let result = run_batch_task(move |token| {
        crate::rename::batch_rename_with_progress(token, &paths, &target_dir, &rule, |progress| {
            let _ = task_app.emit("rename-progress", progress);
        })
    })
    .await;

    let _ = app.emit("rename-progress", &TaskProgress {
        total,
        processed: total,
        current_item: None,
        percentage: 100.0,
    });

    match &result {
        Ok(results) => {
            let success = results.iter().filter(|r| r.success).count();
            log::info!("批量重命名完成: {}/{} 文件成功, 耗时 {:?}", success, results.len(), start.elapsed());
            let _ = state.log_manager.write_event(
                &format!("批量重命名: {success}/{file_count} 个文件"),
                0,
            );
        }
        Err(e) => log::error!("批量重命名失败: {e}"),
    }

    result
}
