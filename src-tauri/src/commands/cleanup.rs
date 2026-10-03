use tauri::State;

use crate::models::{BatchOperationResult, LowQualityScanResult, TaskProgress};

use super::{run_batch_task, AppState};

#[tauri::command]
#[allow(clippy::cast_possible_truncation)]
pub async fn scan_low_quality_images_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
    detect_types: Vec<String>,
    variance_threshold: f64,
    min_dimension: u64,
) -> Result<LowQualityScanResult, String> {
    use tauri::Emitter;

    log::info!(
        "扫描低质图片: {} 个目录, 检测类型: {:?}, 模糊阈值: {}, 最小尺寸: {}",
        paths.len(),
        detect_types,
        variance_threshold,
        min_dimension
    );

    let task_app = app.clone();
    let result = run_batch_task(move |token| {
        crate::cleanup::scan_low_quality_images_with_progress(
            token,
            &paths,
            &detect_types,
            variance_threshold,
            min_dimension as u32,
            |progress| {
                let _ = app.emit("lowquality-scan-progress", progress);
            },
            |img| {
                let _ = task_app.emit("lowquality-scan-result", img);
            },
        )
    })
    .await;

    match &result {
        Ok(scan_result) => {
            log::info!(
                "扫描低质图片完成: {} 张图片中发现 {} 张低质",
                scan_result.total_scanned,
                scan_result.images.len()
            );
        }
        Err(e) => log::error!("扫描低质图片失败: {e}"),
    }

    result
}

#[tauri::command]
pub async fn delete_low_quality_images_async(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    files: Vec<String>,
) -> Result<BatchOperationResult, String> {
    use tauri::Emitter;

    log::info!("删除低质图片: {} 个文件", files.len());
    let start = std::time::Instant::now();

    let total = files.len() as u64;
    let file_count = files.len();
    let task_app = app.clone();
    let result = run_batch_task(move |token| {
        crate::cleanup::delete_low_quality_images_with_progress(token, &files, |progress| {
            let _ = task_app.emit("lowquality-delete-progress", progress);
        })
    })
    .await;

    if let Ok(ref result) = result {
        log::info!("删除低质图片完成: {}/{} 文件成功, 耗时 {:?}", result.success_count, file_count, start.elapsed());
        let _ = app.emit(
            "lowquality-delete-progress",
            &TaskProgress {
                total,
                processed: (result.success_count + result.fail_count) as u64,
                current_item: None,
                percentage: 100.0,
            },
        );
        let _ = state.log_manager.write_event(
            &format!("低质图片删除: {} 个文件", result.success_count),
            result.total_size,
        );
    }

    result
}
