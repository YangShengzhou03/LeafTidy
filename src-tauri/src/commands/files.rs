use crate::file_ops;
use crate::image_util;
use crate::models::{BatchOperationResult, DirectoryStats, FaceCrop, FileEntry};

/// 按需生成 96px 缩略图 data URL，避免扫描结果内联大量 base64 挤爆 IPC 与内存。
#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub async fn get_thumbnail(path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let img = image_util::load_image_limited(&path, 320)?;
        image_util::make_thumb_jpeg(img, 96).ok_or_else(|| "缩略图生成失败".to_string())
    })
    .await
    .map_err(|e| format!("任务执行异常: {e}"))?
}

/// 按需生成人脸裁剪缩略图。crop 为相对整图的比例，先按比例裁出人脸再缩到 96px。
#[allow(clippy::needless_pass_by_value, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
#[tauri::command]
pub async fn get_face_thumbnail(path: String, crop: FaceCrop) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        // 检测时最多解码到 320px，这里用同一量级，保证比例换算一致且内存可控
        let img = image_util::load_image_limited(&path, 320)?;
        let (img_w, img_h) = (img.width() as f32, img.height() as f32);
        let x = (crop.x.clamp(0.0, 1.0) * img_w).round().max(0.0);
        let y = (crop.y.clamp(0.0, 1.0) * img_h).round().max(0.0);
        let w = (crop.w.clamp(0.0, 1.0) * img_w).round().min(img_w - x).max(1.0);
        let h = (crop.h.clamp(0.0, 1.0) * img_h).round().min(img_h - y).max(1.0);
        let face = img.crop_imm(x as u32, y as u32, w as u32, h as u32);
        image_util::make_thumb_jpeg(face, 96).ok_or_else(|| "人脸缩略图生成失败".to_string())
    })
    .await
    .map_err(|e| format!("任务执行异常: {e}"))?
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub async fn scan_directory(path: String) -> Result<Vec<FileEntry>, String> {
    log::info!("扫描目录: {path}");
    let start = std::time::Instant::now();
    let path_buf = std::path::PathBuf::from(path);
    let result = tauri::async_runtime::spawn_blocking(move || file_ops::scan_directory(&path_buf))
        .await
        .map_err(|e| format!("任务执行异常: {e}"))?;
    match &result {
        Ok(entries) => log::info!("扫描完成: {} 个文件, 耗时 {:?}", entries.len(), start.elapsed()),
        Err(e) => log::error!("扫描失败: {e}"),
    }
    result
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub async fn get_directory_stats(paths: Vec<String>) -> Result<DirectoryStats, String> {
    log::info!("获取目录统计: {} 个路径", paths.len());
    let result = tauri::async_runtime::spawn_blocking(move || file_ops::get_directory_stats(paths))
        .await
        .map_err(|e| format!("任务执行异常: {e}"))?;
    if let Ok(ref stats) = result {
        log::info!("目录统计: {} 文件, {} 目录, {} 字节", stats.total_files, stats.total_dirs, stats.total_size);
    }
    result
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub async fn move_files_batch(
    app: tauri::AppHandle,
    sources: Vec<String>,
    target: String,
) -> Result<BatchOperationResult, String> {
    use tauri::Emitter;
    use crate::tasks::{event, scope};

    log::info!("批量移动: {} 个文件 -> {target}", sources.len());

    // 移动会删除源文件，属于不可逆操作：走任务骨架以获得取消与退出保护
    let task_app = app.clone();
    let result = crate::commands::run_batch_task(scope::MOVE, move |token| {
        crate::file_ops::move_files_batch(token, &sources, &target, |progress| {
            let _ = task_app.emit(event::MOVE_PROGRESS, progress);
        })
    })
    .await;

    match &result {
        Ok(r) => log::info!("批量移动完成: {}/{} 个文件", r.success_count, r.total),
        Err(e) => log::error!("移动失败: {e}"),
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

/// 用系统默认浏览器打开链接。仅接受 http/https，避免命令行注入。
#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
    if !url.starts_with("https://") && !url.starts_with("http://") {
        return Err(format!("不支持的链接: {url}"));
    }
    log::debug!("打开链接: {url}");

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("打开浏览器失败: {e}"))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("打开浏览器失败: {e}"))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("打开浏览器失败: {e}"))?;
    }

    Ok(())
}
