use crate::models::{BatchOperationResult, DuplicateScanResult};
use crate::tasks::{event, scope};

use super::run_batch_task;

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
    let result = run_batch_task(scope::DUPLICATE, move |token| {
        crate::duplicates::find_duplicates_with_progress(token, &paths, &detect_mode, |progress| {
            let _ = task_app.emit(event::DUPLICATE_PROGRESS, progress);
        })
    })
    .await;

    if let Ok(ref result) = result {
        log::info!("重复扫描完成: {} 文件, {} 组重复, 浪费 {} 字节, 耗时 {:?}",
            result.total_files, result.duplicate_groups.len(), result.wasted_space, start.elapsed());
        let _ = app.emit(event::DUPLICATE_PROGRESS, &crate::models::TaskProgress {
            total: result.total_files,
            processed: result.total_files,
            current_item: None,
            percentage: 100.0,
        });
    }

    result
}

#[tauri::command]
pub async fn clean_duplicates(
    app: tauri::AppHandle,
    paths: Vec<String>,
    use_trash: bool,
) -> Result<BatchOperationResult, String> {
    use tauri::Emitter;

    log::warn!("清理重复文件: {} 个文件, 移至回收站: {use_trash}", paths.len());

    // 信任边界校验：只允许删除绝对路径的真实文件，拒绝目录
    for p in &paths {
        let path = std::path::Path::new(p);
        if !path.is_absolute() {
            return Err(format!("非法路径（必须是绝对路径）: {p}"));
        }
        if path.is_dir() {
            return Err(format!("拒绝删除目录: {p}"));
        }
    }

    // 必须走 run_batch_task：删除属于破坏性操作，需要可取消并注册 ACTIVE，
    // 否则任务运行期间托盘"退出程序"不会被拦截，可能导致删到一半退出。
    let task_app = app.clone();
    run_batch_task(scope::DUPLICATE_CLEAN, move |token| {
        crate::duplicates::clean_duplicates_with_progress(token, &paths, use_trash, |progress| {
            let _ = task_app.emit(event::DUPLICATE_CLEAN_PROGRESS, progress);
        })
    })
    .await
}
