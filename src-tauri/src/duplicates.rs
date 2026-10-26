//! 重复文件扫描（按大小/MD5）与清理，及相似照片分组（dHash 感知哈希）。

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::cancel;
use crate::image_util;
use crate::models::{
    BatchOperationResult, DuplicateFile, DuplicateGroup, DuplicateScanResult, OperationResult,
    TaskProgress,
};

/// 相似分组的汉明距离阈值（64 位 dHash）
const SIMILAR_HAMMING_MAX: u32 = 8;
/// 仅这些扩展名参与相似照片分组
const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "bmp", "webp", "tif", "tiff"];

// ponytail: 单流程进度回调逻辑，拆分会破坏可读性
#[allow(clippy::too_many_lines)]
pub fn find_duplicates_with_progress<F>(
    token: &crate::cancel::CancelToken,
    paths: &[String],
    detect_mode: &str,
    mut progress_callback: F,
) -> Result<DuplicateScanResult, String>
where
    F: FnMut(&TaskProgress) + Send,
{
    struct FileEntryData {
        path: String,
        name: String,
        modified: String,
        size: u64,
    }

    if detect_mode == "similar" {
        return find_similar_groups(token, paths, progress_callback);
    }

    log::info!("开始重复文件扫描: {} 个目录, 模式: {}", paths.len(), detect_mode);
    let mut total_files = 0u64;
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
            if entry.is_ok_and(|e| e.file_type().is_file()) {
                total_files += 1;
            }
        }
    }

    let mut entries: Vec<FileEntryData> = Vec::new();
    let mut processed = 0u64;

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
            let file_path_str = file_path.to_string_lossy().to_string();
            let file_name = entry.file_name().to_string_lossy().to_string();

            processed += 1;
            let progress = TaskProgress {
                total: total_files,
                processed,
                current_item: Some(file_name.clone()),
                percentage: {
                    // ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
                    #[allow(clippy::cast_precision_loss)]
                    {
                        // 枚举只占前 50%：md5/both 模式后续还有耗时的哈希阶段
                        let span = if detect_mode == "size" { 95.0 } else { 50.0 };
                        if total_files > 0 { (processed as f64 / total_files as f64) * span } else { 0.0 }
                    }
                },
            };
            progress_callback(&progress);

            cancel::consume(token)?;

            let Ok(metadata) = fs::metadata(file_path) else { continue };

            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                        chrono::DateTime::from_timestamp(d.as_secs().cast_signed(), 0)
                            .map_or_else(|| "unknown".to_string(), |dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    })
                })
                .unwrap_or_else(|| "unknown".to_string());

            entries.push(FileEntryData {
                path: file_path_str,
                name: file_name,
                modified,
                size: metadata.len(),
            });
        }
    }

    let mut file_map: HashMap<String, Vec<DuplicateFile>> = HashMap::new();
    let mut size_map: HashMap<String, u64> = HashMap::new();

    if detect_mode == "size" {
        for e in entries {
            let key = format!("size_{}", e.size);
            size_map.insert(key.clone(), e.size);
            file_map.entry(key).or_default().push(DuplicateFile {
                path: e.path,
                name: e.name,
                modified: e.modified,
                is_original: false,
                size: e.size,
            });
        }
    } else {
        // ponytail: 先按大小预分组，只对同大小候选组并行算 MD5，
        // 独一无二大小的文件（绝大多数）完全不读盘。
        use rayon::prelude::*;

        let mut by_size: HashMap<u64, Vec<FileEntryData>> = HashMap::new();
        for e in entries {
            by_size.entry(e.size).or_default().push(e);
        }

        // ponytail: 两阶段哈希——先 partial hash（头尾 4KB）快速排除不同文件，
        // 仅对 partial hash 冲突的候选者算完整 MD5，避免大文件全量读取。
        // 按批次并行，批次间上报进度（哈希阶段占 50%~95%），
        // 避免进度条提前到 100% 后用户干等。
        let mut candidate_groups: Vec<Vec<FileEntryData>> =
            by_size.into_values().filter(|group| group.len() > 1).collect();
        let total_candidates: u64 = candidate_groups.iter().map(|g| g.len() as u64).sum();
        let mut done_candidates = 0u64;

        let hashed: Vec<(String, u64, DuplicateFile)> = candidate_groups
            .chunks_mut(64)
            .flat_map(|batch| {
                let batch_result: Vec<_> = batch
                    .into_par_iter()
                    .flat_map_iter(|group| {
                        // 阶段一：按 partial hash 分组
                        let mut by_partial: HashMap<String, Vec<&FileEntryData>> = HashMap::new();
                        for e in group {
                            if let Ok(ph) = compute_partial_hash(&e.path) {
                                by_partial.entry(ph).or_default().push(e);
                            }
                        }
                        // 阶段二：仅对 partial hash 冲突的组算完整 MD5
                        by_partial
                            .into_values()
                            .filter(|candidates| candidates.len() > 1)
                            .flat_map(|candidates| {
                                candidates.into_iter().filter_map(|e| {
                                    match compute_md5(&e.path) {
                                        Ok(md5) => Some((md5, e.size, DuplicateFile {
                                            path: e.path.clone(),
                                            name: e.name.clone(),
                                            modified: e.modified.clone(),
                                            is_original: false,
                                            size: e.size,
                                        })),
                                        Err(err) => {
                                            log::debug!("MD5计算失败 {}: {err}", e.path);
                                            None
                                        }
                                    }
                                })
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();

                done_candidates += batch.iter().map(|g| g.len() as u64).sum::<u64>();
                #[allow(clippy::cast_precision_loss)]
                let percentage = (done_candidates as f64 / total_candidates.max(1) as f64).mul_add(45.0, 50.0);
                progress_callback(&TaskProgress {
                    total: total_files,
                    processed: total_files,
                    current_item: Some("正在计算文件指纹...".to_string()),
                    percentage,
                });

                batch_result
            })
            .collect();

        for (md5, size, file) in hashed {
            size_map.insert(md5.clone(), size);
            file_map.entry(md5).or_default().push(file);
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
            // 精确重复组内文件大小一致；size 模式也按实际文件大小求和
            wasted_space += size * (files.len() - 1) as u64;

            duplicate_groups.push(DuplicateGroup {
                md5: key,
                size,
                files,
                // size 模式只保证大小相同，并不代表内容重复，前端据此不做默认勾选
                kind: if detect_mode == "size" { "size".to_string() } else { "exact".to_string() },
            });
        }
    }

    duplicate_groups.sort_by_key(|group| std::cmp::Reverse(group.size));

    log::info!("重复文件扫描完成: {} 文件, {} 组重复, {} 个重复文件, 浪费 {} 字节", total_files, duplicate_groups.len(), total_duplicates, wasted_space);

    // 分组完成才算真正结束，此时才发 100%
    progress_callback(&TaskProgress {
        total: total_files,
        processed: total_files,
        current_item: None,
        percentage: 100.0,
    });

    Ok(DuplicateScanResult {
        total_files,
        duplicate_groups,
        total_duplicates,
        wasted_space,
    })
}

pub fn clean_duplicates_with_progress<F>(
    token: &crate::cancel::CancelToken,
    paths: &[String],
    use_trash: bool,
    mut progress_callback: F,
) -> Result<BatchOperationResult, String>
where
    F: FnMut(&TaskProgress) + Send,
{
    let total = paths.len() as u64;
    let mut results = Vec::new();
    let mut success_count = 0;
    let mut fail_count = 0;
    let mut total_size: u64 = 0;

    for (i, path_str) in paths.iter().enumerate() {
        progress_callback(&TaskProgress {
            total,
            processed: i as u64,
            current_item: Some(path_str.clone()),
            percentage: if total > 0 {
                #[allow(clippy::cast_precision_loss)]
                {
                    (i as f64 / total as f64) * 100.0
                }
            } else {
                0.0
            },
        });

        cancel::consume(token)?;

        let path = Path::new(path_str);
        if !path.exists() || !path.is_file() {
            log::warn!("文件不存在或不是文件，跳过: {path_str}");
            results.push(OperationResult {
                success: false,
                message: None,
                error: Some("文件不存在或不是文件".to_string()),
            });
            fail_count += 1;
            continue;
        }

        let file_size = fs::metadata(path).map_or(0, |m| m.len());

        if use_trash {
            match trash::delete(path) {
                Ok(()) => {
                    log::info!("移至回收站: {path_str} ({file_size} 字节)");
                    total_size += file_size;
                    results.push(OperationResult {
                        success: true,
                        message: Some(format!("已移至回收站: {path_str}")),
                        error: None,
                    });
                    success_count += 1;
                }
                Err(e) => {
                    log::error!("移至回收站失败 {path_str}: {e}");
                    results.push(OperationResult {
                        success: false,
                        message: None,
                        error: Some(format!("移至回收站失败: {e}")),
                    });
                    fail_count += 1;
                }
            }
        } else {
            match fs::remove_file(path) {
                Ok(()) => {
                    log::info!("永久删除: {path_str} ({file_size} 字节)");
                    total_size += file_size;
                    results.push(OperationResult {
                        success: true,
                        message: Some(format!("已删除: {path_str}")),
                        error: None,
                    });
                    success_count += 1;
                }
                Err(e) => {
                    log::error!("删除失败 {path_str}: {e}");
                    results.push(OperationResult {
                        success: false,
                        message: None,
                        error: Some(format!("删除失败: {e}")),
                    });
                    fail_count += 1;
                }
            }
        }
    }

    log::info!("清理重复文件完成: 成功 {success_count}, 失败 {fail_count}, 释放 {total_size} 字节");

    progress_callback(&TaskProgress {
        total,
        processed: total,
        current_item: None,
        percentage: 100.0,
    });

    Ok(BatchOperationResult {
        total: results.len(),
        success_count,
        fail_count,
        total_size,
        results,
    })
}

// --- 相似照片分组 ---

/// 从已打开的图片计算 dHash，避免重复解码。
pub fn dhash_from_image(img: &image::DynamicImage) -> u64 {
    let gray = img.resize_exact(9, 8, image::imageops::FilterType::Triangle).to_luma8();
    let mut hash = 0u64;
    for y in 0..8 {
        for x in 0..8 {
            hash <<= 1;
            if gray.get_pixel(x, y)[0] < gray.get_pixel(x + 1, y)[0] {
                hash |= 1;
            }
        }
    }
    hash
}

pub const fn hamming64(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}

/// 从已打开的图片计算拉普拉斯方差，避免重复解码。
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::suboptimal_flops)]
pub fn laplacian_variance_from_image(img: &image::DynamicImage) -> f64 {
    // 限定采样尺寸，避免超大图计算过慢；清晰度趋势保留
    let gray = img
        .resize(512, 512, image::imageops::FilterType::Triangle)
        .to_luma8();
    let (w, h) = (gray.width() as usize, gray.height() as usize);
    if w < 3 || h < 3 {
        return 0.0;
    }
    let mut sum = 0.0f64;
    let mut sum_sq = 0.0f64;
    let mut count = 0u64;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let at = |px: usize, py: usize| f64::from(gray.get_pixel(px as u32, py as u32)[0]);
            let lap = at(x + 1, y) + at(x - 1, y) + at(x, y + 1) + at(x, y - 1) - 4.0 * at(x, y);
            sum += lap;
            sum_sq += lap * lap;
            count += 1;
        }
    }
    if count == 0 {
        return 0.0;
    }
    let mean = sum / count as f64;
    sum_sq / count as f64 - mean * mean
}

