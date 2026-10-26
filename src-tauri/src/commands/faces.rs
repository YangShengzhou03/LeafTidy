use crate::models::{ArchiveGroup, BatchOperationResult, OperationResult, Person, PersonGroup, TaskProgress};
use crate::tasks::{event, scope};

use super::{acquire_classify_lock, run_batch_task, ClassifyLockGuard};

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub async fn face_cluster_async(
    app: tauri::AppHandle,
    paths: Vec<String>,
) -> Result<Vec<PersonGroup>, String> {
    use tauri::Emitter;

    // ONNX Runtime 不支持并发会话
    if !acquire_classify_lock() {
        log::warn!("[人脸聚类] 拒绝启动：已有任务在运行");
        return Err("已有分类任务在运行，请等待完成或取消后重试".to_string());
    }
    let _lock_guard = ClassifyLockGuard;

    log::info!("[人脸聚类] 启动: {} 个目录", paths.len());
    let start = std::time::Instant::now();

    let progress_app = app.clone();
    let dir_count = paths.len();

    let result = run_batch_task(scope::FACES, move |token| {
        crate::faces::run_face_cluster(
            &paths,
            |progress: &TaskProgress| {
                let _ = progress_app.emit(event::FACE_CLUSTER_PROGRESS, progress);
            },
            token,
        )
    })
    .await;

    match &result {
        Ok(groups) => {
            log::info!("[人脸聚类] 完成: {dir_count} 个目录, {} 个人物, 耗时 {:?}", groups.len(), start.elapsed());
        }
        Err(e) => log::error!("[人脸聚类] 失败: {e}"),
    }

    result
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn rename_person(id: String, new_name: String) -> Result<Vec<Person>, String> {
    let name = new_name.trim().to_string();
    if name.is_empty() {
        return Err("人物名称不能为空".to_string());
    }
    let _guard = crate::faces::try_lock_persons()
        .ok_or_else(|| "人脸聚类进行中，请等待完成后再修改人物".to_string())?;
    let mut persons = crate::faces::load_persons();
    let Some(person) = persons.iter_mut().find(|p| p.id == id) else {
        return Err("人物不存在".to_string());
    };
    person.name = name;
    // 改名视为用户确认，下次聚类优先匹配
    person.confirmed = true;
    let display = person.name.clone();
    crate::faces::save_persons(&persons)?;
    log::info!("[人脸聚类] 人物改名: {id} -> {display}");
    Ok(persons)
}

#[allow(clippy::needless_pass_by_value, clippy::cast_precision_loss, clippy::suboptimal_flops)]
#[tauri::command]
pub fn merge_persons(target_id: String, source_id: String) -> Result<Vec<Person>, String> {
    if target_id == source_id {
        return Err("不能合并同一个人".to_string());
    }
    let _guard = crate::faces::try_lock_persons()
        .ok_or_else(|| "人脸聚类进行中，请等待完成后再合并人物".to_string())?;
    let mut persons = crate::faces::load_persons();
    let Some(ti) = persons.iter().position(|p| p.id == target_id) else {
        return Err("目标人物不存在".to_string());
    };
    let Some(si) = persons.iter().position(|p| p.id == source_id) else {
        return Err("来源人物不存在".to_string());
    };

    // 加权质心合并：c = (c_t·n_t + c_s·n_s) / (n_t + n_s)
    let source = persons[si].clone();
    {
        let t = &mut persons[ti];
        let nt = t.photo_count as f32;
        let ns = source.photo_count as f32;
        let total = nt + ns;
        if total > 0.0 {
            for (c, v) in t.centroid.iter_mut().zip(&source.centroid) {
                *c = (*c * nt + v * ns) / total;
            }
        }
        t.photo_count += source.photo_count;
        t.confirmed = true;
        // 合并后的质心必须重新归一化，否则后续 cosine 相似度会随合并次数失准
        crate::faces::normalize_vector(&mut t.centroid);
    }
    persons.remove(si);

    crate::faces::save_persons(&persons)?;
    log::info!("[人脸聚类] 合并人物: {source_id} -> {target_id}");
    Ok(persons)
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub async fn archive_by_person(
    output_dir: String,
    groups: Vec<ArchiveGroup>,
) -> Result<BatchOperationResult, String> {
    log::info!("[人脸聚类] 按人物归档: {} 组 -> {output_dir}", groups.len());
    let start = std::time::Instant::now();
    let result = run_batch_task(scope::FACES, move |token| archive_by_person_sync(token, &output_dir, &groups)).await;

    match &result {
        Ok(r) => {
            log::info!("[人脸聚类] 按人物归档完成: 成功 {}/{} 个文件, 耗时 {:?}", r.success_count, r.total, start.elapsed());
        }
        Err(e) => log::error!("[人脸聚类] 按人物归档失败: {e}"),
    }

    result
}

/// 按人物复制到 `output_dir/<人物名>/`，重名自动追加序号
fn archive_by_person_sync(
    token: &crate::cancel::CancelToken,
    output_dir: &str,
    groups: &[ArchiveGroup],
) -> Result<BatchOperationResult, String> {
    crate::fs_util::ensure_absolute_dir(output_dir)?;
    let total: usize = groups.iter().map(|g| g.paths.len()).sum();
    let mut results = Vec::new();
    let mut success_count = 0usize;
    let mut total_size = 0u64;

    for group in groups {
        let dir = std::path::Path::new(output_dir).join(crate::fs_util::sanitize_filename(&group.name));
        std::fs::create_dir_all(&dir).map_err(|e| format!("创建目录失败: {e}"))?;

        for path_str in &group.paths {
            crate::cancel::consume(token)?;
            let source = std::path::Path::new(path_str);
            if !source.is_absolute() || !source.is_file() {
                results.push(OperationResult {
                    success: false,
                    message: None,
                    error: Some(format!("非法或不存在源文件: {path_str}")),
                });
                continue;
            }
            let file_name = source
                .file_name()
                .map_or_else(|| "unknown".to_string(), |n| n.to_string_lossy().to_string());
            let dest = crate::fs_util::resolve_duplicate_filename(&dir, &file_name).1;

            match std::fs::copy(source, &dest) {
                Ok(size) => {
                    success_count += 1;
                    total_size += size;
                    results.push(OperationResult { success: true, message: None, error: None });
                }
                Err(e) => {
                    log::error!("归档复制失败 {path_str}: {e}");
                    results.push(OperationResult {
                        success: false,
                        message: None,
                        error: Some(e.to_string()),
                    });
                }
            }
        }
    }

    Ok(BatchOperationResult {
        total,
        success_count,
        fail_count: total - success_count,
        total_size,
        results,
    })
}
