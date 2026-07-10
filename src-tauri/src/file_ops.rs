use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use crate::models::*;
use crate::metadata;

pub fn list_directory(path_str: &str) -> Result<Vec<FileEntry>, String> {
    let path = Path::new(path_str);
    if !path.is_dir() {
        return Err(format!("路径不是有效目录: {}", path_str));
    }

    let mut entries = Vec::new();
    let mut dir = fs::read_dir(path).map_err(|e| format!("读取目录失败: {}", e))?;

    while let Some(entry) = dir.next() {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let metadata = entry.metadata().map_err(|e| format!("获取元数据失败: {}", e))?;

        let name = entry.file_name().to_string_lossy().to_string();
        let path_buf = entry.path();
        let path_str = path_buf.to_string_lossy().to_string();
        let is_dir = metadata.is_dir();
        let size = if is_dir { 0 } else { metadata.len() };

        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                    let secs = d.as_secs();
                    let dt = chrono::DateTime::from_timestamp(secs as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    dt
                })
            })
            .unwrap_or_else(|| "unknown".to_string());

        let created = metadata
            .created()
            .ok()
            .and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                    let secs = d.as_secs();
                    let dt = chrono::DateTime::from_timestamp(secs as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    dt
                })
            })
            .unwrap_or_else(|| "unknown".to_string());

        let format = if is_dir {
            "folder".to_string()
        } else {
            match path_buf.extension() {
                Some(ext) => ext.to_string_lossy().to_lowercase(),
                None => "unknown".to_string(),
            }
        };

        entries.push(FileEntry {
            name,
            path: path_str,
            is_dir,
            size,
            format,
            modified,
            created,
        });
    }

    entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));

    Ok(entries)
}

pub fn scan_directory(path: &PathBuf) -> Result<Vec<FileEntry>, String> {
    let mut entries = Vec::new();

    for entry in walkdir::WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            !e.file_name()
                .to_string_lossy()
                .starts_with('.')
        })
    {
        let entry = entry.map_err(|e| format!("扫描目录失败: {}", e))?;

        if !entry.file_type().is_file() {
            continue;
        }

        let metadata = fs::metadata(entry.path()).map_err(|e| format!("获取元数据失败: {}", e))?;
        let name = entry.file_name().to_string_lossy().to_string();
        let path_str = entry.path().to_string_lossy().to_string();

        let format = match entry.path().extension() {
            Some(ext) => ext.to_string_lossy().to_lowercase(),
            None => "unknown".to_string(),
        };

        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
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
                t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                    let secs = d.as_secs();
                    chrono::DateTime::from_timestamp(secs as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                })
            })
            .unwrap_or_else(|| "unknown".to_string());

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

    Ok(entries)
}

pub fn get_directory_stats(paths: Vec<String>) -> Result<DirectoryStats, String> {
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
            let entry = entry.map_err(|e| format!("扫描目录失败: {}", e))?;

            if entry.file_type().is_file() {
                total_files += 1;

                let metadata = fs::metadata(entry.path())
                    .map_err(|e| format!("获取元数据失败: {}", e))?;
                total_size += metadata.len();

                let format = entry
                    .path()
                    .extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
                    .unwrap_or_else(|| "unknown".to_string());
                *file_types.entry(format).or_insert(0) += 1;

                if let Ok(modified) = metadata.modified() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if oldest_time.is_none() || modified < oldest_time.unwrap() {
                        oldest_time = Some(modified);
                        oldest_file = Some(file_name.clone());
                    }
                    if newest_time.is_none() || modified > newest_time.unwrap() {
                        newest_time = Some(modified);
                        newest_file = Some(file_name);
                    }
                }
            } else if entry.file_type().is_dir() {
                total_dirs += 1;
            }
        }
    }

    Ok(DirectoryStats {
        total_files,
        total_dirs,
        total_size,
        file_types,
        oldest_file,
        newest_file,
    })
}

pub fn list_subdirs(path_str: &str) -> Result<Vec<DirEntry>, String> {
    let path = Path::new(path_str);

    if path_str.is_empty() || path_str == "/" || path_str == "\\" {
        return list_root_directories();
    }
    
    if !path.is_dir() {
        return Err(format!("路径不是有效目录: {}", path_str));
    }

    let mut entries = Vec::new();
    let dir = fs::read_dir(path).map_err(|e| format!("读取目录失败: {}", e))?;

    for entry in dir {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let entry_path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') || name.starts_with('$') {
            continue;
        }

        if entry_path.is_dir() {
            entries.push(DirEntry {
                name,
                path: entry_path.to_string_lossy().to_string(),
                is_dir: true,
                children: None,
            });
        }
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(entries)
}

