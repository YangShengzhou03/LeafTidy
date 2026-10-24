//! 供各业务模块（扫描/整理/重命名/分类）共用的文件系统与命名工具函数。

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::metadata;

pub fn system_time_to_string(time: SystemTime) -> String {
    time.duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|d| {
            chrono::DateTime::from_timestamp(d.as_secs().cast_signed(), 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

// Windows 保留设备名（无论扩展名）
const RESERVED: &[&str] = &[
    "CON", "PRN", "AUX", "NUL",
    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

pub fn sanitize_filename(name: &str) -> String {
    let illegal_chars = ['\\', '/', ':', '?', '"', '<', '>', '|'];
    let mut result: String = name
        .chars()
        .map(|c| if illegal_chars.contains(&c) { '_' } else { c })
        .collect();

    // 防止路径遍历：移除 ".." 序列和开头的 '.'
    result = result.replace("..", "_");
    if result.starts_with('.') {
        result = format!("_{result}");
    }

    // 处理 Windows 保留设备名（无论扩展名）
    let stem = Path::new(&result)
        .file_stem()
        .map(|s| s.to_string_lossy().to_uppercase())
        .unwrap_or_default();
    if RESERVED.contains(&stem.as_str()) {
        result = format!("_{result}");
    }

    result
}

// ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
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

pub fn get_size_category(size: u64) -> String {
    match size {
        0..=1024 => "小于1KB",
        1025..=10240 => "1KB-10KB",
        10_241..=102_400 => "10KB-100KB",
        102_401..=1_048_576 => "100KB-1MB",
        1_048_577..=10_485_760 => "1MB-10MB",
        10_485_761..=104_857_600 => "10MB-100MB",
        104_857_601..=1_073_741_824 => "100MB-1GB",
        _ => "大于1GB",
    }.to_string()
}

pub fn get_time_string(
    metadata: &fs::Metadata,
    time_source: &str,
    file_path: &Path,
) -> Result<String, String> {
    let path_str = file_path.to_string_lossy().to_string();
    let is_media_file = metadata::is_image_file(&path_str) || metadata::is_video_file(&path_str);

    let result = match time_source {
        "modified" => metadata.modified().ok().map(system_time_to_string),
        "created" => metadata.created().ok().map(system_time_to_string),
        "taken" => {
            if is_media_file {
                metadata::read_exif(&path_str)
                    .ok()
                    .and_then(|exif| exif.date_taken)
                    .or_else(|| metadata.modified().ok().map(system_time_to_string))
            } else {
                metadata.modified().ok().map(system_time_to_string)
            }
        }
        _ => return Err("未知时间来源".to_string()),
    };
    result.ok_or_else(|| format!("无法获取{}时间", match time_source {
        "modified" => "修改",
        "created" => "创建",
        "taken" => "拍摄",
        _ => "未知",
    }))
}

/// 原子写文件：先写同目录临时文件再 rename，避免写一半崩溃/断电损坏目标文件。
pub fn atomic_write(path: &Path, data: &[u8]) -> Result<(), String> {
    let tmp = path.with_extension(format!(
        "{}tmp",
        path.extension().map_or_else(String::new, |e| format!("{}.", e.to_string_lossy()))
    ));
    std::fs::write(&tmp, data).map_err(|e| format!("写入临时文件失败: {e}"))?;
    std::fs::rename(&tmp, path).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        format!("替换文件失败: {e}")
    })
}

/// 真正的移动：同盘直接 rename，跨盘失败时退化为复制后删除源文件
pub fn move_file(source: &Path, target: &Path) -> std::io::Result<()> {
    std::fs::rename(source, target).or_else(|_| {
        std::fs::copy(source, target)
            .map(|_| ())
            .and_then(|()| std::fs::remove_file(source))
    })
}

/// 处理目标目录文件名冲突（`file.jpg` -> `file_1.jpg`），供各业务模块共用；返回 (最终文件名, 完整路径)
pub fn resolve_duplicate_filename(target_dir: &Path, base_name: &str) -> (String, PathBuf) {
    let target_path = target_dir.join(base_name);

    if !target_path.exists() {
        return (base_name.to_string(), target_path);
    }

    let name_without_ext = Path::new(base_name)
        .file_stem()
        .map_or_else(|| base_name.to_string(), |n| n.to_string_lossy().to_string());
    let extension = Path::new(base_name)
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    // 最多尝试 999 次，避免无限循环
    for i in 1..1000 {
        let new_name = format!("{name_without_ext}_{i}{extension}");
        let new_path = target_dir.join(&new_name);
        if !new_path.exists() {
            return (new_name, new_path);
        }
    }

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let new_name = format!("{name_without_ext}_{timestamp}{extension}");
    (new_name.clone(), target_dir.join(&new_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_reserved_names() {
        // 保留名（无论大小写、无论扩展名）都应被加前缀
        assert_eq!(sanitize_filename("CON"), "_CON");
        assert_eq!(sanitize_filename("con"), "_con");
        assert_eq!(sanitize_filename("CON.jpg"), "_CON.jpg");
        assert_eq!(sanitize_filename("con.jpg"), "_con.jpg");
        assert_eq!(sanitize_filename("NUL.txt"), "_NUL.txt");
        assert_eq!(sanitize_filename("COM1"), "_COM1");
        assert_eq!(sanitize_filename("LPT9.log"), "_LPT9.log");

        // 非保留名不应被修改
        assert_eq!(sanitize_filename("file.CON"), "file.CON");
        assert_eq!(sanitize_filename("myfile.txt"), "myfile.txt");
        assert_eq!(sanitize_filename("CONTACT.jpg"), "CONTACT.jpg");
        assert_eq!(sanitize_filename("NULLER"), "NULLER");
    }
}
