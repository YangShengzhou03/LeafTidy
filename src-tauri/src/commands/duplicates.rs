use tauri::State;

use crate::models::{BatchOperationResult, DuplicateScanResult};

use super::{run_batch_task, AppState};

#[tauri::command]
pub async fn find_duplicates_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    detect_mode: String,
) -> Result<DuplicateScanResult, String> {
    use tauri::Emitter;

    log::info!("查找重复文件: {} 个目录, 模式: {detect_mode}", paths.len());
    let start = std::time::Instant::now();

    let task_app = app.clone();
    let result = run_batch_task(move |token| {
        crate::duplicates::find_duplicates_with_progress(token, &paths, &detect_mode, |progress| {
            let _ = task_app.emit("duplicate-progress", progress);
        })
    })
    .await;

    if let Ok(ref result) = result {
        log::info!("重复扫描完成: {} 文件, {} 组重复, 浪费 {} 字节, 耗时 {:?}",
            result.total_files, result.duplicate_groups.len(), result.wasted_space, start.elapsed());
        let _ = app.emit("duplicate-progress", &crate::models::TaskProgress {
            total: result.total_files,
            processed: result.total_files,
            current_item: None,
            percentage: 100.0,
        });
    }

    result
}

// tauri::command 对前端统一返回 Result，内部实现不会失败
#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn clean_duplicates(
    state: State<'_, AppState>,
    paths: Vec<String>,
    keep_original: bool,
) -> Result<BatchOperationResult, String> {
    log::warn!("清理重复文件: {} 个文件, 保留原文件: {}", paths.len(), !keep_original);

    let result = crate::duplicates::clean_duplicates(&paths, keep_original);

    let action = if keep_original { "删除重复" } else { "移至回收站" };
    let _ = state.log_manager.write_event(
        &format!("重复清理({action}): {} 个文件", result.success_count),
        result.total_size,
    );

    Ok(result)
}