fn list_root_directories() -> Result<Vec<DirEntry>, String> {
    let mut entries = Vec::new();

    for letter in 'A'..'Z' {
        let drive_path = format!("{}:\\", letter);
        if Path::new(&drive_path).exists() {
            entries.push(DirEntry {
                name: format!("本地磁盘 ({})", letter),
                path: drive_path,
                is_dir: true,
                children: None,
            });
        }
    }
    
    Ok(entries)
}

pub fn rename_file(source: &str, new_name: &str) -> Result<String, String> {
    let source_path = Path::new(source);
    if !source_path.exists() {
        return Err(format!("文件不存在: {}", source));
    }

    let parent = source_path.parent().ok_or("无法获取父目录")?;
    let target_path = parent.join(new_name);

    if target_path.exists() {
        return Err(format!("目标文件已存在: {}", new_name));
    }

    fs::rename(source_path, &target_path)
        .map_err(|e| format!("重命名失败: {}", e))?;

    Ok(target_path.to_string_lossy().to_string())
}

pub fn delete_to_trash(paths: Vec<String>) -> BatchOperationResult {
    let mut results = Vec::new();
    let mut success_count = 0;
    let mut fail_count = 0;

    for path_str in paths {
        let path = Path::new(&path_str);
        if !path.exists() {
            results.push(OperationResult {
                success: true,
                message: Some("文件不存在，跳过".to_string()),
                error: None,
            });
            success_count += 1;
            continue;
        }

        match trash::delete(path) {
            Ok(_) => {
                results.push(OperationResult {
                    success: true,
                    message: Some(format!("已移至回收站: {}", path_str)),
                    error: None,
                });
                success_count += 1;
            }
            Err(e) => {
                results.push(OperationResult {
                    success: false,
                    message: None,
                    error: Some(format!("移入回收站失败: {}", e)),
                });
                fail_count += 1;
            }
        }
    }

    BatchOperationResult {
        total: results.len(),
        success_count,
        fail_count,
        results,
    }
}

pub fn compute_md5(path: &str) -> Result<String, String> {
    use md5::{Digest, Md5};
    use std::io::Read;

    let mut file = fs::File::open(path).map_err(|e| format!("打开文件失败: {}", e))?;
    let mut hasher = Md5::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).map_err(|e| format!("读取文件失败: {}", e))?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn sanitize_filename(name: &str) -> String {
    let illegal_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    name.chars()
        .map(|c| if illegal_chars.contains(&c) { '_' } else { c })
        .collect()
}

pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}

pub fn organize_files(
    source_dirs: &[String],
    target_dir: &str,
    rule: &OrganizeRule,
) -> Result<Vec<OrganizeResult>, String> {
    let target_path = Path::new(target_dir);
    if !target_path.exists() {
        fs::create_dir_all(target_path).map_err(|e| format!("创建目标目录失败: {}", e))?;
    }

    let mut results = Vec::new();

    for source_dir in source_dirs {
        let source_path = Path::new(source_dir);
        if !source_path.exists() || !source_path.is_dir() {
            continue;
        }

        for entry in walkdir::WalkDir::new(source_path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !e.file_name().to_string_lossy().starts_with('.'))
        {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !entry.file_type().is_file() {
                continue;
            }

            let file_path = entry.path();
            let file_path_str = file_path.to_string_lossy().to_string();

            let start_time = std::time::Instant::now();
            let file_metadata = collect_file_metadata(file_path, rule);

            let target_subpath = match build_target_path(file_path, target_path, rule) {
                Ok(p) => p,
                Err(e) => {
                    let elapsed_ms = start_time.elapsed().as_millis() as u64;
                    results.push(OrganizeResult {
                        source_path: file_path_str,
                        target_path: String::new(),
                        success: false,
                        error: Some(e),
                        metadata: file_metadata,
                        process_time_ms: Some(elapsed_ms),
                    });
                    continue;
                }
            };

            if let Some(parent) = target_subpath.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    let elapsed_ms = start_time.elapsed().as_millis() as u64;
                    results.push(OrganizeResult {
                        source_path: file_path_str,
                        target_path: target_subpath.to_string_lossy().to_string(),
                        success: false,
                        error: Some(format!("创建目录失败: {}", e)),
                        metadata: file_metadata,
                        process_time_ms: Some(elapsed_ms),
                    });
                    continue;
                }
            }

            let target_path_str = target_subpath.to_string_lossy().to_string();
            let op_result = fs::copy(file_path, &target_subpath)
                .map(|_| ())
                .map_err(|e| format!("复制文件失败: {}", e));

            // 计算处理耗时
            let elapsed_ms = start_time.elapsed().as_millis() as u64;

            match op_result {
                Ok(_) => {
                    results.push(OrganizeResult {
                        source_path: file_path_str,
                        target_path: target_path_str,
                        success: true,
                        error: None,
                        metadata: file_metadata,
                        process_time_ms: Some(elapsed_ms),
                    });
                }
                Err(e) => {
                    results.push(OrganizeResult {
                        source_path: file_path_str,
                        target_path: target_path_str,
                        success: false,
                        error: Some(e),
                        metadata: file_metadata,
                        process_time_ms: Some(elapsed_ms),
                    });
                }
            }
        }
    }

    Ok(results)
}

