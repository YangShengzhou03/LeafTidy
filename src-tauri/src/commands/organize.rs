use tauri::State;

use crate::models::{OrganizeResult, OrganizeRule};

use super::{run_batch_task, AppState};

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub async fn organize_files_async(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    source_dirs: Vec<String>,
    target_dir: String,
    rule: OrganizeRule,
) -> Result<Vec<OrganizeResult>, String> {
    use tauri::Emitter;

    log::info!("开始文件整理: {} 个源目录 -> {target_dir}, 规则: {rule:?}", source_dirs.len());
    let start = std::time::Instant::now();

    let dirs_joined = source_dirs.join(", ");
    let target_display = target_dir.clone();
    let result = run_batch_task(move |token| {
        crate::organize::organize_files_with_progress(token, &source_dirs, &target_dir, &rule, |progress| {
            let _ = app.emit("organize-progress", progress);
        })
    })
    .await;

    match &result {
        Ok(results) => {
            let total_size: u64 = results.iter().filter_map(|r| r.metadata.as_ref().and_then(|m| m.size)).sum();
            let success = results.iter().filter(|r| r.success).count();
            log::info!("文件整理完成: {}/{} 文件成功, 耗时 {:?}", success, results.len(), start.elapsed());
            let _ = state.log_manager.write_event(
                &format!("文件整理: {dirs_joined} -> {target_display} (成功 {}/{})", success, results.len()),
                total_size,
            );
        }
        Err(e) => log::error!("文件整理失败: {e}"),
    }

    result
}
