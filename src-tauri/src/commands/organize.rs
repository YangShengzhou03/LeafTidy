use crate::models::{OrganizeResult, OrganizeRule};
use crate::tasks::{event, scope};

use super::run_batch_task;

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub async fn organize_files_async(
    app: tauri::AppHandle,
    source_dirs: Vec<String>,
    target_dir: String,
    rule: OrganizeRule,
) -> Result<Vec<OrganizeResult>, String> {
    use tauri::Emitter;

    log::info!("开始文件整理: {} 个源目录 -> {target_dir}, 规则: {rule:?}", source_dirs.len());
    // 信任边界校验：整理会移动源文件，源目录与目标目录都必须是绝对路径
    crate::fs_util::ensure_absolute_dir(&target_dir)?;
    crate::fs_util::ensure_absolute_dirs(&source_dirs)?;
    let start = std::time::Instant::now();

    let result = run_batch_task(scope::ORGANIZE, move |token| {
        crate::organize::organize_files_with_progress(token, &source_dirs, &target_dir, &rule, |progress| {
            let _ = app.emit(event::ORGANIZE_PROGRESS, progress);
        })
    })
    .await;

    match &result {
        Ok(results) => {
            let success = results.iter().filter(|r| r.success).count();
            log::info!("文件整理完成: {}/{} 文件成功, 耗时 {:?}", success, results.len(), start.elapsed());
        }
        Err(e) => log::error!("文件整理失败: {e}"),
    }

    result
}