fn collect_file_metadata(file_path: &Path, _rule: &OrganizeRule) -> Option<OrganizeMetadata> {
    let metadata = fs::metadata(file_path).ok()?;
    let path_str = file_path.to_string_lossy().to_string();
    let is_media_file = metadata::is_image_file(&path_str) || metadata::is_video_file(&path_str);
    
    let modified = metadata.modified().ok().and_then(|t| {
        t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
            let secs = d.as_secs();
            chrono::DateTime::from_timestamp(secs as i64, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "unknown".to_string())
        })
    });
    
    let created = metadata.created().ok().and_then(|t| {
        t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
            let secs = d.as_secs();
            chrono::DateTime::from_timestamp(secs as i64, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "unknown".to_string())
        })
    });
    
    let exif_info = if is_media_file {
        metadata::read_exif(&path_str).ok()
    } else {
        None
    };
    
    let taken = exif_info.as_ref().and_then(|e| e.date_taken.clone());
    
    let gps_location = if is_media_file {
        get_gps_location(file_path).ok().flatten()
    } else {
        None
    };
    
    let format = file_path.extension()
        .map(|e| e.to_string_lossy().to_lowercase());
    
    Some(OrganizeMetadata {
        modified,
        created,
        taken,
        gps_latitude: gps_location.as_ref().map(|l| l.latitude),
        gps_longitude: gps_location.as_ref().map(|l| l.longitude),
        gps_province: gps_location.as_ref().and_then(|l| l.province.clone()),
        gps_city: gps_location.as_ref().and_then(|l| l.city.clone()),
        gps_district: gps_location.as_ref().and_then(|l| l.district.clone()),
        gps_place: gps_location.as_ref().and_then(|l| l.place.clone()),
        camera_make: exif_info.as_ref().and_then(|e| e.camera_make.clone()),
        camera_model: exif_info.as_ref().and_then(|e| e.camera_model.clone()),
        size: Some(metadata.len()),
        format,
    })
}

fn build_target_path(
    source_path: &Path,
    target_base: &Path,
    rule: &OrganizeRule,
) -> Result<PathBuf, String> {
    let metadata = fs::metadata(source_path).map_err(|e| format!("获取文件元数据失败: {}", e))?;
    let file_name = source_path
        .file_name()
        .ok_or("无法获取文件名")?
        .to_string_lossy()
        .to_string();

    let path_str = source_path.to_string_lossy().to_string();
    let is_media_file = metadata::is_image_file(&path_str) || metadata::is_video_file(&path_str);

    let gps_location = if rule.tags.iter().any(|tag| tag == "province" || tag == "city" || tag == "district" || tag == "place") {
        get_gps_location(source_path)?
    } else {
        None
    };

    let exif_info = if rule.tags.iter().any(|tag| tag == "make" || tag == "model") {
        if is_media_file {
            metadata::read_exif(&path_str).ok()
        } else {
            None
        }
    } else {
        None
    };

    let mut path_parts = Vec::new();

    for tag in &rule.tags {
        let part = match tag.as_str() {
            "type" => get_file_type_category(source_path),
            "year" => get_year_from_time(&metadata, &rule.time_source, source_path)?,
            "month" => get_month_from_time(&metadata, &rule.time_source, source_path)?,
            "day" => get_day_from_time(&metadata, &rule.time_source, source_path)?,
            "date" => get_full_date_from_time(&metadata, &rule.time_source, source_path)?,
            "province" => {
                if !is_media_file {
                    continue;
                }
                match &gps_location {
                    Some(loc) => {
                        let province = loc.province.clone().unwrap_or_default();
                        if province.is_empty() {
                            "未知省份".to_string()
                        } else {
                            province
                        }
                    },
                    None => "未知省份".to_string(),
                }
            }
            "city" => {
                if !is_media_file {
                    continue;
                }
                match &gps_location {
                    Some(loc) => {
                        let city = loc.city.clone().unwrap_or_default();
                        if city.is_empty() {
                            "未知城市".to_string()
                        } else {
                            city
                        }
                    },
                    None => "未知城市".to_string(),
                }
            }
            "district" => {
                if !is_media_file {
                    continue;
                }
                match &gps_location {
                    Some(loc) => {
                        let district = loc.district.clone().unwrap_or_default();
                        if district.is_empty() {
                            "未知区县".to_string()
                        } else {
                            district
                        }
                    },
                    None => "未知区县".to_string(),
                }
            }
            "place" => {
                if !is_media_file {
                    continue;
                }
                match &gps_location {
                    Some(loc) => {
                        let place = loc.place.clone().unwrap_or_default();
                        if place.is_empty() {
                            "未知地点".to_string()
                        } else {
                            place
                        }
                    },
                    None => "未知地点".to_string(),
                }
            }
            "make" => {
                if !is_media_file {
                    continue;
                }
                match &exif_info {
                    Some(exif) => {
                        let make = exif.camera_make.clone().unwrap_or_default();
                        if make.is_empty() {
                            "未知品牌".to_string()
                        } else {
                            make
                        }
                    },
                    None => "未知品牌".to_string(),
                }
            }
            "model" => {
                if !is_media_file {
                    continue;
                }
                match &exif_info {
                    Some(exif) => {
                        let model = exif.camera_model.clone().unwrap_or_default();
                        if model.is_empty() {
                            "未知型号".to_string()
                        } else {
                            model
                        }
                    },
                    None => "未知型号".to_string(),
                }
            }
            "ext" => get_extension(source_path),
            "size" => get_size_category(metadata.len()),
            _ => "未知分类".to_string(),
        };
        path_parts.push(part);
    }

    let mut target_path = target_base.to_path_buf();
    for part in path_parts {
        target_path.push(sanitize_path_component(&part));
    }
    target_path.push(file_name);

    Ok(target_path)
}

