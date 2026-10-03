use tauri::State;

use crate::metadata;
use crate::models::ImageProcessResult;

use super::{run_batch_task, AppState};

#[tauri::command]
pub async fn fix_date_taken_async(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    paths: Vec<String>,
    target_dir: String,
    date_source: String,
    specified_time: Option<String>,
) -> Result<Vec<ImageProcessResult>, String> {
    use tauri::Emitter;

    log::info!("修复拍摄时间: {} 张图片 -> {target_dir}, 来源: {date_source}", paths.len());
    let start = std::time::Instant::now();

    let total = paths.len();
    let result = run_batch_task(move |token| {
        metadata::fix_date_taken_batch(token, &paths, &target_dir, &date_source, specified_time.as_deref(), |progress| {
            let _ = app.emit("image-process-progress", progress);
        })
    })
    .await;

    match &result {
        Ok(results) => {
            let success = results.iter().filter(|r| r.success).count();
            let total_size: u64 = results.iter().filter(|r| r.success).map(|r| r.source_path.len() as u64).sum();
            log::info!("修复拍摄时间完成: {}/{} 张成功, 耗时 {:?}", success, total, start.elapsed());
            let _ = state.log_manager.write_event(
                &format!("修复拍摄时间: {success}/{total} 张"),
                total_size,
            );
        }
        Err(e) => log::error!("修复拍摄时间失败: {e}"),
    }

    result
}

#[tauri::command]
pub async fn strip_exif_async(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    paths: Vec<String>,
    target_dir: String,
    options: metadata::StripOptions,
) -> Result<Vec<ImageProcessResult>, String> {
    use tauri::Emitter;

    log::info!("EXIF 脱敏: {} 张图片 -> {target_dir}, 选项: exif={}, gps={}, camera={}, xmp={}, ps={}",
        paths.len(), options.exif, options.gps, options.camera, options.xmp, options.photoshop);
    let start = std::time::Instant::now();

    let total = paths.len();
    let result = run_batch_task(move |token| {
        metadata::strip_metadata_batch(token, &paths, &target_dir, options, |progress| {
            let _ = app.emit("image-process-progress", progress);
        })
    })
    .await;

    match &result {
        Ok(results) => {
            let success = results.iter().filter(|r| r.success).count();
            log::info!("EXIF 脱敏完成: {}/{} 张成功, 耗时 {:?}", success, total, start.elapsed());
            let _ = state.log_manager.write_event(
                &format!("EXIF 脱敏: {success}/{total} 张"),
                0,
            );
        }
        Err(e) => log::error!("EXIF 脱敏失败: {e}"),
    }

    result
}

#[tauri::command]
pub async fn write_gps_async(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    paths: Vec<String>,
    target_dir: String,
    latitude: f64,
    longitude: f64,
) -> Result<Vec<ImageProcessResult>, String> {
    use tauri::Emitter;

    log::info!("写入 GPS: {} 张图片 -> {target_dir}, 坐标: ({latitude}, {longitude})", paths.len());
    let start = std::time::Instant::now();

    let total = paths.len();
    let result = run_batch_task(move |token| {
        metadata::write_gps_batch(token, &paths, &target_dir, latitude, longitude, |progress| {
            let _ = app.emit("image-process-progress", progress);
        })
    })
    .await;

    match &result {
        Ok(results) => {
            let success = results.iter().filter(|r| r.success).count();
            log::info!("写入 GPS 完成: {}/{} 张成功, 耗时 {:?}", success, total, start.elapsed());
            let _ = state.log_manager.write_event(
                &format!("写入 GPS: {success}/{total} 张"),
                0,
            );
        }
        Err(e) => log::error!("写入 GPS 失败: {e}"),
    }

    result
}
