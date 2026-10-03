//! 低质图片检测（模糊/小尺寸/截图/闭眼）与删除。

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use crate::duplicates::{self, is_image_path};
use crate::fs_util;
use crate::models::{BatchOperationResult, LowQualityImage, LowQualityScanResult, OperationResult, TaskProgress};

/// 缩略图最大边长（px）
const THUMB_MAX_PX: u32 = 96;

/// 常见截图宽高比（宽/高）
const SCREENSHOT_RATIOS: &[(u32, u32)] = &[
    (16, 9),
    (16, 10),
    (21, 9),
    (4, 3),
    (5, 4),    // 桌面显示器（如 1280x1024）
    (3, 2),
    (1, 1),
    (9, 16),   // 竖屏
    (9, 19),   // 手机
    (9, 21),   // 超宽屏手机
];
/// 截图比例容差
const RATIO_TOLERANCE: f64 = 0.05;

/// 检查宽高比是否匹配常见截图比例
fn matches_screenshot_ratio(w: u32, h: u32) -> bool {
    if h == 0 || w == 0 {
        return false;
    }
    let actual = f64::from(w) / f64::from(h);
    for (rw, rh) in SCREENSHOT_RATIOS {
        let expected = f64::from(*rw) / f64::from(*rh);
        if (actual - expected).abs() < RATIO_TOLERANCE {
            return true;
        }
    }
    false
}

/// 扫描指定目录中的低质图片，支持多种检测类型。
///
/// 检测类型：
/// - "blur": 模糊检测（拉普拉斯方差 < threshold）
/// - "small": 图片尺寸过低（宽或高 < `min_dimension`）
/// - "screenshot": 截图（匹配常见屏幕比例）
/// - "`closed_eye`": 闭眼检测（需 YuNet，暂未实现）
#[allow(clippy::too_many_lines)]
pub fn scan_low_quality_images_with_progress<F, G>(
    token: &crate::cancel::CancelToken,
    paths: &[String],
    detect_types: &[String],
    variance_threshold: f64,
    min_dimension: u32,
    mut progress_callback: F,
    mut result_callback: G,
) -> Result<LowQualityScanResult, String>
where
    F: FnMut(&TaskProgress) + Send,
    G: FnMut(&LowQualityImage) + Send,
{
    let type_set: HashSet<&str> = detect_types.iter().map(String::as_str).collect();
    let detect_blur = type_set.contains("blur");
    let detect_small = type_set.contains("small");
    let detect_screenshot = type_set.contains("screenshot");

    log::info!(
        "开始低质图片扫描: {} 个目录, 检测类型: {:?}, 模糊阈值: {}, 最小尺寸: {}",
        paths.len(),
        detect_types,
        variance_threshold,
        min_dimension
    );

    // 第一遍：统计图片文件总数（用于进度百分比）
    let mut total_images = 0u64;
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
            if let Ok(e) = entry.as_ref() {
                if e.file_type().is_file() && is_image_path(&e.path().to_string_lossy()) {
                    total_images += 1;
                }
            }
        }
    }

    let mut processed = 0u64;
    let mut low_quality_images = Vec::new();

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
            let Ok(entry) = entry else { continue };
            if !entry.file_type().is_file() {
                continue;
            }
            let file_path = entry.path();
            if !is_image_path(&file_path.to_string_lossy()) {
                continue;
            }

            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path_str = file_path.to_string_lossy().to_string();
            processed += 1;

            let progress = TaskProgress {
                total: total_images,
                processed,
                current_item: Some(file_name.clone()),
                percentage: {
                    #[allow(clippy::cast_precision_loss)]
                    {
                        if total_images > 0 {
                            (processed as f64 / total_images as f64) * 100.0
                        } else {
                            0.0
                        }
                    }
                },
            };
            progress_callback(&progress);

            crate::cancel::consume(token)?;

            // 仅读取文件头获取尺寸（不解码像素），用于小尺寸/截图检测
            let Ok((w, h)) = image::image_dimensions(file_path) else {
                continue;
            };
            let Ok(metadata) = fs::metadata(file_path) else {
                continue;
            };
            let size = metadata.len();

            // 1) 小尺寸检测
            let mut detected_reason: Option<String> = if detect_small && (w < min_dimension || h < min_dimension) {
                Some("small".to_string())
            } else {
                None
            };

            // 2) 截图检测
            if detect_screenshot && detected_reason.is_none() && matches_screenshot_ratio(w, h) {
                // 截图通常文件大小适中（100KB-20MB），排除 RAW 和极小缩略图
                if size > 100_000 && size < 20_000_000 {
                    detected_reason = Some("screenshot".to_string());
                }
            }

            // 3) 模糊检测（拉普拉斯方差内部缩放到 512×512）
            let mut blur_variance = 0.0;
            let mut opened_img: Option<image::DynamicImage> = None;
            if detect_blur && detected_reason.is_none() {
                if let Ok(img) = fs_util::load_image_limited(&file_path_str, 512) {
                    match duplicates::laplacian_variance_from_image(&img) {
                        variance if variance < variance_threshold => {
                            detected_reason = Some("blur".to_string());
                            blur_variance = variance;
                        }
                        variance => blur_variance = variance,
                    }
                    opened_img = Some(img);
                }
            }

            // 4) 闭眼检测（ponytail: 预留，需 YuNet + 眼部关键点）

            if let Some(reason) = detected_reason {
                // 生成缩略图：复用已打开的图片，或重新加载（96px 缩略图）
                let thumb = opened_img.map_or_else(
                    || {
                        fs_util::load_image_limited(&file_path_str, 96)
                            .ok()
                            .and_then(|img| fs_util::make_thumb_jpeg(img, THUMB_MAX_PX))
                    },
                    |img| fs_util::make_thumb_jpeg(img, THUMB_MAX_PX),
                );
                let detected = LowQualityImage {
                    path: file_path_str,
                    name: file_name,
                    size,
                    variance: if reason == "blur" { blur_variance } else { 0.0 },
                    reason: Some(reason),
                    thumb,
                };
                result_callback(&detected);
                low_quality_images.push(detected);
            }
        }
    }

    log::info!(
        "低质图片扫描完成: 扫描 {total_images} 张, 发现 {} 张低质图片",
        low_quality_images.len()
    );

    Ok(LowQualityScanResult {
        images: low_quality_images,
        total_scanned: total_images,
    })
}