fn sanitize_path_component(s: &str) -> String {
    let illegal_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    s.chars()
        .map(|c| if illegal_chars.contains(&c) { '_' } else { c })
        .collect()
}

fn get_file_type_category(path: &Path) -> String {
    let ext = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_else(|| "unknown".to_string());

    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif" | "heic" | "heif" | "svg" => "图片",
        "mp4" | "avi" | "mov" | "mkv" | "wmv" | "flv" | "webm" | "mts" | "m2ts" => "视频",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => "音频",
        "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "pdf" | "txt" | "md" => "文档",
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => "压缩包",
        _ => "其他",
    }.to_string()
}

fn get_year_from_time(
    metadata: &fs::Metadata,
    time_source: &str,
    file_path: &Path,
) -> Result<String, String> {
    let time_str = get_time_string(metadata, time_source, file_path)?;
    if time_str.len() >= 4 {
        Ok(time_str[..4].to_string())
    } else {
        Ok("未知年份".to_string())
    }
}

fn get_month_from_time(
    metadata: &fs::Metadata,
    time_source: &str,
    file_path: &Path,
) -> Result<String, String> {
    let time_str = get_time_string(metadata, time_source, file_path)?;
    if time_str.len() >= 7 {
        Ok(time_str[5..7].to_string())
    } else {
        Ok("未知月份".to_string())
    }
}

fn get_day_from_time(
    metadata: &fs::Metadata,
    time_source: &str,
    file_path: &Path,
) -> Result<String, String> {
    let time_str = get_time_string(metadata, time_source, file_path)?;
    if time_str.len() >= 10 {
        Ok(time_str[8..10].to_string())
    } else {
        Ok("未知日期".to_string())
    }
}

fn get_full_date_from_time(
    metadata: &fs::Metadata,
    time_source: &str,
    file_path: &Path,
) -> Result<String, String> {
    let time_str = get_time_string(metadata, time_source, file_path)?;
    if time_str.len() >= 10 {
        Ok(time_str[..10].to_string())
    } else {
        Ok("未知日期".to_string())
    }
}

fn get_time_string(
    metadata: &fs::Metadata,
    time_source: &str,
    file_path: &Path,
) -> Result<String, String> {
    let path_str = file_path.to_string_lossy().to_string();
    let is_media_file = metadata::is_image_file(&path_str) || metadata::is_video_file(&path_str);

    match time_source {
        "modified" => metadata
            .modified()
            .ok()
            .and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                    chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "未知时间".to_string())
                })
            })
            .ok_or("无法获取修改时间".to_string()),
        "created" => metadata
            .created()
            .ok()
            .and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                    chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "未知时间".to_string())
                })
            })
            .ok_or("无法获取创建时间".to_string()),
        "taken" => {
            if !is_media_file {
                return metadata
                    .modified()
                    .ok()
                    .and_then(|t| {
                        t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                            chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                .unwrap_or_else(|| "未知时间".to_string())
                        })
                    })
                    .ok_or("无法获取修改时间".to_string());
            }
            metadata::read_exif(&path_str)
                .ok()
                .and_then(|exif| exif.date_taken)
                .or_else(|| {
                    metadata
                        .modified()
                        .ok()
                        .and_then(|t| {
                            t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                                chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                    .unwrap_or_else(|| "未知时间".to_string())
                            })
                        })
                })
                .ok_or("无法获取拍摄日期".to_string())
        }
        _ => Err("未知时间来源".to_string()),
    }
}

