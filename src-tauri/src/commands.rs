use tauri::State;

use crate::cancel;
use crate::file_ops;
use crate::log::LogManager;
use crate::metadata;
use crate::models::*;

pub struct AppState {
    pub log_manager: LogManager,
}

#[tauri::command]
pub fn scan_directory(path: String) -> Result<Vec<FileEntry>, String> {
    let path_buf = std::path::PathBuf::from(&path);
    file_ops::scan_directory(&path_buf)
}

#[tauri::command]
pub fn get_directory_stats(paths: Vec<String>) -> Result<DirectoryStats, String> {
    file_ops::get_directory_stats(paths)
}

#[tauri::command]
pub fn is_geocoder_ready() -> bool {
    crate::geocode::is_geocoder_ready()
}

/// 根据经纬度离线查询最近地名（省/市/区/地点）
#[tauri::command]
pub fn query_location(latitude: f64, longitude: f64) -> Result<GpsLocation, String> {
    crate::geocode::reverse_geocode(latitude, longitude)
}

#[tauri::command]
pub fn write_log(
    state: State<'_, AppState>,
    operation_type: String,
    source_path: String,
    target_path: Option<String>,
    status: String,
    detail: Option<String>,
) -> Result<LogEntry, String> {
    state.log_manager.write_log(
        &operation_type,
        &source_path,
        target_path.as_deref(),
        &status,
        detail.as_deref(),
    )
}

#[tauri::command]
pub fn query_logs(
    state: State<'_, AppState>,
    operation_type: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<usize>,
) -> Result<Vec<LogEntry>, String> {
    state
        .log_manager
        .query_logs(operation_type.as_deref(), start_date.as_deref(), end_date.as_deref(), limit.unwrap_or(100))
}

#[tauri::command]
pub fn clear_all_logs(
    state: State<'_, AppState>,
) -> Result<usize, String> {
    state.log_manager.clear_all()
}

#[tauri::command]
pub fn delete_log(
    state: State<'_, AppState>,
    log_id: String,
) -> Result<bool, String> {
    state.log_manager.delete_log(&log_id)
}

#[tauri::command]
pub fn get_app_settings(state: State<'_, AppState>) -> AppSettings {
    AppSettings {
        log_retention_days: state.log_manager.get_retention_days(),
        log_dir: state.log_manager.log_dir().to_string_lossy().to_string(),
    }
}

#[tauri::command]
pub fn set_log_retention(state: State<'_, AppState>, days: u32) -> Result<(), String> {
    state.log_manager.set_retention_days(days)
}

#[tauri::command]
pub fn organize_files_async(
    app: tauri::AppHandle,
    source_dirs: Vec<String>,
    target_dir: String,
    rule: OrganizeRule,
) -> Result<Vec<OrganizeResult>, String> {
    use tauri::Emitter;

    cancel::reset();

    let result = file_ops::organize_files_with_progress(
        &source_dirs,
        &target_dir,
        &rule,
        |progress: &OrganizeProgress| {
            let _ = app.emit("organize-progress", progress);
        },
    );

    if cancel::is_cancelled() {
        cancel::reset();
        return Err("用户已取消操作".to_string());
    }

    result
}

#[tauri::command]
pub fn cancel_operation() -> CancelResult {
    cancel::set_cancelled();
    CancelResult {
        cancelled: true,
        message: "操作已取消".to_string(),
    }
}

#[tauri::command]
pub async fn fix_date_taken_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    target_dir: String,
    date_source: String,
    specified_time: Option<String>,
) -> Result<Vec<ImageProcessResult>, String> {
    use tauri::Emitter;

    cancel::reset();

    let result = metadata::fix_date_taken_batch(&paths, &target_dir, &date_source, specified_time.as_deref(), |progress: &TaskProgress| {
        let _ = app.emit("image-process-progress", progress);
    });

    if cancel::is_cancelled() {
        cancel::reset();
        return Err("用户已取消操作".to_string());
    }

    result
}

#[tauri::command]
pub async fn strip_exif_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    target_dir: String,
    options: metadata::StripOptions,
) -> Result<Vec<ImageProcessResult>, String> {
    use tauri::Emitter;

    cancel::reset();

    let result = metadata::strip_metadata_batch(&paths, &target_dir, options, |progress: &TaskProgress| {
        let _ = app.emit("image-process-progress", progress);
    });

    if cancel::is_cancelled() {
        cancel::reset();
        return Err("用户已取消操作".to_string());
    }

    result
}

