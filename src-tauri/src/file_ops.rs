use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::fs_util::system_time_to_string;
use crate::models::{BatchOperationResult, DirectoryStats, FileEntry, OperationResult};

#[allow(clippy::unnecessary_wraps)]
pub fn scan_directory(path: &PathBuf) -> Result<Vec<FileEntry>, String> {
    log::info!("读取目录: {}", path.display());
    let start = std::time::Instant::now();
    let mut entries = Vec::new();
    let mut errors = 0u64;

    for entry in walkdir::WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            !e.file_name()
                .to_string_lossy()
                .starts_with('.')
        })
    {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                log::debug!("扫描条目失败: {e}");
                errors += 1;
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let metadata = match fs::metadata(entry.path()) {
            Ok(m) => m,
            Err(e) => {
                log::debug!("获取元数据失败 {}: {e}", entry.path().display());
                errors += 1;
                continue;
            }
        };
        let name = entry.file_name().to_string_lossy().to_string();
        let path_str = entry.path().to_string_lossy().to_string();

        let format = entry
            .path()
            .extension()
            .map_or_else(|| "unknown".to_string(), |ext| ext.to_string_lossy().to_lowercase());

        let modified = metadata
            .modified()
            .ok()
            .map_or_else(|| "unknown".to_string(), system_time_to_string);
        let created = metadata
            .created()
            .ok()
            .map_or_else(|| "unknown".to_string(), system_time_to_string);

        entries.push(FileEntry {
            name,
            path: path_str,
            is_dir: false,
            size: metadata.len(),
            format,
            modified,
            created,
        });
    }

    if errors > 0 {
        log::warn!("读取 {} 完成: {} 文件, {} 个错误, 耗时 {:?}", path.display(), entries.len(), errors, start.elapsed());
    } else {
        log::info!("读取 {} 完成: {} 文件, 耗时 {:?}", path.display(), entries.len(), start.elapsed());
    }

    Ok(entries)
}

pub fn get_directory_stats(paths: Vec<String>) -> Result<DirectoryStats, String> {
    log::info!("统计目录: {} 个路径", paths.len());
    let start = std::time::Instant::now();
    let mut total_files = 0u64;
    let mut total_dirs = 0u64;
    let mut total_size = 0u64;
    let mut file_types: HashMap<String, u64> = HashMap::new();
    let mut oldest_file: Option<String> = None;
    let mut newest_file: Option<String> = None;
    let mut oldest_time: Option<std::time::SystemTime> = None;
    let mut newest_time: Option<std::time::SystemTime> = None;

    for path_str in paths {
        let path = Path::new(&path_str);
        if !path.exists() {
            continue;
        }

        for entry in walkdir::WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !e.file_name().to_string_lossy().starts_with('.'))
        {
            let entry = entry.map_err(|e| format!("扫描目录失败: {e}"))?;

            if entry.file_type().is_file() {
                total_files += 1;

                let metadata = fs::metadata(entry.path())
                    .map_err(|e| format!("获取元数据失败: {e}"))?;
                total_size += metadata.len();

                let format = entry
                    .path()
                    .extension()
                    .map_or_else(|| "unknown".to_string(), |e| e.to_string_lossy().to_lowercase());
                *file_types.entry(format).or_insert(0) += 1;

                if let Ok(modified) = metadata.modified() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if oldest_time.is_none() || modified < *oldest_time.as_ref().unwrap() {
                        oldest_time = Some(modified);
                        oldest_file = Some(file_name.clone());
                    }
                    if newest_time.is_none() || modified > *newest_time.as_ref().unwrap() {
                        newest_time = Some(modified);
                        newest_file = Some(file_name);
                    }
                }
            } else if entry.file_type().is_dir() {
                total_dirs += 1;
            }
        }
    }

    log::info!("统计完成: {} 文件, {} 目录, {} 字节, 耗时 {:?}", total_files, total_dirs, total_size, start.elapsed());

    Ok(DirectoryStats {
        total_files,
        total_dirs,
        total_size,
        file_types,
        oldest_file,
        newest_file,
    })
}

pub fn copy_files_batch(
    sources: &[String],
    target: &str,
) -> Result<BatchOperationResult, String> {
    let target_path = Path::new(target);
    if !target_path.exists() {
        fs::create_dir_all(target_path).map_err(|e| format!("创建目标目录失败: {e}"))?;
    }

    let mut results = Vec::new();
    let mut success_count = 0;
    let mut fail_count = 0;

    for source_str in sources {
        let source_path = Path::new(source_str);
        if !source_path.exists() {
            log::warn!("源文件不存在，跳过: {source_str}");
            results.push(OperationResult {
                success: false,
                message: None,
                error: Some("文件不存在".to_string()),
            });
            fail_count += 1;
            continue;
        }

        let file_name = source_path
            .file_name()
            .map_or_else(|| "unknown".to_string(), |n| n.to_string_lossy().to_string());

        let dest_path = target_path.join(&file_name);

        let final_dest = if dest_path.exists() {
            let mut counter = 1;
            let stem = source_path
                .file_stem()
                .map_or_else(|| "file".to_string(), |s| s.to_string_lossy().to_string());
            let ext = source_path
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();

            loop {
                let new_name = format!("{stem}_{counter}{ext}");
                let new_path = target_path.join(&new_name);
                if !new_path.exists() {
                    break new_path;
                }
                counter += 1;
            }
        } else {
            dest_path
        };

        let dest_str = final_dest.to_string_lossy().to_string();

        match fs::copy(source_path, &final_dest) {
            Ok(_) => {
                log::info!("复制: {source_str} -> {dest_str}");
                results.push(OperationResult {
                    success: true,
                    message: Some(format!("已复制: {source_str} -> {dest_str}")),
                    error: None,
                });
                success_count += 1;
            }
            Err(e) => {
                log::error!("复制失败 {source_str} -> {target}: {e}");
                results.push(OperationResult {
                    success: false,
                    message: None,
                    error: Some(e.to_string()),
                });
                fail_count += 1;
            }
        }
    }

    log::info!("批量复制完成: 成功 {success_count}, 失败 {fail_count} -> {target}");

    Ok(BatchOperationResult {
        total: results.len(),
        success_count,
        fail_count,
        total_size: 0,
        results,
    })
}