fn get_gps_location(file_path: &Path) -> Result<Option<GpsLocation>, String> {
    let path_str = file_path.to_string_lossy().to_string();
    if !metadata::is_image_file(&path_str) {
        return Ok(None);
    }
    
    match metadata::read_exif(&path_str) {
        Ok(exif) => {
            if let (Some(lat), Some(lng)) = (exif.gps_latitude, exif.gps_longitude) {
                match metadata::reverse_geocode(lat, lng) {
                    Ok(location) => Ok(Some(location)),
                    Err(_) => Ok(None),
                }
            } else {
                Ok(None)
            }
        }
        Err(_) => Ok(None),
    }
}

fn get_province(file_path: &Path) -> Result<String, String> {
    match get_gps_location(file_path)? {
        Some(location) => Ok(location.province.unwrap_or_default()),
        None => Ok(String::new()),
    }
}

fn get_city(file_path: &Path) -> Result<String, String> {
    match get_gps_location(file_path)? {
        Some(location) => Ok(location.city.unwrap_or_default()),
        None => Ok(String::new()),
    }
}

fn get_district(file_path: &Path) -> Result<String, String> {
    match get_gps_location(file_path)? {
        Some(location) => Ok(location.district.unwrap_or_default()),
        None => Ok(String::new()),
    }
}

fn get_place(file_path: &Path) -> Result<String, String> {
    match get_gps_location(file_path)? {
        Some(location) => Ok(location.place.unwrap_or_default()),
        None => Ok(String::new()),
    }
}

fn get_camera_make(file_path: &Path) -> Result<String, String> {
    let path_str = file_path.to_string_lossy().to_string();
    if !metadata::is_image_file(&path_str) {
        return Ok(String::new());
    }
    
    match metadata::read_exif(&path_str) {
        Ok(exif) => {
            Ok(exif.camera_make.unwrap_or_default())
        }
        Err(_) => Ok(String::new()),
    }
}

fn get_camera_model(file_path: &Path) -> Result<String, String> {
    let path_str = file_path.to_string_lossy().to_string();
    if !metadata::is_image_file(&path_str) {
        return Ok(String::new());
    }
    
    match metadata::read_exif(&path_str) {
        Ok(exif) => {
            Ok(exif.camera_model.unwrap_or_default())
        }
        Err(_) => Ok(String::new()),
    }
}

fn get_extension(path: &Path) -> String {
    path.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_else(|| "无扩展名".to_string())
}

fn get_size_category(size: u64) -> String {
    match size {
        0..=1024 => "小于1KB",
        1025..=10240 => "1KB-10KB",
        10241..=102400 => "10KB-100KB",
        102401..=1_048_576 => "100KB-1MB",
        1_048_577..=10_485_760 => "1MB-10MB",
        10_485_761..=104_857_600 => "10MB-100MB",
        104_857_601..=1_073_741_824 => "100MB-1GB",
        _ => "大于1GB",
    }.to_string()
}

pub fn batch_rename(
    paths: &[String],
    target_dir: &str,
    rule: &RenameRule,
) -> Result<Vec<RenameResult>, String> {
    // 确保目标目录存在
    let target_path = Path::new(target_dir);
    if !target_path.exists() {
        fs::create_dir_all(target_path)
            .map_err(|e| format!("创建目标目录失败: {}", e))?;
    }

    let mut results = Vec::new();
    let mut index = rule.start_index;

    for path_str in paths {
        let source_path = Path::new(path_str);
        if !source_path.exists() {
            results.push(RenameResult {
                source_path: path_str.clone(),
                new_name: String::new(),
                target_path: String::new(),
                success: false,
                error: Some("源文件不存在".to_string()),
            });
            continue;
        }

        let metadata = match fs::metadata(source_path) {
            Ok(m) => m,
            Err(e) => {
                results.push(RenameResult {
                    source_path: path_str.clone(),
                    new_name: String::new(),
                    target_path: String::new(),
                    success: false,
                    error: Some(format!("获取元数据失败: {}", e)),
                });
                continue;
            }
        };

        let original_name = source_path
            .file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "file".to_string());
        let extension = source_path
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();

        let new_name_base = build_rename(&original_name, &metadata, rule, index, source_path);
        let new_name = format!("{}{}", new_name_base, extension);

        // 处理目标文件已存在的情况，自动添加序号
        let (final_name, final_target_path) = resolve_duplicate_filename(target_path, &new_name);

        let target_path_str = final_target_path.to_string_lossy().to_string();

        // 复制文件到目标目录（安全模式，不修改原文件）
        match fs::copy(source_path, &final_target_path) {
            Ok(_) => {
                results.push(RenameResult {
                    source_path: path_str.clone(),
                    new_name: final_name,
                    target_path: target_path_str,
                    success: true,
                    error: None,
                });
                index += 1;
            }
            Err(e) => {
                results.push(RenameResult {
                    source_path: path_str.clone(),
                    new_name: final_name,
                    target_path: target_path_str,
                    success: false,
                    error: Some(format!("复制文件失败: {}", e)),
                });
            }
        }
    }

    Ok(results)
}

