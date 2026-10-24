use crate::metadata;
use crate::models::ImageProcessResult;
use crate::tasks::{event, scope};

use super::run_batch_task;

#[tauri::command]
pub async fn fix_date_taken_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    target_dir: String,
    date_source: String,
    specified_time: Option<String>,
) -> Result<Vec<ImageProcessResult>, String> {
    use tauri::Emitter;

    log::info!("修复拍摄时间: {} 张图片 -> {target_dir}, 来源: {date_source}", paths.len());
    let start = std::time::Instant::now();

    let total = paths.len();
    let result = run_batch_task(scope::FIX_DATE, move |token| {
        metadata::fix_date_taken_batch(token, &paths, &target_dir, &date_source, specified_time.as_deref(), |progress| {
            let _ = app.emit(event::FIX_DATE_PROGRESS, progress);
        })
    })
    .await;

    let success = result.as_ref().map_or(0, |results| results.iter().filter(|r| r.success).count());
    log::info!("修复拍摄时间完成: {success}/{total} 张成功, 耗时 {:?}", start.elapsed());

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

    log::info!("EXIF 脱敏: {} 张图片 -> {target_dir}, 选项: exif={}, gps={}, camera={}, xmp={}, ps={}",
        paths.len(), options.exif, options.gps, options.camera, options.xmp, options.photoshop);
    let start = std::time::Instant::now();

    let total = paths.len();
    let result = run_batch_task(scope::EXIF_CLEAN, move |token| {
        metadata::strip_metadata_batch(token, &paths, &target_dir, options, |progress| {
            let _ = app.emit(event::EXIF_CLEAN_PROGRESS, progress);
        })
    })
    .await;

    let success = result.as_ref().map_or(0, |results| results.iter().filter(|r| r.success).count());
    log::info!("EXIF 脱敏完成: {success}/{total} 张成功, 耗时 {:?}", start.elapsed());

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

    log::info!("写入 GPS: {} 张图片 -> {target_dir}, 坐标: ({latitude}, {longitude})", paths.len());
    let start = std::time::Instant::now();

    let total = paths.len();
    let result = run_batch_task(scope::WRITE_GPS, move |token| {
        metadata::write_gps_batch(token, &paths, &target_dir, latitude, longitude, |progress| {
            let _ = app.emit(event::WRITE_GPS_PROGRESS, progress);
        })
    })
    .await;

    let success = result.as_ref().map_or(0, |results| results.iter().filter(|r| r.success).count());
    log::info!("写入 GPS 完成: {success}/{total} 张成功, 耗时 {:?}", start.elapsed());

    result
}
