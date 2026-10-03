//! 按模板规则批量重命名（复制到目标目录并生成新文件名）。

use std::fs;
use std::path::Path;

use crate::cancel;
use crate::fs_util::{format_file_size, get_size_category, get_time_string, sanitize_filename};
use crate::metadata;
use crate::models::{RenameResult, RenameRule, TaskProgress};
use crate::organize::{
    get_gps_location, get_day_from_time, get_file_type_category, get_month_from_time,
    get_year_from_time,
};

pub fn batch_rename_with_progress<F>(
    token: &crate::cancel::CancelToken,
    paths: &[String],
    target_dir: &str,
    rule: &RenameRule,
    mut progress_callback: F,
) -> Result<Vec<RenameResult>, String>
where
    F: FnMut(&TaskProgress) + Send,
{
    let target_path = Path::new(target_dir);
    if !target_path.exists() {
        fs::create_dir_all(target_path)
            .map_err(|e| format!("创建目标目录失败: {e}"))?;
    }

    let total = paths.len() as u64;
    let mut results = Vec::new();
    let mut index = rule.start_index;
    let mut success_count = 0u64;
    let mut fail_count = 0u64;

    for (i, path_str) in paths.iter().enumerate() {
        let source_path = Path::new(path_str);
        let file_name = source_path
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().to_string());
        let progress = TaskProgress {
            total,
            processed: i as u64,
            current_item: Some(file_name.clone()),
            percentage: {
                // ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
                #[allow(clippy::cast_precision_loss)]
                {
                    if total > 0 { (i as f64 / total as f64) * 100.0 } else { 0.0 }
                }
            },
        };
        progress_callback(&progress);

        cancel::consume(token)?;

        if !source_path.exists() {
            log::warn!("源文件不存在，跳过: {path_str}");
            results.push(RenameResult {
                source_path: path_str.clone(),
                new_name: String::new(),
                target_path: String::new(),
                success: false,
                error: Some("源文件不存在".to_string()),
            });
            fail_count += 1;
            continue;
        }

        let metadata = match fs::metadata(source_path) {
            Ok(m) => m,
            Err(e) => {
                log::error!("获取元数据失败 {path_str}: {e}");
                results.push(RenameResult {
                    source_path: path_str.clone(),
                    new_name: String::new(),
                    target_path: String::new(),
                    success: false,
                    error: Some(format!("获取元数据失败: {e}")),
                });
                fail_count += 1;
                continue;
            }
        };

        let original_name = source_path
            .file_stem()
            .map_or_else(|| "file".to_string(), |n| n.to_string_lossy().to_string());
        let extension = source_path
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();

        let new_name_base = build_rename(&original_name, &metadata, rule, index, source_path);
        let new_name = format!("{new_name_base}{extension}");

        let (final_name, final_target_path) = crate::fs_util::resolve_duplicate_filename(target_path, &new_name);
        let target_path_str = final_target_path.to_string_lossy().to_string();

        match fs::copy(source_path, &final_target_path) {
            Ok(_) => {
                log::debug!("重命名: {original_name} -> {final_name}");
                results.push(RenameResult {
                    source_path: path_str.clone(),
                    new_name: final_name,
                    target_path: target_path_str,
                    success: true,
                    error: None,
                });
                index += 1;
                success_count += 1;
            }
            Err(e) => {
                log::error!("复制失败 {path_str} -> {target_path_str}: {e}");
                results.push(RenameResult {
                    source_path: path_str.clone(),
                    new_name: final_name,
                    target_path: target_path_str,
                    success: false,
                    error: Some(format!("复制文件失败: {e}")),
                });
                fail_count += 1;
            }
        }
    }

    progress_callback(&TaskProgress {
        total,
        processed: total,
        current_item: None,
        percentage: 100.0,
    });

    log::info!("批量重命名完成: 成功 {success_count}, 失败 {fail_count} -> {target_dir}");

    Ok(results)
}

fn build_rename(
    original_name: &str,
    metadata: &fs::Metadata,
    rule: &RenameRule,
    index: u32,
    file_path: &Path,
) -> String {
    let path_str = file_path.to_string_lossy().to_string();
    let is_media = metadata::is_image_file(&path_str) || metadata::is_video_file(&path_str);

    let gps_location = if is_media { get_gps_location(file_path) } else { None };
    let (camera_make, camera_model) = if is_media {
        let exif = metadata::read_exif(&path_str).ok();
        (
            exif.as_ref().and_then(|e| e.camera_make.clone()),
            exif.as_ref().and_then(|e| e.camera_model.clone()),
        )
    } else {
        (None, None)
    };

    let mut parts = Vec::new();

    for part in &rule.template_parts {
        let value = match part.part_type.as_str() {
            "tag" => get_tag_value(
                &part.value, metadata, rule, index, file_path,
                gps_location.as_ref(),
                camera_make.as_deref(),
                camera_model.as_deref(),
            ),
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

#[allow(clippy::too_many_arguments)]
fn get_tag_value(
    tag: &str,
    metadata: &fs::Metadata,
    rule: &RenameRule,
    index: u32,
    file_path: &Path,
    gps_location: Option<&crate::models::GpsLocation>,
    camera_make: Option<&str>,
    camera_model: Option<&str>,
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
        "index" => format!("{index:03}"),
        "province" => {
            if is_media_file {
                gps_location.and_then(|l| l.province.as_deref()).unwrap_or("未知省份").to_string()
            } else {
                String::new()
            }
        },
        "city" => {
            if is_media_file {
                gps_location.and_then(|l| l.city.as_deref()).unwrap_or("未知城市").to_string()
            } else {
                String::new()
            }
        },
        "district" => {
            if is_media_file {
                gps_location.and_then(|l| l.district.as_deref()).unwrap_or("未知区县").to_string()
            } else {
                String::new()
            }
        },
        "place" => {
            if is_media_file {
                gps_location.and_then(|l| l.place.as_deref()).unwrap_or("未知地点").to_string()
            } else {
                String::new()
            }
        },
        "make" => {
            if is_media_file {
                camera_make.unwrap_or("未知品牌").to_string()
            } else {
                String::new()
            }
        },
        "model" => {
            if is_media_file {
                camera_model.unwrap_or("未知型号").to_string()
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
        Ok(time_str[..10].replace('-', ""))
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
        Ok(time_part.replace(':', ""))
    } else {
        Ok("未知时间".to_string())
    }
}