fn resolve_duplicate_filename(target_dir: &Path, base_name: &str) -> (String, PathBuf) {
    let target_path = target_dir.join(base_name);
    
    if !target_path.exists() {
        return (base_name.to_string(), target_path);
    }
    
    let name_without_ext = Path::new(base_name)
        .file_stem()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| base_name.to_string());
    let extension = Path::new(base_name)
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    
    for i in 1..1000 {
        let new_name = format!("{}_{}{}", name_without_ext, i, extension);
        let new_path = target_dir.join(&new_name);
        if !new_path.exists() {
            return (new_name, new_path);
        }
    }
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let new_name = format!("{}_{}{}", name_without_ext, timestamp, extension);
    (new_name.clone(), target_dir.join(&new_name))
}

fn build_rename(
    original_name: &str,
    metadata: &fs::Metadata,
    rule: &RenameRule,
    index: u32,
    file_path: &Path,
) -> String {
    let mut parts = Vec::new();

    for part in &rule.template_parts {
        let value = match part.part_type.as_str() {
            "tag" => get_tag_value(&part.value, metadata, rule, index, file_path),
            "separator" => part.value.clone(),
            _ => String::new(),
        };
        if !value.is_empty() {
            parts.push(value);
        }
    }

    let result = parts.join("");
    let sanitized = sanitize_filename(&result);

    if sanitized.is_empty() {
        sanitize_filename(original_name)
    } else {
        sanitized
    }
}

fn get_tag_value(
    tag: &str,
    metadata: &fs::Metadata,
    rule: &RenameRule,
    index: u32,
    file_path: &Path,
) -> String {
    let path_str = file_path.to_string_lossy().to_string();
    let is_media_file = metadata::is_image_file(&path_str) || metadata::is_video_file(&path_str);

    match tag {
        "date" => get_date_from_time(metadata, &rule.time_source, file_path).unwrap_or_default(),
        "time" => get_time_of_day(metadata, &rule.time_source, file_path).unwrap_or_default(),
        "year" => get_year_from_time(metadata, &rule.time_source, file_path).unwrap_or_default(),
        "month" => get_month_from_time(metadata, &rule.time_source, file_path).unwrap_or_default(),
        "day" => get_day_from_time(metadata, &rule.time_source, file_path).unwrap_or_default(),
        "type" => get_file_type_category(file_path),
        "name" => file_path
            .file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        "ext" => file_path
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default(),
        "index" => format!("{:03}", index),
        "province" => {
            if is_media_file {
                let province = get_province(file_path).unwrap_or_default();
                if province.is_empty() {
                    "未知省份".to_string()
                } else {
                    province
                }
            } else {
                String::new()
            }
        },
        "city" => {
            if is_media_file {
                let city = get_city(file_path).unwrap_or_default();
                if city.is_empty() {
                    "未知城市".to_string()
                } else {
                    city
                }
            } else {
                String::new()
            }
        },
        "district" => {
            if is_media_file {
                let district = get_district(file_path).unwrap_or_default();
                if district.is_empty() {
                    "未知区县".to_string()
                } else {
                    district
                }
            } else {
                String::new()
            }
        },
        "place" => {
            if is_media_file {
                let place = get_place(file_path).unwrap_or_default();
                if place.is_empty() {
                    "未知地点".to_string()
                } else {
                    place
                }
            } else {
                String::new()
            }
        },
        "make" => {
            if is_media_file {
                let make = get_camera_make(file_path).unwrap_or_default();
                if make.is_empty() {
                    "未知品牌".to_string()
                } else {
                    make
                }
            } else {
                String::new()
            }
        },
        "model" => {
            if is_media_file {
                let model = get_camera_model(file_path).unwrap_or_default();
                if model.is_empty() {
                    "未知型号".to_string()
                } else {
                    model
                }
            } else {
                String::new()
            }
        },
        "size" => get_size_category(metadata.len()),
        "exact_size" => format_file_size(metadata.len()),
        _ => String::new(),
    }
}

