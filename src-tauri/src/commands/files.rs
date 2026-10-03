use tauri::State;

use crate::file_ops;
use crate::models::{BatchOperationResult, DirectoryStats, FileEntry};

use super::AppState;

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn scan_directory(path: String) -> Result<Vec<FileEntry>, String> {
    log::info!("扫描目录: {path}");
    let start = std::time::Instant::now();
    let path_buf = std::path::PathBuf::from(&path);
    let result = file_ops::scan_directory(&path_buf);
    match &result {
        Ok(entries) => log::info!("扫描完成: {} 个文件, 耗时 {:?}", entries.len(), start.elapsed()),
        Err(e) => log::error!("扫描失败: {e}"),
    }
    result
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn get_directory_stats(paths: Vec<String>) -> Result<DirectoryStats, String> {
    log::info!("获取目录统计: {} 个路径", paths.len());
    let result = file_ops::get_directory_stats(paths);
    if let Ok(ref stats) = result {
        log::info!("目录统计: {} 文件, {} 目录, {} 字节", stats.total_files, stats.total_dirs, stats.total_size);
    }
    result
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn copy_files_batch(
    state: State<'_, AppState>,
    sources: Vec<String>,
    target: String,
) -> Result<BatchOperationResult, String> {
    log::info!("批量复制: {} 个文件 -> {target}", sources.len());

    let result = file_ops::copy_files_batch(&sources, &target);

    match &result {
        Ok(r) => {
            let _ = state.log_manager.write_event(
                &format!("文件复制: {} 个文件 -> {target}", r.success_count),
                0,
            );
        }
        Err(e) => log::error!("复制失败: {e}"),
    }

    result
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn open_in_explorer(path: String) -> Result<(), String> {
    log::debug!("打开资源管理器: {path}");

    let path_obj = std::path::Path::new(&path);
    if !path_obj.exists() {
        return Err(format!("文件不存在: {path}"));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| format!("打开资源管理器失败: {e}"))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| format!("打开Finder失败: {e}"))?;
    }

    #[cfg(target_os = "linux")]
    {
        let parent = path_obj.parent().unwrap_or(path_obj);
        std::process::Command::new("xdg-open")
            .arg(parent)
            .spawn()
            .map_err(|e| format!("打开文件管理器失败: {e}"))?;
    }

    Ok(())
}
