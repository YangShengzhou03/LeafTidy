use crate::models::{RenameResult, RenameRule, TaskProgress};
use crate::tasks::{event, scope};

use super::run_batch_task;

#[tauri::command]
pub async fn batch_rename_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    target_dir: String,
    rule: RenameRule,
) -> Result<Vec<RenameResult>, String> {
    use tauri::Emitter;

    log::info!("批量重命名: {} 个文件 -> {target_dir}", paths.len());
    // 信任边界校验：重命名会移动源文件，只接受绝对路径文件与绝对目标目录
    crate::fs_util::ensure_absolute_dir(&target_dir)?;
    crate::fs_util::ensure_absolute_non_dirs(&paths)?;
    let start = std::time::Instant::now();
    let total = paths.len() as u64;

    let task_app = app.clone();
    let result = run_batch_task(scope::RENAME, move |token| {
        crate::rename::batch_rename_with_progress(token, &paths, &target_dir, &rule, |progress| {
            let _ = task_app.emit(event::RENAME_PROGRESS, progress);
        })
    })
    .await;

    // 只在任务真正完成时补发 100%，取消/失败不应显示"已处理全部"
    if result.is_ok() {
        let _ = app.emit(event::RENAME_PROGRESS, &TaskProgress {
            total,
            processed: total,
            current_item: None,
            percentage: 100.0,
        });
    }

    match &result {
        Ok(results) => {
            let success = results.iter().filter(|r| r.success).count();
            log::info!("批量重命名完成: {}/{} 文件成功, 耗时 {:?}", success, results.len(), start.elapsed());
        }
        Err(e) => log::error!("批量重命名失败: {e}"),
    }

    result
}