fn get_date_from_time(
    metadata: &fs::Metadata,
    time_source: &str,
    file_path: &Path,
) -> Result<String, String> {
    let time_str = get_time_string(metadata, time_source, file_path)?;
    if time_str.len() >= 10 {
        Ok(time_str[..10].replace("-", ""))
    } else {
        Ok("未知日期".to_string())
    }
}

fn get_time_of_day(
    metadata: &fs::Metadata,
    time_source: &str,
    file_path: &Path,
) -> Result<String, String> {
    let time_str = get_time_string(metadata, time_source, file_path)?;
    if time_str.len() >= 19 {
        let time_part = &time_str[11..19];
        Ok(time_part.replace(":", ""))
    } else {
        Ok("未知时间".to_string())
    }
}

pub fn find_duplicates(
    paths: &[String],
    detect_mode: &str,
) -> Result<DuplicateScanResult, String> {
    use std::collections::HashMap;

    let mut file_map: HashMap<String, Vec<DuplicateFile>> = HashMap::new();
    let mut total_files = 0u64;
    let mut size_map: HashMap<String, u64> = HashMap::new();

    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            continue;
        }

        for entry in walkdir::WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !e.file_name().to_string_lossy().starts_with('.'))
        {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !entry.file_type().is_file() {
                continue;
            }

            total_files += 1;
            let file_path = entry.path();
            let file_path_str = file_path.to_string_lossy().to_string();

            let metadata = match fs::metadata(file_path) {
                Ok(m) => m,
                Err(_) => continue,
            };

            let size = metadata.len();
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                        chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or_else(|| "unknown".to_string())
                    })
                })
                .unwrap_or_else(|| "unknown".to_string());

            let name = entry.file_name().to_string_lossy().to_string();

            let key = if detect_mode == "size" {
                format!("size_{}", size)
            } else {
                match compute_md5(&file_path_str) {
                    Ok(md5) => md5,
                    Err(_) => continue,
                }
            };

            size_map.insert(key.clone(), size);
            file_map
                .entry(key)
                .or_insert_with(Vec::new)
                .push(DuplicateFile {
                    path: file_path_str,
                    name,
                    modified,
                    is_original: false,
                });
        }
    }

    let mut duplicate_groups: Vec<DuplicateGroup> = Vec::new();
    let mut total_duplicates = 0u64;
    let mut wasted_space = 0u64;

    for (key, mut files) in file_map {
        if files.len() > 1 {
            files.sort_by(|a, b| a.modified.cmp(&b.modified));
            files[0].is_original = true;

            let size = *size_map.get(&key).unwrap_or(&0);
            total_duplicates += (files.len() - 1) as u64;
            wasted_space += size * (files.len() - 1) as u64;

            duplicate_groups.push(DuplicateGroup {
                md5: key,
                size,
                files,
            });
        }
    }

    duplicate_groups.sort_by(|a, b| b.size.cmp(&a.size));

    Ok(DuplicateScanResult {
        total_files,
        duplicate_groups,
        total_duplicates,
        wasted_space,
    })
}

pub fn clean_duplicates(
    paths: Vec<String>,
    keep_original: bool,
) -> Result<BatchOperationResult, String> {
    let mut results = Vec::new();
    let mut success_count = 0;
    let mut fail_count = 0;

    for path_str in paths {
        let path = Path::new(&path_str);
        if !path.exists() {
            results.push(OperationResult {
                success: false,
                message: None,
                error: Some("文件不存在".to_string()),
            });
            fail_count += 1;
            continue;
        }

        if keep_original {
            match trash::delete(path) {
                Ok(_) => {
                    results.push(OperationResult {
                        success: true,
                        message: Some(format!("已移至回收站: {}", path_str)),
                        error: None,
                    });
                    success_count += 1;
                }
                Err(e) => {
                    results.push(OperationResult {
                        success: false,
                        message: None,
                        error: Some(format!("移至回收站失败: {}", e)),
                    });
                    fail_count += 1;
                }
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => {
                    results.push(OperationResult {
                        success: true,
                        message: Some(format!("已删除: {}", path_str)),
                        error: None,
                    });
                    success_count += 1;
                }
                Err(e) => {
                    results.push(OperationResult {
                        success: false,
                        message: None,
                        error: Some(format!("删除失败: {}", e)),
                    });
                    fail_count += 1;
                }
            }
        }
    }

    Ok(BatchOperationResult {
        total: results.len(),
        success_count,
        fail_count,
        results,
    })
}