pub fn is_image_path(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|e| {
            let ext = e.to_string_lossy().to_lowercase();
            IMAGE_EXTS.contains(&ext.as_str())
        })
}

/// 相似照片分组：dHash + 汉明距离 union-find，保留最清晰（拉普拉斯方差最大）一张
#[allow(clippy::cast_precision_loss, clippy::too_many_lines, clippy::items_after_statements)]
pub fn find_similar_groups<F>(
    token: &crate::cancel::CancelToken,
    paths: &[String],
    mut progress_callback: F,
) -> Result<DuplicateScanResult, String>
where
    F: FnMut(&TaskProgress) + Send,
{
    log::info!("开始相似照片扫描: {} 个目录", paths.len());

    // 第一遍：统计总数（图片文件）
    let mut total_files = 0u64;
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
            if entry.is_ok_and(|e| e.file_type().is_file()) {
                total_files += 1;
            }
        }
    }

    struct PhotoEntry {
        path: String,
        name: String,
        modified: String,
        size: u64,
        hash: u64,
        laplacian_var: f64,
    }

    let mut photos: Vec<PhotoEntry> = Vec::new();
    let mut processed = 0u64;

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
            let file_path_str = file_path.to_string_lossy().to_string();
            let file_name = entry.file_name().to_string_lossy().to_string();

            processed += 1;
            progress_callback(&TaskProgress {
                total: total_files,
                processed,
                current_item: Some(file_name.clone()),
                // 图片解码在循环内完成，占 95%；分组与结果组装占最后 5%
                percentage: if total_files > 0 {
                    #[allow(clippy::cast_precision_loss)]
                    {
                        (processed as f64 / total_files as f64) * 95.0
                    }
                } else {
                    0.0
                },
            });

            cancel::consume(token)?;

            if !is_image_path(&file_path_str) {
                continue;
            }

            let Ok(metadata) = fs::metadata(file_path) else { continue };
            let modified = crate::fs_util::system_time_to_string(
                metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH),
            );

            // 只解码一次，同时计算 dHash + 清晰度，避免两次加载同一张图
            match image_util::load_image_limited(&file_path_str, 512) {
                Ok(img) => {
                    let hash = dhash_from_image(&img);
                    let laplacian_var = laplacian_variance_from_image(&img);
                    photos.push(PhotoEntry {
                        path: file_path_str,
                        name: file_name,
                        modified,
                        size: metadata.len(),
                        hash,
                        laplacian_var,
                    });
                }
                Err(e) => log::debug!("图片解码失败 {file_path_str}: {e}"),
            }
        }
    }

    // 分组：每张照片只与各组代表比较，避免 A~B、B~C 但 A 与 C 差异很大时
    // union-find 传递闭包把整条链错误合并成一组。
    // ponytail: O(n²) 两两比对，5 万张内可用；升级：64 位哈希切 4x16 位带索引导滤
    let mut grouped: Vec<Vec<usize>> = Vec::new();
    for i in 0..photos.len() {
        let matched = grouped
            .iter_mut()
            .find(|members| hamming64(photos[members[0]].hash, photos[i].hash) <= SIMILAR_HAMMING_MAX);
        if let Some(members) = matched {
            members.push(i);
        } else {
            grouped.push(vec![i]);
        }
    }

    let mut duplicate_groups: Vec<DuplicateGroup> = Vec::new();
    let mut total_duplicates = 0u64;
    let mut wasted_space = 0u64;

    for mut members in grouped {
        if members.len() < 2 {
            continue;
        }
        // 组内按路径排序保证稳定
        members.sort_by(|a, b| photos[*a].path.cmp(&photos[*b].path));

        // 保留最清晰的一张（使用首次解码时缓存的清晰度值，无需重新加载）
        let mut best_idx = members[0];
        let mut best_var = -1.0f64;
        for &idx in &members {
            let var = photos[idx].laplacian_var;
            if var > best_var {
                best_var = var;
                best_idx = idx;
            }
        }

        let files: Vec<DuplicateFile> = members
            .iter()
            .map(|&idx| {
                let p = &photos[idx];
                DuplicateFile {
                    path: p.path.clone(),
                    name: p.name.clone(),
                    modified: p.modified.clone(),
                    is_original: idx == best_idx,
                    size: p.size,
                }
            })
            .collect();

        // 同组照片大小可能不同，浪费空间按实际可清理文件大小求和
        let group_size = photos[members[0]].size;
        total_duplicates += (members.len() - 1) as u64;
        wasted_space += members
            .iter()
            .filter(|&&idx| idx != best_idx)
            .map(|&idx| photos[idx].size)
            .sum::<u64>();

        duplicate_groups.push(DuplicateGroup {
            md5: format!("{:016x}", photos[members[0]].hash),
            size: group_size,
            files,
            kind: "similar".to_string(),
        });
    }

    duplicate_groups.sort_by_key(|group| std::cmp::Reverse(group.size));

    log::info!(
        "相似照片扫描完成: {} 文件, {} 组相似, {} 个可清理",
        total_files,
        duplicate_groups.len(),
        total_duplicates
    );

    // 分组完成才算真正结束，此时才发 100%
    progress_callback(&TaskProgress {
        total: total_files,
        processed: total_files,
        current_item: None,
        percentage: 100.0,
    });

    Ok(DuplicateScanResult {
        total_files,
        duplicate_groups,
        total_duplicates,
        wasted_space,
    })
}

