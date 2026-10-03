//! 按规则（类型/时间/GPS/相机等标签）整理文件。

use std::fs;
use std::path::{Path, PathBuf};

use crate::cancel;
use crate::fs_util::{get_size_category, get_time_string, sanitize_filename, system_time_to_string};
use crate::metadata;
use crate::models::{ExifInfo, GpsLocation, OrganizeMetadata, OrganizeProgress, OrganizeResult, OrganizeRule};

/// `collect_file_metadata` 返回值，包含解析好的元数据和原始 EXIF/GPS，
/// 供 `build_target_path` 复用，避免重复读取文件。
struct FileMetadata {
    metadata: fs::Metadata,
    exif_info: Option<ExifInfo>,
    gps_location: Option<GpsLocation>,
    organize: Option<OrganizeMetadata>,
}

// ponytail: 单流程进度回调逻辑，拆分会破坏可读性
#[allow(clippy::too_many_lines)]
pub fn organize_files_with_progress<F>(
    token: &crate::cancel::CancelToken,
    source_dirs: &[String],
    target_dir: &str,
    rule: &OrganizeRule,
    mut progress_callback: F,
) -> Result<Vec<OrganizeResult>, String>
where
    F: FnMut(&OrganizeProgress) + Send,
{
    log::info!("开始文件整理: {} 个源目录 -> {}, 规则标签: {:?}", source_dirs.len(), target_dir, rule.tags);
    let target_path = Path::new(target_dir);
    if !target_path.exists() {
        fs::create_dir_all(target_path).map_err(|e| format!("创建目标目录失败: {e}"))?;
    }

    // 单次目录遍历，收集所有文件路径，避免重复 walkdir
    let mut file_entries: Vec<PathBuf> = Vec::new();
    for source_dir in source_dirs {
        let source_path = Path::new(source_dir);
        if !source_path.exists() || !source_path.is_dir() {
            log::warn!("源目录不存在或不是目录: {source_dir}");
            continue;
        }
        log::info!("读取源目录: {source_dir}");
        let mut dir_file_count = 0u64;
        for e in walkdir::WalkDir::new(source_path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !e.file_name().to_string_lossy().starts_with('.'))
            .flatten()
        {
            if e.file_type().is_file() {
                file_entries.push(e.path().to_path_buf());
                dir_file_count += 1;
            }
        }
        log::info!("源目录 {source_dir}: 读取 {dir_file_count} 个文件");
    }

    let total_files = file_entries.len() as u64;
    log::info!("文件整理共读取 {total_files} 个文件");
    let mut results = Vec::new();
    let mut processed = 0u64;
    let mut success_count = 0u64;
    let mut fail_count = 0u64;

    for file_path in &file_entries {
        let file_path_str = file_path.to_string_lossy().to_string();
        let file_name = file_path
            .file_name()
            .map_or_else(|| file_path_str.clone(), |n| n.to_string_lossy().to_string());

            let progress = OrganizeProgress {
                total: total_files,
                processed,
                success_count,
                fail_count,
                current_file: Some(file_name.clone()),
                percentage: {
                    // ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
                    #[allow(clippy::cast_precision_loss)]
                    {
                        if total_files > 0 {
                            (processed as f64 / total_files as f64) * 100.0
                        } else {
                            0.0
                        }
                    }
                },
            };
            progress_callback(&progress);

            cancel::consume(token)?;

            let start_time = std::time::Instant::now();
            let Ok(meta) = collect_file_metadata(file_path) else {
                log::error!("获取文件元数据失败 {file_path_str}");
                let elapsed_ms = elapsed_millis(start_time);
                results.push(OrganizeResult {
                    source_path: file_path_str.clone(),
                    target_path: String::new(),
                    success: false,
                    error: Some("获取文件元数据失败".to_string()),
                    metadata: None,
                    process_time_ms: Some(elapsed_ms),
                });
                processed += 1;
                fail_count += 1;
                continue;
            };

            let target_subpath = match build_target_path(file_path, target_path, rule, &meta) {
                Ok(p) => p,
                Err(e) => {
                    log::error!("构建目标路径失败 {file_path_str}: {e}");
                    let elapsed_ms = elapsed_millis(start_time);
                    results.push(OrganizeResult {
                        source_path: file_path_str,
                        target_path: String::new(),
                        success: false,
                        error: Some(e),
                        metadata: meta.organize,
                        process_time_ms: Some(elapsed_ms),
                    });
                    processed += 1;
                    fail_count += 1;
                    continue;
                }
            };

            if let Some(parent) = target_subpath.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    log::error!("创建目标子目录失败 {}: {e}", parent.display());
                    let elapsed_ms = elapsed_millis(start_time);
                    results.push(OrganizeResult {
                        source_path: file_path_str,
                        target_path: target_subpath.to_string_lossy().to_string(),
                        success: false,
                        error: Some(format!("创建目录失败: {e}")),
                        metadata: meta.organize,
                        process_time_ms: Some(elapsed_ms),
                    });
                    processed += 1;
                    fail_count += 1;
                    continue;
                }
            }

            let target_path_str = target_subpath.to_string_lossy().to_string();
            let op_result = fs::copy(file_path, &target_subpath)
                .map(|_| ())
                .map_err(|e| format!("复制文件失败: {e}"));

            let elapsed_ms = elapsed_millis(start_time);

            match op_result {
                Ok(()) => {
                    log::debug!("复制: {file_path_str} -> {target_path_str}");
                    results.push(OrganizeResult {
                        source_path: file_path_str,
                        target_path: target_path_str,
                        success: true,
                        error: None,
                        metadata: meta.organize,
                        process_time_ms: Some(elapsed_ms),
                    });
                    success_count += 1;
                }
                Err(e) => {
                    log::error!("复制失败 {file_path_str} -> {target_path_str}: {e}");
                    results.push(OrganizeResult {
                        source_path: file_path_str,
                        target_path: target_path_str,
                        success: false,
                        error: Some(e),
                        metadata: meta.organize,
                        process_time_ms: Some(elapsed_ms),
                    });
                    fail_count += 1;
                }
            }

            processed += 1;
        }

    log::info!("文件整理完成: 成功 {success_count}, 失败 {fail_count}, 总文件 {total_files}");


    let final_progress = OrganizeProgress {
        total: total_files,
        processed,
        success_count,
        fail_count,
        current_file: None,
        percentage: 100.0,
    };
    progress_callback(&final_progress);

    Ok(results)
}

// ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
#[allow(clippy::cast_possible_truncation)]
fn elapsed_millis(start: std::time::Instant) -> u64 {
    start.elapsed().as_millis() as u64
}

fn collect_file_metadata(file_path: &Path) -> Result<FileMetadata, String> {
    let Ok(metadata) = fs::metadata(file_path) else {
        return Err("获取文件元数据失败".to_string());
    };
    let path_str = file_path.to_string_lossy().to_string();
    let is_media_file = metadata::is_image_file(&path_str) || metadata::is_video_file(&path_str);

    let modified = metadata.modified().ok().map(system_time_to_string);
    let created = metadata.created().ok().map(system_time_to_string);

    let exif_info = if is_media_file {
        metadata::read_exif(&path_str).ok()
    } else {
        None
    };

    let taken = exif_info.as_ref().and_then(|e| e.date_taken.clone());

    let gps_location = if is_media_file {
        get_gps_location_from_exif(exif_info.as_ref())
    } else {
        None
    };

    let format = file_path.extension().map(|e| e.to_string_lossy().to_lowercase());

    let organize = Some(OrganizeMetadata {
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
    });

    Ok(FileMetadata {
        metadata,
        exif_info,
        gps_location,
        organize,
    })
}

// ponytail: 按标签逐项映射目录层级，拆分会破坏标签语义的连贯性
#[allow(clippy::too_many_lines)]
fn build_target_path(
    source_path: &Path,
    target_base: &Path,
    rule: &OrganizeRule,
    meta: &FileMetadata,
) -> Result<PathBuf, String> {
    let metadata = &meta.metadata;
    let file_name = source_path
        .file_name()
        .ok_or("无法获取文件名")?
        .to_string_lossy()
        .to_string();

    let path_str = source_path.to_string_lossy().to_string();
    let is_media_file = metadata::is_image_file(&path_str) || metadata::is_video_file(&path_str);

    let gps_location = &meta.gps_location;
    let exif_info = &meta.exif_info;

    let mut path_parts = Vec::new();

    for tag in &rule.tags {
        let part = match tag.as_str() {
            "type" => get_file_type_category(source_path),
            "year" => get_year_from_time(metadata, &rule.time_source, source_path)?,
            "month" => get_month_from_time(metadata, &rule.time_source, source_path)?,
            "day" => get_day_from_time(metadata, &rule.time_source, source_path)?,
            "date" => get_full_date_from_time(metadata, &rule.time_source, source_path)?,
            "province" => {
                if !is_media_file {
                    continue;
                }
                gps_location.as_ref().map_or_else(|| "未知省份".to_string(), |loc| {
                    let province = loc.province.clone().unwrap_or_default();
                    if province.is_empty() {
                        "未知省份".to_string()
                    } else {
                        province
                    }
                })
            }
            "city" => {
                if !is_media_file {
                    continue;
                }
                gps_location.as_ref().map_or_else(|| "未知城市".to_string(), |loc| {
                    let city = loc.city.clone().unwrap_or_default();
                    if city.is_empty() {
                        "未知城市".to_string()
                    } else {
                        city
                    }
                })
            }
            "district" => {
                if !is_media_file {
                    continue;
                }
                gps_location.as_ref().map_or_else(|| "未知区县".to_string(), |loc| {
                    let district = loc.district.clone().unwrap_or_default();
                    if district.is_empty() {
                        "未知区县".to_string()
                    } else {
                        district
                    }
                })
            }
            "place" => {
                if !is_media_file {
                    continue;
                }
                gps_location.as_ref().map_or_else(|| "未知地点".to_string(), |loc| {
                    let place = loc.place.clone().unwrap_or_default();
                    if place.is_empty() {
                        "未知地点".to_string()
                    } else {
                        place
                    }
                })
            }
            "make" => {
                if !is_media_file {
                    continue;
                }
                exif_info.as_ref().map_or_else(|| "未知品牌".to_string(), |exif| {
                    let make = exif.camera_make.clone().unwrap_or_default();
                    if make.is_empty() {
                        "未知品牌".to_string()
                    } else {
                        make
                    }
                })
            }
            "model" => {
                if !is_media_file {
                    continue;
                }
                exif_info.as_ref().map_or_else(|| "未知型号".to_string(), |exif| {
                    let model = exif.camera_model.clone().unwrap_or_default();
                    if model.is_empty() {
                        "未知型号".to_string()
                    } else {
                        model
                    }
                })
            }
            "ext" => get_extension(source_path),
            "size" => get_size_category(metadata.len()),
            _ => "未知分类".to_string(),
        };
        path_parts.push(part);
    }

    let mut target_path = target_base.to_path_buf();
    for part in path_parts {
        target_path.push(sanitize_filename(&part));
    }
    target_path.push(file_name);

    Ok(target_path)
}