pub fn move_files_batch(
    sources: Vec<String>,
    target: &str,
) -> Result<BatchOperationResult, String> {
    let target_path = Path::new(target);
    if !target_path.exists() {
        fs::create_dir_all(target_path).map_err(|e| format!("创建目标目录失败: {}", e))?;
    }

    let mut results = Vec::new();
    let mut success_count = 0;
    let mut fail_count = 0;

    for source_str in sources {
        let source_path = Path::new(&source_str);
        if !source_path.exists() {
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
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let dest_path = target_path.join(&file_name);

        let final_dest = if dest_path.exists() {
            let mut counter = 1;
            let stem = source_path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "file".to_string());
            let ext = source_path
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();

            loop {
                let new_name = format!("{}_{}{}", stem, counter, ext);
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

        match fs::rename(source_path, &final_dest).or_else(|_| {
            fs::copy(source_path, &final_dest)
                .and_then(|_| fs::remove_file(source_path))
                .map_err(|e| format!("移动文件失败: {}", e))
        }) {
            Ok(_) => {
                results.push(OperationResult {
                    success: true,
                    message: Some(format!("已移动: {} -> {}", source_str, dest_str)),
                    error: None,
                });
                success_count += 1;
            }
            Err(e) => {
                results.push(OperationResult {
                    success: false,
                    message: None,
                    error: Some(e),
                });
                fail_count += 1;
            }
        }
    }

    Ok(BatchOperationResult {
        total: results.len(),
        success_count,
        fail_count,
        results,
    })
}

pub fn scan_auxiliary_files(
    paths: &[String],
    cleanup_types: &[String],
) -> Result<Vec<CleanupResult>, String> {
    let mut results: Vec<CleanupResult> = Vec::new();

    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            continue;
        }

        for cleanup_type in cleanup_types {
            let mut files: Vec<CleanupFile> = Vec::new();
            let mut total_size = 0u64;

            for entry in walkdir::WalkDir::new(path)
                .follow_links(false)
                .into_iter()
                .filter_entry(|e| !e.file_name().to_string_lossy().starts_with('.'))
            {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                if !entry.file_type().is_file() {
                    continue;
                }

                let file_name = entry.file_name().to_string_lossy();
                let file_path = entry.path();
                let file_path_str = file_path.to_string_lossy().to_string();

                let should_clean = if cleanup_type == "thumbnail" {
                    file_name == "Thumbs.db" || file_name == "thumbs.db" ||
                    file_name.starts_with(".thumb")
                } else if cleanup_type == "temp" {
                    file_name.ends_with(".tmp") || file_name.ends_with(".temp") ||
                    file_name.ends_with(".bak") || file_name.starts_with("~$") ||
                    file_name.starts_with(".~")
                } else if cleanup_type == "ds_store" {
                    file_name == ".DS_Store" || file_path_str.contains("__MACOSX")
                } else if cleanup_type == "thumbs" {
                    file_name == "Thumbs.db" || file_name == "thumbs.db"
                } else if cleanup_type == "desktop.ini" {
                    file_name == "desktop.ini" || file_name == "Desktop.ini"
                } else {
                    false
                };

                if should_clean {
                    let metadata = match fs::metadata(file_path) {
                        Ok(m) => m,
                        Err(_) => continue,
                    };

                    let size = metadata.len();
                    total_size += size;

                    files.push(CleanupFile {
                        path: file_path_str,
                        name: file_name.to_string(),
                        size,
                        success: false,
                        error: None,
                    });
                }
            }

            if !files.is_empty() {
                results.push(CleanupResult {
                    cleanup_type: cleanup_type.clone(),
                    files,
                    total_size,
                });
            }
        }
    }

    Ok(results)
}

pub fn cleanup_auxiliary_files(
    files: Vec<String>,
) -> Result<BatchOperationResult, String> {
    let mut results = Vec::new();
    let mut success_count = 0;
    let mut fail_count = 0;

    for path_str in files {
        let path = Path::new(&path_str);
        if !path.exists() {
            results.push(OperationResult {
                success: false,
                message: None,
                error: Some("文件不存在".to_string()),
            });
            fail_count += 1;
            continue;
        }

        match trash::delete(path) {
            Ok(_) => {
                results.push(OperationResult {
                    success: true,
                    message: Some(format!("已移至回收站: {}", path_str)),
                    error: None,
                });
                success_count += 1;
            }
            Err(e) => {
                results.push(OperationResult {
                    success: false,
                    message: None,
                    error: Some(format!("移至回收站失败: {}", e)),
                });
                fail_count += 1;
            }
        }
    }

    Ok(BatchOperationResult {
        total: results.len(),
        success_count,
        fail_count,
        results,
    })
}