#[tauri::command]
pub async fn write_gps_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    target_dir: String,
    latitude: f64,
    longitude: f64,
) -> Result<Vec<ImageProcessResult>, String> {
    use tauri::Emitter;

    cancel::reset();

    let result = metadata::write_gps_batch(&paths, &target_dir, latitude, longitude, |progress: &TaskProgress| {
        let _ = app.emit("image-process-progress", progress);
    });

    if cancel::is_cancelled() {
        cancel::reset();
        return Err("用户已取消操作".to_string());
    }

    result
}

#[tauri::command]
pub fn open_in_explorer(path: String) -> Result<(), String> {
    use std::process::Command;
    
    let path_obj = std::path::Path::new(&path);
    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }
    
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| format!("打开资源管理器失败: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| format!("打开Finder失败: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        let parent = path_obj.parent().unwrap_or(path_obj);
        Command::new("xdg-open")
            .arg(parent)
            .spawn()
            .map_err(|e| format!("打开文件管理器失败: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn batch_rename_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    target_dir: String,
    rule: RenameRule,
) -> Result<Vec<RenameResult>, String> {
    use tauri::Emitter;

    cancel::reset();

    let total = paths.len() as u64;

    let result = file_ops::batch_rename_with_progress(
        &paths,
        &target_dir,
        &rule,
        |progress: &TaskProgress| {
            let _ = app.emit("rename-progress", progress);
        },
    );

    if cancel::is_cancelled() {
        cancel::reset();
        return Err("用户已取消操作".to_string());
    }

    let _ = app.emit("rename-progress", &TaskProgress {
        total,
        processed: total,
        current_item: None,
        percentage: 100.0,
    });

    result
}

#[tauri::command]
pub async fn find_duplicates_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    detect_mode: String,
) -> Result<DuplicateScanResult, String> {
    use tauri::Emitter;

    cancel::reset();

    let result = file_ops::find_duplicates_with_progress(
        &paths,
        &detect_mode,
        |progress: &TaskProgress| {
            let _ = app.emit("duplicate-progress", progress);
        },
    );

    if cancel::is_cancelled() {
        cancel::reset();
        return Err("用户已取消操作".to_string());
    }

    if let Ok(ref result) = result {
        let _ = app.emit("duplicate-progress", &TaskProgress {
            total: result.total_files,
            processed: result.total_files,
            current_item: None,
            percentage: 100.0,
        });
    }

    result
}

#[tauri::command]
pub fn clean_duplicates(
    paths: Vec<String>,
    keep_original: bool,
) -> Result<BatchOperationResult, String> {
    file_ops::clean_duplicates(paths, keep_original)
}

#[tauri::command]
pub fn move_files_batch(
    sources: Vec<String>,
    target: String,
) -> Result<BatchOperationResult, String> {
    file_ops::move_files_batch(sources, &target)
}

#[tauri::command]
pub async fn scan_auxiliary_files_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    cleanup_types: Vec<String>,
) -> Result<Vec<CleanupResult>, String> {
    use tauri::Emitter;

    cancel::reset();

    let result = file_ops::scan_auxiliary_files_with_progress(
        &paths,
        &cleanup_types,
        |progress: &TaskProgress| {
            let _ = app.emit("cleanup-scan-progress", progress);
        },
    );

    if cancel::is_cancelled() {
        cancel::reset();
        return Err("用户已取消操作".to_string());
    }

    result
}

#[tauri::command]
pub async fn cleanup_auxiliary_files_async(
    app: tauri::AppHandle,
    files: Vec<String>,
) -> Result<BatchOperationResult, String> {
    use tauri::Emitter;

    cancel::reset();

    let total = files.len() as u64;
    let result = file_ops::cleanup_auxiliary_files_with_progress(
        files,
        |progress: &TaskProgress| {
            let _ = app.emit("cleanup-clean-progress", progress);
        },
    );

    if cancel::is_cancelled() {
        cancel::reset();
        return Err("用户已取消操作".to_string());
    }

    if let Ok(ref result) = result {
        let _ = app.emit("cleanup-clean-progress", &TaskProgress {
            total,
            processed: (result.success_count + result.fail_count) as u64,
            current_item: None,
            percentage: 100.0,
        });
    }

    result
}