pub fn get_file_type_category(path: &Path) -> String {
    let ext = path
        .extension()
        .map_or_else(|| "unknown".to_string(), |e| e.to_string_lossy().to_lowercase());

    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif" | "heic" | "heif" | "svg" => "图片",
        "mp4" | "avi" | "mov" | "mkv" | "wmv" | "flv" | "webm" | "mts" | "m2ts" => "视频",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => "音频",
        "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "pdf" | "txt" | "md" => "文档",
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => "压缩包",
        _ => "其他",
    }.to_string()
}

pub fn get_year_from_time(
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

pub fn get_month_from_time(
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

pub fn get_day_from_time(
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

pub fn get_gps_location(file_path: &Path) -> Option<GpsLocation> {
    let path_str = file_path.to_string_lossy().to_string();
    if !metadata::is_image_file(&path_str) {
        return None;
    }
    let exif = metadata::read_exif(&path_str).ok()?;
    let (lat, lng) = exif.gps_latitude.zip(exif.gps_longitude)?;
    crate::geocode::reverse_geocode(lat, lng).ok()
}

fn get_gps_location_from_exif(exif: Option<&ExifInfo>) -> Option<GpsLocation> {
    let exif = exif?;
    let (lat, lng) = exif.gps_latitude.zip(exif.gps_longitude)?;
    crate::geocode::reverse_geocode(lat, lng).ok()
}

fn get_extension(path: &Path) -> String {
    path.extension()
        .map_or_else(|| "无扩展名".to_string(), |e| e.to_string_lossy().to_lowercase())
}
