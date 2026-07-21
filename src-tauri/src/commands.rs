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
pub fn list_directory(path: String) -> Result<Vec<FileEntry>, String> {
    file_ops::list_directory(&path)
}

#[tauri::command]
pub fn scan_directory(path: String) -> Result<Vec<FileEntry>, String> {
    let path_buf = std::path::PathBuf::from(&path);
    file_ops::scan_directory(&path_buf)
}

#[tauri::command]
pub fn list_subdirs(path: String) -> Result<Vec<DirEntry>, String> {
    file_ops::list_subdirs(&path)
}

#[tauri::command]
pub fn get_directory_stats(paths: Vec<String>) -> Result<DirectoryStats, String> {
    file_ops::get_directory_stats(paths)
}

#[tauri::command]
pub fn rename_file_command(source: String, new_name: String) -> Result<String, String> {
    file_ops::rename_file(&source, &new_name)
}

#[tauri::command]
pub fn delete_to_trash(paths: Vec<String>) -> BatchOperationResult {
    file_ops::delete_to_trash(paths)
}

#[tauri::command]
pub fn compute_md5(path: String) -> Result<String, String> {
    file_ops::compute_md5(&path)
}

#[tauri::command]
pub fn sanitize_filename(name: String) -> String {
    file_ops::sanitize_filename(&name)
}

#[tauri::command]
pub fn format_file_size(size: u64) -> String {
    file_ops::format_file_size(size)
}

#[tauri::command]
pub fn read_exif(path: String) -> Result<ExifInfo, String> {
    metadata::read_exif(&path)
}

#[tauri::command]
pub fn reverse_geocode(lat: f64, lng: f64) -> Result<GpsLocation, String> {
    metadata::reverse_geocode(lat, lng)
}

#[tauri::command]
pub fn init_geocoder(data: String) -> Result<(), String> {
    metadata::init_geocode_data(&data)
}

#[tauri::command]
pub fn is_geocoder_ready() -> bool {
    crate::geocode::is_geocoder_ready()
}

#[tauri::command]
pub fn get_file_category(format: String) -> String {
    metadata::get_file_category(&format)
}

#[tauri::command]
pub fn is_image_file(path: String) -> bool {
    metadata::is_image_file(&path)
}

#[tauri::command]
pub fn is_video_file(path: String) -> bool {
    metadata::is_video_file(&path)
}

#[tauri::command]
pub fn get_file_detail(path: String) -> Result<FileDetailInfo, String> {
    use std::path::Path;

    let path_obj = Path::new(&path);
    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    let metadata = std::fs::metadata(&path).map_err(|e| format!("获取元数据失败: {}", e))?;

    let name = path_obj
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let format = path_obj
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_else(|| "unknown".to_string());

    let modified = metadata
        .modified()
        .ok()
        .and_then(|t| {
            t.duration_since(std::time::UNIX_EPOCH)
                .ok()
                .map(|d| {
                    let secs = d.as_secs();
                    chrono::DateTime::from_timestamp(secs as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                })
        })
        .unwrap_or_else(|| "unknown".to_string());

    let created = metadata
        .created()
        .ok()
        .and_then(|t| {
            t.duration_since(std::time::UNIX_EPOCH)
                .ok()
                .map(|d| {
                    let secs = d.as_secs();
                    chrono::DateTime::from_timestamp(secs as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                })
        })
        .unwrap_or_else(|| "unknown".to_string());

    let basic = FileDetail {
        name,
        path: path.clone(),
        format: format.clone(),
        size: metadata.len(),
        modified,
        created,
        is_dir: metadata.is_dir(),
    };

    let exif = if metadata::is_image_file(&path) {
        metadata::read_exif(&path).ok()
    } else {
        None
    };

    let gps = exif.as_ref().and_then(|e| {
        if let (Some(lat), Some(lng)) = (e.gps_latitude, e.gps_longitude) {
            metadata::reverse_geocode(lat, lng).ok()
        } else {
            None
        }
    });

    Ok(FileDetailInfo {
        basic,
        exif,
        gps,
        ai_tags: vec![],
    })
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
pub fn get_log_stats(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    state.log_manager.get_log_stats()
}

#[tauri::command]
pub fn clean_expired_logs(
    state: State<'_, AppState>,
    retention_days: u32,
) -> Result<usize, String> {
    state.log_manager.clean_expired(retention_days)
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
pub fn get_app_info() -> serde_json::Value {
    serde_json::json!({
        "name": "轻羽归档",
        "version": "1.0.0",
        "description": "开源免费跨平台文件整理工具",
        "tech_stack": ["Tauri", "Rust", "Vue3", "TypeScript", "Element Plus"],
        "license": "MIT",
        "url": "https://github.com/YangShengzhou03/LeafTidy"
    })
}

#[tauri::command]
pub fn organize_files(
    source_dirs: Vec<String>,
    target_dir: String,
    rule: OrganizeRule,
) -> Result<Vec<OrganizeResult>, String> {
    file_ops::organize_files(&source_dirs, &target_dir, &rule)
}

#[tauri::command]
pub async fn organize_files_async(
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

/// 取消当前正在运行的操作
#[tauri::command]
pub fn cancel_operation() -> CancelResult {
    cancel::set_cancelled();
    CancelResult {
        cancelled: true,
        message: "操作已取消".to_string(),
    }
}

/// 获取取消标志状态（前端轮询用）
#[tauri::command]
pub fn is_operation_cancelled() -> bool {
    cancel::is_cancelled()
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
pub fn batch_rename(
    paths: Vec<String>,
    target_dir: String,
    rule: RenameRule,
) -> Result<Vec<RenameResult>, String> {
    file_ops::batch_rename(&paths, &target_dir, &rule)
}

/// 带进度事件的异步批量重命名
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

    // 发送完成进度
    let _ = app.emit("rename-progress", &TaskProgress {
        total,
        processed: total,
        current_item: None,
        percentage: 100.0,
    });

    result
}

#[tauri::command]
pub fn find_duplicates(
    paths: Vec<String>,
    detect_mode: String,
) -> Result<DuplicateScanResult, String> {
    file_ops::find_duplicates(&paths, &detect_mode)
}

/// 带进度事件的异步重复文件检测
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

    // 发送完成进度
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
pub fn scan_auxiliary_files(
    paths: Vec<String>,
    cleanup_types: Vec<String>,
) -> Result<Vec<CleanupResult>, String> {
    file_ops::scan_auxiliary_files(&paths, &cleanup_types)
}

/// 带进度事件的异步附属文件扫描
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
pub fn cleanup_auxiliary_files(
    files: Vec<String>,
) -> Result<BatchOperationResult, String> {
    file_ops::cleanup_auxiliary_files(files)
}

/// 带进度事件的异步附属文件清理
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

    // 发送完成进度
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
