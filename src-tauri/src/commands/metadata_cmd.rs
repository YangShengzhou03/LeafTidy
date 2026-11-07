use crate::metadata;
use crate::models::ImageProcessResult;
use crate::tasks::{event, scope};

use super::run_batch_task;

#[tauri::command]
/// 修复图片拍摄时间（按文件名日期/指定时间/EXIF 最早时间重写 EXIF）。
///
/// # Errors
///
/// - 目标目录或源路径不是绝对路径时返回参数错误
/// - 图片解码/重编码失败或任务被取消时返回错误
pub async fn fix_date_taken_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    target_dir: String,
    date_source: String,
    specified_time: Option<String>,
) -> Result<Vec<ImageProcessResult>, String> {
    use tauri::Emitter;

    log::info!("修复拍摄时间: {} 张图片 -> {target_dir}, 来源: {date_source}", paths.len());
    // 信任边界校验：目标目录必须绝对，源文件必须为绝对路径文件
    crate::fs_util::ensure_absolute_dir(&target_dir)?;
    crate::fs_util::ensure_absolute_non_dirs(&paths)?;
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

/// 去除 EXIF / XMP 等隐私元数据（仅保留编码必需段），输出临时文件路径返回。
///
/// # Errors
///
/// - 目标目录或源路径不是绝对路径时返回参数错误
/// - 图片解码/重编码失败或任务被取消时返回错误
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
    crate::fs_util::ensure_absolute_dir(&target_dir)?;
    crate::fs_util::ensure_absolute_non_dirs(&paths)?;
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
/// 批量写入 GPS 坐标到 EXIF。失败的文件跳过并记录错误日志。
///
/// # Errors
///
/// - 存在非绝对路径的条目时返回参数错误
/// - 参数校验失败时返回错误
pub async fn write_gps_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    target_dir: String,
    latitude: f64,
    longitude: f64,
) -> Result<Vec<ImageProcessResult>, String> {
    use tauri::Emitter;

    log::info!("写入 GPS: {} 张图片 -> {target_dir}, 坐标: ({latitude}, {longitude})", paths.len());
    // 信任边界校验：目标目录必须绝对，源文件必须为绝对路径文件
    crate::fs_util::ensure_absolute_dir(&target_dir)?;
    crate::fs_util::ensure_absolute_non_dirs(&paths)?;
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
