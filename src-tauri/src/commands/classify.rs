use tauri::State;

use crate::models::{ClassifyProgress, ClassifyResult, ClassifyRule, ModelDefinition};

use super::{acquire_classify_lock, run_batch_task, AppState, ClassifyLockGuard};

#[tauri::command]
pub fn list_builtin_models() -> Vec<ModelDefinition> {
    crate::classify::list_builtin_models()
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub async fn classify_images_async(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    model_source: String,
    is_builtin: bool,
    image_paths: Vec<String>,
    rule: ClassifyRule,
    output_dir: String,
) -> Result<Vec<ClassifyResult>, String> {
    use tauri::Emitter;

    // 并发锁：ONNX Runtime 不支持并发会话
    if !acquire_classify_lock() {
        log::warn!("[AI分类] 拒绝启动：已有分类任务在运行");
        let _ = state.log_manager.write_event(
            "[AI分类] 拒绝启动：已有任务在运行",
            0,
        );
        return Err("已有分类任务在运行，请等待完成或取消后重试".to_string());
    }

    // 确保锁在任务结束时释放（即使 panic 也会通过 Drop 释放）
    let _lock_guard = ClassifyLockGuard;

    let total = image_paths.len();
    log::info!("[AI分类] 启动: {total} 张图片 -> {output_dir}, 模型: {model_source} (内置: {is_builtin})");
    let _ = state.log_manager.write_event(
        &format!("[AI分类] 启动: {total} 张图片"),
        0,
    );

    let log_manager = state.log_manager.clone();
    let progress_app = app.clone();

    let result = run_batch_task(move |token| {
        crate::classify::classify_images_batch(
            &model_source,
            is_builtin,
            &image_paths,
            &rule,
            &output_dir,
            |log_msg: &str| {
                let _ = log_manager.write_event(log_msg, 0);
            },
            |progress: &ClassifyProgress| {
                let _ = progress_app.emit("classify-progress", progress);
            },
            token,
        )
    })
    .await;

    if result.is_ok() {
        let _ = app.emit("classify-progress", &ClassifyProgress {
            total: total as u64,
            processed: total as u64,
            current_item: None,
            percentage: 100.0,
            current_category: None,
            result: None,
        });
    }

    match &result {
        Ok(results) => {
            let success = results.iter().filter(|r| r.error.is_none()).count();
            let fail = results.len() - success;
            let _ = state.log_manager.write_event(
                &format!("[AI分类] 完成: 成功 {success} 张, 失败 {fail} 张"),
                0,
            );
        }
        Err(e) => {
            log::error!("[AI分类] 失败: {e}");
            let _ = state.log_manager.write_event(
                &format!("[AI分类] 失败: {e}"),
                0,
            );
        }
    }

    result
}

#[tauri::command]
pub fn validate_model_path(model_path: &str) -> bool {
    log::debug!("验证模型路径: {model_path}");
    crate::classify::validate_model(model_path)
}