/// 删除低质图片（移至回收站），返回批量操作结果。
#[allow(clippy::cast_possible_truncation)]
pub fn delete_low_quality_images_with_progress<F>(
    token: &crate::cancel::CancelToken,
    files: &[String],
    mut progress_callback: F,
) -> Result<BatchOperationResult, String>
where
    F: FnMut(&TaskProgress) + Send,
{
    let total = files.len() as u64;
    let mut results = Vec::new();
    let mut success_count = 0u64;
    let mut fail_count = 0u64;
    let mut total_size = 0u64;

    for (i, path_str) in files.iter().enumerate() {
        let progress = TaskProgress {
            total,
            processed: i as u64,
            current_item: Some(path_str.clone()),
            percentage: {
                #[allow(clippy::cast_precision_loss)]
                {
                    if total > 0 {
                        (i as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    }
                }
            },
        };
        progress_callback(&progress);

        crate::cancel::consume(token)?;

        let path = Path::new(path_str);
        if !path.exists() {
            results.push(OperationResult {
                success: false,
                message: None,
                error: Some("文件不存在".to_string()),
            });
            fail_count += 1;
            continue;
        }

        // 获取文件大小用于日志
        if let Ok(metadata) = fs::metadata(path) {
            total_size += metadata.len();
        }

        match trash::delete(path) {
            Ok(()) => {
                log::debug!("删除低质图片: {path_str}");
                results.push(OperationResult {
                    success: true,
                    message: Some(format!("已移至回收站: {path_str}")),
                    error: None,
                });
                success_count += 1;
            }
            Err(e) => {
                log::error!("删除低质图片失败 {path_str}: {e}");
                results.push(OperationResult {
                    success: false,
                    message: None,
                    error: Some(format!("移至回收站失败: {e}")),
                });
                fail_count += 1;
            }
        }
    }

    log::info!("删除低质图片完成: 成功 {success_count}, 失败 {fail_count}");

    Ok(BatchOperationResult {
        total: results.len(),
        success_count: success_count as usize,
        fail_count: fail_count as usize,
        total_size,
        results,
    })
}

#[cfg(test)]
mod tests {
    /// 截图比例匹配
    #[test]
    fn screenshot_ratio_matching() {
        assert!(super::matches_screenshot_ratio(1920, 1080));
        assert!(super::matches_screenshot_ratio(2560, 1440));
        assert!(super::matches_screenshot_ratio(1280, 1024)); // ~5:4
        assert!(super::matches_screenshot_ratio(100, 100));   // 1:1
        assert!(!super::matches_screenshot_ratio(5000, 100));
    }
}