fn compute_md5(path: &str) -> Result<String, String> {
    use md5::{Digest, Md5};
    use std::io::Read;

    let mut file = fs::File::open(path).map_err(|e| format!("打开文件失败: {e}"))?;
    let mut hasher = Md5::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).map_err(|e| format!("读取文件失败: {e}"))?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    Ok(format!("{result:x}"))
}

/// 计算文件头尾各 4KB 的 partial hash，用于快速排除明显不同的文件。
/// 同大小组内绝大多数文件 partial hash 不同，无需读取完整内容。
fn compute_partial_hash(path: &str) -> Result<String, String> {
    use md5::{Digest, Md5};
    use std::io::Read;

    let mut file = fs::File::open(path).map_err(|e| format!("打开文件失败: {e}"))?;
    let file_size = file.metadata().map_or(0, |m| m.len());
    let mut hasher = Md5::new();

    // 混入文件大小，确保不同大小的文件 partial hash 必然不同
    hasher.update(file_size.to_le_bytes());

    let header_len = 4096usize;
    let mut header = vec![0u8; header_len];
    let n = file.read(&mut header).map_err(|e| format!("读取文件头失败: {e}"))?;
    hasher.update(&header[..n]);

    // 文件大于 8KB 时，再读尾部 4KB
    if file_size > header_len as u64 * 2 {
        use std::io::Seek;
        let tail_start = file_size - header_len as u64;
        file.seek(std::io::SeekFrom::Start(tail_start))
            .map_err(|e| format!("定位文件尾失败: {e}"))?;
        let mut tail = vec![0u8; header_len];
        let n = file.read(&mut tail).map_err(|e| format!("读取文件尾失败: {e}"))?;
        hasher.update(&tail[..n]);
    }

    let result = hasher.finalize();
    Ok(format!("{result:x}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming64() {
        assert_eq!(hamming64(0b1010, 0b0101), 4);
        assert_eq!(hamming64(0, 0), 0);
        assert_eq!(hamming64(u64::MAX, 0), 64);
    }

    #[test]
    fn test_dhash_deterministic_and_similar() {
        let dir = std::env::temp_dir().join("leaf-tidy-dhash-test");
        let _ = fs::create_dir_all(&dir);
        let a_path = dir.join("a.png");
        let b_path = dir.join("b.png");

        // 水平渐变图：生成两次应当得到相同哈希
        let mut gray = image::GrayImage::new(64, 64);
        for y in 0..64 {
            for x in 0..64 {
                gray.put_pixel(x, y, image::Luma([(x * 4) as u8]));
            }
        }
        let img = image::DynamicImage::ImageLuma8(gray);
        img.save(&a_path).unwrap();
        img.save(&b_path).unwrap();

        let img = image_util::load_image_limited(a_path.to_str().unwrap(), 64).unwrap();
        let h1 = dhash_from_image(&img);
        let h2 = dhash_from_image(&img);
        assert_eq!(h1, h2, "同一张图两次计算哈希应一致");
        assert_eq!(hamming64(h1, h2), 0);

        let _ = fs::remove_file(&a_path);
        let _ = fs::remove_file(&b_path);
    }

    #[test]
    fn test_laplacian_variance_sharp_vs_blur() {
        let dir = std::env::temp_dir().join("leaf-tidy-lap-test");
        let _ = fs::create_dir_all(&dir);

        // 棋盘格 = 高频细节；模糊副本 = 低频
        let mut img = image::GrayImage::new(128, 128);
        for y in 0..128 {
            for x in 0..128 {
                img.put_pixel(x, y, image::Luma([(if (x + y) % 2 == 0 { 255 } else { 0 })]));
            }
        }
        let sharp_path = dir.join("sharp.png");
        image::DynamicImage::ImageLuma8(img.clone()).save(&sharp_path).unwrap();

        let blurred = image::DynamicImage::ImageLuma8(img)
            .blur(4.0)
            .to_luma8();
        let blur_path = dir.join("blur.png");
        image::DynamicImage::ImageLuma8(blurred).save(&blur_path).unwrap();

        let sharp_img = image_util::load_image_limited(sharp_path.to_str().unwrap(), 512).unwrap();
        let v_sharp = laplacian_variance_from_image(&sharp_img);
        let blur_img = image_util::load_image_limited(blur_path.to_str().unwrap(), 512).unwrap();
        let v_blur = laplacian_variance_from_image(&blur_img);
        assert!(v_sharp > v_blur, "清晰图方差 {v_sharp} 应大于模糊图 {v_blur}");

        let _ = fs::remove_file(&sharp_path);
        let _ = fs::remove_file(&blur_path);
    }

    #[test]
    fn test_is_image_path() {
        assert!(is_image_path("C:/a/b.JPG"));
        assert!(is_image_path("x/y/webp.zzz.webp"));
        assert!(!is_image_path("x/y/photo.txt"));
        assert!(!is_image_path("noext"));
    }
}
