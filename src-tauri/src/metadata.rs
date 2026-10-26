use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use exif::experimental::Writer;
use exif::{In, Rational, Tag, Value};
use rayon::prelude::*;

use crate::cancel;
use crate::date_parse::parse_date_from_filename;
use crate::fs_util::atomic_write;
use crate::models::{ExifInfo, ImageProcessResult, TaskProgress};

pub fn read_exif(path: &str) -> Result<ExifInfo, String> {
    log::trace!("读取EXIF: {path}");
    let file = std::fs::File::open(path).map_err(|e| format!("打开文件失败: {e}"))?;
    let mut bufreader = std::io::BufReader::new(file);

    let exif_reader = exif::Reader::new();
    let exif = exif_reader
        .read_from_container(&mut bufreader)
        .map_err(|e| format!("读取EXIF失败: {e}"))?;

    let get_str = |tag: exif::Tag| -> Option<String> {
        exif.get_field(tag, exif::In::PRIMARY)
            .map(|f| {
                let s = f.value.display_as(f.tag).to_string();
                s.trim_matches('"').trim().to_string()
            })
    };

    let gps_lat = parse_gps_coordinate(&exif, exif::Tag::GPSLatitude, exif::Tag::GPSLatitudeRef);
    let gps_lng = parse_gps_coordinate(&exif, exif::Tag::GPSLongitude, exif::Tag::GPSLongitudeRef);

    Ok(ExifInfo {
        camera_make: get_str(exif::Tag::Make),
        camera_model: get_str(exif::Tag::Model),
        date_taken: get_str(exif::Tag::DateTimeOriginal).or_else(|| get_str(exif::Tag::DateTime)),
        gps_latitude: gps_lat,
        gps_longitude: gps_lng,
    })
}

fn parse_gps_coordinate(
    exif: &exif::Exif,
    coord_tag: exif::Tag,
    ref_tag: exif::Tag,
) -> Option<f64> {
    let coord = exif.get_field(coord_tag, exif::In::PRIMARY)?;
    let coord_ref = exif
        .get_field(ref_tag, exif::In::PRIMARY)
        .map(|f| f.value.display_as(f.tag).to_string());

    let value = match &coord.value {
        exif::Value::Rational(ref ratios) if !ratios.is_empty() => {
            let r = ratios[0];
            let degrees = f64::from(r.num) / f64::from(r.denom);
            if ratios.len() < 2 {
                degrees
            } else {
                let r2 = ratios[1];
                let minutes = f64::from(r2.num) / f64::from(r2.denom);
                if ratios.len() < 3 {
                    degrees + minutes / 60.0
                } else {
                    let r3 = ratios[2];
                    let seconds = f64::from(r3.num) / f64::from(r3.denom);
                    degrees + minutes / 60.0 + seconds / 3600.0
                }
            }
        }
        _ => return None,
    };

    match coord_ref.as_deref() {
        Some("S" | "W") => Some(-value),
        _ => Some(value),
    }
}

pub fn is_image_file(path: &str) -> bool {
    let path = Path::new(path);
    path.extension().is_some_and(|ext| {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(
            ext.as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif" | "heic" | "heif"
        )
    })
}

pub fn is_video_file(path: &str) -> bool {
    let path = Path::new(path);
    path.extension().is_some_and(|ext| {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(
            ext.as_str(),
            "mp4" | "avi" | "mov" | "mkv" | "wmv" | "flv" | "webm" | "mts" | "m2ts"
        )
    })
}

// --- 图像元数据编辑 ---

/// 批量处理并输出到目标目录：顺序预分配目标文件名（批内同名追加序号，绝不覆盖），
/// 再用 rayon 并行逐文件处理；输出目录与源文件同目录时拒绝，避免改写原文件。
/// ponytail: 当前仅支持 JPG（EXIF 重写依赖 JPEG APP1 段结构），
/// 其他格式报错跳过；后续如需 PNG/HEIC 再引入对应分段读写。
fn process_image_batch<F>(
    token: &cancel::CancelToken,
    paths: &[String],
    target_dir: &str,
    process: F,
    progress_callback: impl Fn(&TaskProgress) + Sync,
) -> Result<Vec<ImageProcessResult>, String>
where
    F: Fn(&Path, &Path) -> Result<(), String> + Sync,
{
    let target = Path::new(target_dir);
    if !target.exists() {
        std::fs::create_dir_all(target).map_err(|e| format!("创建目标目录失败: {e}"))?;
    }

    let total = paths.len() as u64;

    // 预分配目标文件名：兼顾磁盘已有文件与批内同名（Windows 不区分大小写）
    let mut claimed: HashSet<String> = HashSet::with_capacity(paths.len());
    let mut targets: Vec<Result<PathBuf, String>> = Vec::with_capacity(paths.len());
    for path_str in paths {
        let source = Path::new(path_str);
        if source.parent().is_some_and(|p| p == target) {
            targets.push(Err("输出目录与源文件所在目录相同，为避免覆盖原文件已跳过".to_string()));
            continue;
        }
        let file_name = source
            .file_name()
            .map_or_else(|| "unknown".to_string(), |n| n.to_string_lossy().to_string());
        let is_taken = |n: &str| {
            #[cfg(windows)]
            let key = n.to_lowercase();
            #[cfg(not(windows))]
            let key = n.as_str();
            claimed.contains(&key) || target.join(n).exists()
        };
        let mut candidate = file_name.clone();
        if is_taken(&candidate) {
            let stem = Path::new(&file_name)
                .file_stem()
                .map_or_else(|| file_name.clone(), |n| n.to_string_lossy().to_string());
            let ext = Path::new(&file_name)
                .extension()
                .map_or_else(String::new, |e| format!(".{}", e.to_string_lossy()));
            let mut i = 1;
            loop {
                candidate = format!("{stem}_{i}{ext}");
                if !is_taken(&candidate) {
                    break;
                }
                i += 1;
            }
        }
        // Windows 不区分大小写，HashSet 存小写；其他平台存原始大小写
        #[cfg(windows)]
        claimed.insert(candidate.to_lowercase());
        #[cfg(not(windows))]
        claimed.insert(candidate.clone());
        targets.push(Ok(target.join(&candidate)));
    }

    let processed = AtomicUsize::new(0);
    let result: Result<Vec<ImageProcessResult>, String> = paths
        .par_iter()
        .zip(targets)
        .map(|(path_str, target_out)| {
            cancel::consume(token)?;

            let n = (processed.fetch_add(1, Ordering::Relaxed) + 1)
                .min(usize::try_from(total).unwrap_or(usize::MAX));
            progress_callback(&TaskProgress {
                total,
                processed: n as u64,
                current_item: Path::new(path_str)
                    .file_name()
                    .map(|nm| nm.to_string_lossy().to_string()),
                percentage: {
                    // ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
                    #[allow(clippy::cast_precision_loss)]
                    {
                        if total > 0 { n as f64 / total as f64 * 100.0 } else { 100.0 }
                    }
                },
            });

            let push = |target_path_str: String, outcome: Result<(), String>| ImageProcessResult {
                source_path: path_str.clone(),
                target_path: target_path_str,
                success: outcome.is_ok(),
                error: outcome.err(),
            };
            match target_out {
                Ok(out) => {
                    let target_path_str = out.to_string_lossy().to_string();
                    let outcome = process(Path::new(path_str), &out);
                    if outcome.is_ok() {
                        log::debug!("处理成功: {path_str} -> {target_path_str}");
                    } else if let Err(err) = &outcome {
                        log::error!("处理失败 {path_str}: {err}");
                    }
                    Ok(push(target_path_str, outcome))
                }
                Err(e) => {
                    log::error!("处理失败 {path_str}: {e}");
                    Ok(push(String::new(), Err(e)))
                }
            }
        })
        // rayon 的 Result 收集遇取消错误短路返回，剩余项不再产出
        .collect::<Result<Vec<_>, String>>();

    let results = result?;
    let success_count = results.iter().filter(|r| r.success).count();
    let fail_count = results.len() - success_count;

    log::info!("图像处理完成: 成功 {success_count}, 失败 {fail_count} -> {target_dir}");

    progress_callback(&TaskProgress {
        total,
        processed: total,
        current_item: None,
        percentage: 100.0,
    });

    Ok(results)
}

pub fn fix_date_taken_batch(
    token: &cancel::CancelToken,
    paths: &[String],
    target_dir: &str,
    date_source: &str,
    specified_time: Option<&str>,
    progress_callback: impl Fn(&TaskProgress) + Sync,
) -> Result<Vec<ImageProcessResult>, String> {
    log::info!("开始修复拍摄时间: {} 文件 -> {}, 来源: {}", paths.len(), target_dir, date_source);
    process_image_batch(
        token,
        paths,
        target_dir,
        |source, out| fix_date_one(source, out, date_source, specified_time),
        progress_callback,
    )
}

/// 脱敏选项：各隐私项是否清除
// ponytail: 5 个 bool 是前端传入的独立功能开关
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy, serde::Deserialize)]
pub struct StripOptions {
    pub exif: bool,
    pub gps: bool,
    pub camera: bool,
    pub xmp: bool,
    pub photoshop: bool,
}

pub fn strip_metadata_batch(
    token: &cancel::CancelToken,
    paths: &[String],
    target_dir: &str,
    options: StripOptions,
    progress_callback: impl Fn(&TaskProgress) + Sync,
) -> Result<Vec<ImageProcessResult>, String> {
    log::info!("开始元数据脱敏: {} 文件 -> {}, EXIF:{}, GPS:{}, 相机:{}, XMP:{}, PS:{}", paths.len(), target_dir, options.exif, options.gps, options.camera, options.xmp, options.photoshop);
    process_image_batch(
        token,
        paths,
        target_dir,
        |source, out| strip_metadata_one(source, out, options),
        progress_callback,
    )
}

pub fn write_gps_batch(
    token: &cancel::CancelToken,
    paths: &[String],
    target_dir: &str,
    latitude: f64,
    longitude: f64,
    progress_callback: impl Fn(&TaskProgress) + Sync,
) -> Result<Vec<ImageProcessResult>, String> {
    log::info!("开始写入GPS: {} 文件 -> {}, 坐标: {}, {}", paths.len(), target_dir, latitude, longitude);
    process_image_batch(
        token,
        paths,
        target_dir,
        |source, out| write_gps_one(source, out, latitude, longitude),
        progress_callback,
    )
}

/// 前端日期选择器保证格式为 "YYYY:MM:DD HH:mm:ss"，仍做严格解析防止直接调用传入非法时间
fn parse_specified_time(time: Option<&str>) -> Result<String, String> {
    let time = time.ok_or("指定时间无效")?;
    chrono::NaiveDateTime::parse_from_str(time, "%Y:%m:%d %H:%M:%S")
        .map_err(|_| "指定时间无效")?;
    Ok(time.to_string())
}

fn fix_date_one(source: &Path, out: &Path, date_source: &str, specified_time: Option<&str>) -> Result<(), String> {
    ensure_jpg(source)?;
    let name = source
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let dt = match date_source {
        "specified" => parse_specified_time(specified_time)?,
        // 用户明确选择“从文件名识别”时不要静默回退到文件时间，
        // 否则看不出日期到底来自文件名还是修改时间。识别失败按失败项报告。
        "filename" => parse_date_from_filename(&name).ok_or("无法从文件名识别日期")?,
        "modified" => file_modified_time(source).ok_or("获取文件修改时间失败")?,
        other => return Err(format!("未知日期来源: {other}")),
    };

    let data = std::fs::read(source).map_err(|e| format!("读取文件失败: {e}"))?;
    let new_data = rebuild_jpeg_exif(&data, |fields| {
        fields.retain(|f| f.tag != Tag::DateTimeOriginal && f.tag != Tag::DateTimeDigitized);
        push_field(fields, Tag::DateTimeOriginal, Value::Ascii(vec![dt.clone().into_bytes()]));
        push_field(fields, Tag::DateTimeDigitized, Value::Ascii(vec![dt.clone().into_bytes()]));
    })?;
    atomic_write(out, &new_data)
}

fn strip_metadata_one(source: &Path, out: &Path, options: StripOptions) -> Result<(), String> {
    ensure_jpg(source)?;
    let data = std::fs::read(source).map_err(|e| format!("读取文件失败: {e}"))?;

    // 方向信息不是隐私数据：直接删除整段 EXIF 会让竖拍照片在查看器里横躺，
    // 因此清除 EXIF 时只保留 Orientation 一个字段重建。
    let orientation = if options.exif {
        load_exif_fields(&data)
            .0
            .into_iter()
            .find(|f| f.tag == Tag::Orientation)
    } else {
        None
    };

    let stripped = strip_jpeg_metadata(&data, options)?;
    let stripped = if options.exif {
        if let Some(field) = orientation {
            rebuild_jpeg_exif(&stripped, |fields| {
                fields.clear();
                fields.push(field);
            })?
        } else {
            stripped
        }
    } else if options.gps || options.camera {
        // EXIF 段未整体清除时，按选项选择性删除 GPS / 相机品牌型号字段
        rebuild_jpeg_exif(&stripped, |fields| {
            if options.gps {
                fields.retain(|f| f.tag.context() != exif::Context::Gps);
            }
            if options.camera {
                fields.retain(|f| f.tag != Tag::Make && f.tag != Tag::Model);
            }
        })?
    } else {
        stripped
    };
    atomic_write(out, &stripped)
}

fn write_gps_one(source: &Path, out: &Path, latitude: f64, longitude: f64) -> Result<(), String> {
    ensure_jpg(source)?;
    if !(-90.0..=90.0).contains(&latitude) {
        return Err("纬度超出范围（-90 到 90）".to_string());
    }
    if !(-180.0..=180.0).contains(&longitude) {
        return Err("经度超出范围（-180 到 180）".to_string());
    }

    let data = std::fs::read(source).map_err(|e| format!("读取文件失败: {e}"))?;
    let new_data = rebuild_jpeg_exif(&data, |fields| {
        fields.retain(|f| f.tag.context() != exif::Context::Gps);
        push_field(fields, Tag::GPSVersionID, Value::Byte(vec![2, 2, 0, 0]));
        push_field(fields, Tag::GPSLatitudeRef, ascii_value(if latitude >= 0.0 { "N" } else { "S" }));
        push_field(fields, Tag::GPSLatitude, Value::Rational(coord_to_dms(latitude.abs())));
        push_field(fields, Tag::GPSLongitudeRef, ascii_value(if longitude >= 0.0 { "E" } else { "W" }));
        push_field(fields, Tag::GPSLongitude, Value::Rational(coord_to_dms(longitude.abs())));
    })?;
    atomic_write(out, &new_data)
}

fn ensure_jpg(path: &Path) -> Result<(), String> {
    match path.extension().map(|e| e.to_string_lossy().to_lowercase()) {
        Some(ext) if ext == "jpg" || ext == "jpeg" => Ok(()),
        _ => Err("仅支持 JPG 格式".to_string()),
    }
}

fn file_modified_time(path: &Path) -> Option<String> {
    let mtime = std::fs::metadata(path).ok()?.modified().ok()?;
    let dt = chrono::DateTime::<chrono::Local>::from(mtime);
    Some(dt.format("%Y:%m:%d %H:%M:%S").to_string())
}

fn push_field(fields: &mut Vec<exif::Field>, tag: Tag, value: Value) {
    fields.push(exif::Field {
        tag,
        ifd_num: In::PRIMARY,
        value,
    });
}

fn ascii_value(s: &str) -> Value {
    Value::Ascii(vec![s.as_bytes().to_vec()])
}

/// 十进制度转度/分/秒 Rational（保留 4 位小数秒）。
// ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn coord_to_dms(v: f64) -> Vec<Rational> {
    // 以 1/10000 秒为单位整体取整后再拆度分秒，避免秒单独四舍五入得到 60.0000
    let total = (v * 3600.0 * 10000.0).round() as u64;
    let deg = total / 36_000_000;
    let rem = total % 36_000_000;
    let min = rem / 600_000;
    let sec = rem % 600_000;
    vec![
        Rational { num: deg as u32, denom: 1 },
        Rational { num: min as u32, denom: 1 },
        Rational { num: sec as u32, denom: 10000 },
    ]
}

fn load_exif_fields(data: &[u8]) -> (Vec<exif::Field>, bool) {
    let mut cursor = std::io::Cursor::new(data);
    exif::Reader::new()
        .read_from_container(&mut cursor)
        .map_or_else(|_| (Vec::new(), false), |exif| (exif.fields().cloned().collect(), exif.little_endian()))
}

/// 检查 JPEG 中是否已经存在 EXIF APP1 段。
fn has_exif_app1(data: &[u8]) -> bool {
    let mut i = 2;
    while i + 4 <= data.len() {
        if data[i] != 0xFF {
            return false;
        }
        let mut j = i + 1;
        while j < data.len() && data[j] == 0xFF {
            j += 1;
        }
        let marker = data.get(j).copied().unwrap_or(0);
        if marker == 0xDA || marker == 0xD9 || j + 3 > data.len() {
            return false;
        }
        let seg_len = u16::from_be_bytes([data[j + 1], data[j + 2]]) as usize;
        let seg_end = j + 1 + seg_len;
        if seg_end > data.len() {
            return false;
        }
        if marker == 0xE1
            && data.get(j + 3..j + 9).is_some_and(|p| p == b"Exif\0\0")
        {
            return true;
        }
        i = seg_end;
    }
    false
}

/// 重建 JPEG 的 EXIF APP1 段：保留原有字段，由 `modify` 闭包修改后重新序列化
fn rebuild_jpeg_exif(data: &[u8], modify: impl FnOnce(&mut Vec<exif::Field>)) -> Result<Vec<u8>, String> {
    if data.len() < 4 || data[0] != 0xFF || data[1] != 0xD8 {
        return Err("不是有效的JPEG文件".to_string());
    }

    let (mut fields, little_endian) = load_exif_fields(data);
    // 原有 EXIF 解析失败时不能拿空字段重建，否则会静默清空元数据
    if fields.is_empty() && has_exif_app1(data) {
        return Err("EXIF数据解析失败，已跳过以保护原有元数据".to_string());
    }
    modify(&mut fields);

    let mut writer = Writer::new();
    for field in &fields {
        writer.push_field(field);
    }
    let mut cursor = std::io::Cursor::new(Vec::new());
    writer
        .write(&mut cursor, little_endian)
        .map_err(|e| format!("生成EXIF数据失败: {e}"))?;
    let tiff = cursor.into_inner();

    let mut payload = Vec::with_capacity(6 + tiff.len());
    payload.extend_from_slice(b"Exif\0\0");
    payload.extend_from_slice(&tiff);
    if payload.len() + 2 > 65535 {
        return Err("EXIF数据过大，无法写入".to_string());
    }

    let mut out = Vec::with_capacity(data.len() + payload.len() + 4);
    out.extend_from_slice(&data[0..2]); // SOI
    let mut inserted = false;
    let mut i = 2;

    while i < data.len() {
        if data[i] != 0xFF {
            // 非段标记，原样保留剩余部分
            out.extend_from_slice(&data[i..]);
            break;
        }
        let mut j = i + 1;
        while j < data.len() && data[j] == 0xFF {
            j += 1;
        }
        let marker = data.get(j).copied().unwrap_or(0);
        if marker == 0xD8 || marker == 0x00 || marker == 0xFF {
            out.push(0xFF);
            i += 1;
            continue;
        }
        if marker == 0xDA || marker == 0xD9 {
            // 扫描数据段或文件结束：插完 APP1 后剩余内容原样复制
            if !inserted {
                out.extend_from_slice(&[0xFF, 0xE1]);
                // ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
                #[allow(clippy::cast_possible_truncation)]
                {
                    out.extend_from_slice(&((payload.len() + 2) as u16).to_be_bytes());
                }
                out.extend_from_slice(&payload);
            }
            out.extend_from_slice(&data[i..]);
            break;
        }
        if j + 3 > data.len() {
            out.extend_from_slice(&data[i..]);
            break;
        }
        let seg_len = u16::from_be_bytes([data[j + 1], data[j + 2]]) as usize;
        let seg_end = j + 1 + seg_len;
        if seg_end > data.len() {
            return Err("JPEG段结构异常".to_string());
        }
        // 在第一个非 APP0（JFIF）段之前插入新的 EXIF APP1
        if !inserted && marker != 0xE0 {
            out.extend_from_slice(&[0xFF, 0xE1]);
            // ponytail: 有意截断，值域由调用方保证；如需严格检查改用 try_from
            #[allow(clippy::cast_possible_truncation)]
            {
                out.extend_from_slice(&((payload.len() + 2) as u16).to_be_bytes());
            }
            out.extend_from_slice(&payload);
            inserted = true;
        }
        // 跳过原 EXIF APP1（已重建），其余原样复制
        let payload_start = j + 3;
        let is_exif_app1 = marker == 0xE1
            && data.get(payload_start..payload_start + 6).is_some_and(|p| p == b"Exif\0\0");
        if !is_exif_app1 {
            out.extend_from_slice(&data[i..seg_end]);
        }
        i = seg_end;
    }

    Ok(out)
}

/// 移除 JPEG 中的隐私元数据段：EXIF（APP1）、XMP（APP1）、Photoshop/IPTC（APP13），按选项启用。
fn strip_jpeg_metadata(data: &[u8], options: StripOptions) -> Result<Vec<u8>, String> {
    if data.len() < 4 || data[0] != 0xFF || data[1] != 0xD8 {
        return Err("不是有效的JPEG文件".to_string());
    }

    let mut out = Vec::with_capacity(data.len());
    out.extend_from_slice(&data[0..2]);
    let mut i = 2;

    while i < data.len() {
        if data[i] != 0xFF {
            out.extend_from_slice(&data[i..]);
            break;
        }
        let mut j = i + 1;
        while j < data.len() && data[j] == 0xFF {
            j += 1;
        }
        let marker = data.get(j).copied().unwrap_or(0);
        if marker == 0xD8 || marker == 0x00 || marker == 0xFF {
            out.push(0xFF);
            i += 1;
            continue;
        }
        if marker == 0xDA || marker == 0xD9 {
            out.extend_from_slice(&data[i..]);
            break;
        }
        if j + 3 > data.len() {
            out.extend_from_slice(&data[i..]);
            break;
        }
        let seg_len = u16::from_be_bytes([data[j + 1], data[j + 2]]) as usize;
        let seg_end = j + 1 + seg_len;
        if seg_end > data.len() {
            return Err("JPEG段结构异常".to_string());
        }
        let payload = data.get(j + 3..seg_end).unwrap_or(&[]);
        let is_exif_seg = marker == 0xE1 && payload.starts_with(b"Exif\0\0");
        let is_xmp_seg = marker == 0xE1 && payload.starts_with(b"http://ns.adobe.com/xap/1.0/\0");
        let is_ps_seg = marker == 0xED && payload.starts_with(b"Photoshop 3.0");
        let is_meta = (options.exif && is_exif_seg)
            || (options.xmp && is_xmp_seg)
            || (options.photoshop && is_ps_seg);
        if !is_meta {
            out.extend_from_slice(&data[i..seg_end]);
        }
        i = seg_end;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_jpeg() -> Vec<u8> {
        let seg = |marker: u8, payload: &[u8]| {
            let mut s = vec![0xFF, marker];
            s.extend_from_slice(&((payload.len() + 2) as u16).to_be_bytes());
            s.extend_from_slice(payload);
            s
        };
        let mut data = vec![0xFF, 0xD8];
        data.extend_from_slice(&seg(0xE1, b"Exif\0\0fake"));
        data.extend_from_slice(&seg(0xE1, b"http://ns.adobe.com/xap/1.0/\0xmp"));
        data.extend_from_slice(&seg(0xED, b"Photoshop 3.0\0ps"));
        data.extend_from_slice(&seg(0xDB, b"quant-table"));
        data.extend_from_slice(&[0xFF, 0xD9]);
        data
    }

    fn all_options() -> StripOptions {
        StripOptions { exif: false, gps: false, camera: false, xmp: false, photoshop: false }
    }

    #[test]
    fn specified_time_is_strictly_parsed() {
        assert!(parse_specified_time(Some("2024:02:29 23:59:59")).is_ok());
        for bad in ["2023:02:29 00:00:00", "2024-01-01 00:00:00", "2024:13:01 00:00:00"] {
            assert!(parse_specified_time(Some(bad)).is_err(), "{bad} 应被拒绝");
        }
        assert!(parse_specified_time(None).is_err());
    }

    #[test]
    fn coord_dms_carries_rounding() {
        // 59.99999 秒四舍五入后应进位为下一分，而不是出现 60 秒
        let v = 12.0 + 34.0 / 60.0 + 59.99999 / 3600.0;
        let dms = coord_to_dms(v);
        assert_eq!(dms[0].num, 12);
        assert_eq!(dms[1].num, 35);
        assert_eq!(dms[2].num, 0);
    }

    #[test]
    fn strip_respects_options() {
        let src = fake_jpeg();
        let has = |d: &[u8], p: &[u8]| d.windows(p.len()).any(|w| w == p);
        let opts = |e, x, p| StripOptions { exif: e, gps: false, camera: false, xmp: x, photoshop: p };

        let full = strip_jpeg_metadata(&src, opts(true, true, true)).unwrap();
        assert!(!has(&full, b"Exif\0\0") && !has(&full, b"http://ns.adobe.com") && !has(&full, b"Photoshop 3.0"));
        assert!(has(&full, b"quant-table"), "普通段必须保留");

        let none = strip_jpeg_metadata(&src, opts(false, false, false)).unwrap();
        assert!(has(&none, b"Exif\0\0") && has(&none, b"Photoshop 3.0"));

        let only_ps = strip_jpeg_metadata(&src, opts(false, false, true)).unwrap();
        assert!(has(&only_ps, b"Exif\0\0") && !has(&only_ps, b"Photoshop 3.0"));
        assert_eq!(strip_jpeg_metadata(&src, all_options()).unwrap(), none);
    }

    /// 批内同名文件必须分配不同目标名，绝不允许静默覆盖（曾因此丢过用户照片）
    #[test]
    fn same_named_sources_get_distinct_targets() {
        let token = crate::cancel::activate("test");
        let target = std::env::temp_dir().join("leaf-tidy-collide-test");
        let _ = std::fs::remove_dir_all(&target);
        std::fs::create_dir_all(&target).unwrap();
        let paths = vec![
            r"C:\a\IMG_0001.jpg".to_string(),
            r"C:\b\IMG_0001.jpg".to_string(),
        ];
        let results = process_image_batch(&token, &paths, target.to_str().unwrap(), |_, _| Ok(()), |_| {})
            .unwrap();
        assert!(results[0].success && results[1].success);
        assert_ne!(results[0].target_path, results[1].target_path);
        crate::cancel::deactivate(&token);
        let _ = std::fs::remove_dir_all(&target);
    }

    /// 输出目录与源文件同目录时必须拒绝，避免改写原文件
    #[test]
    fn rejects_same_dir_output() {
        let token = crate::cancel::activate("test");
        let target = std::env::temp_dir().join("leaf-tidy-samedir-test");
        let _ = std::fs::remove_dir_all(&target);
        std::fs::create_dir_all(&target).unwrap();
        let paths = vec![target.join("a.jpg").to_string_lossy().to_string()];
        let results = process_image_batch(&token, &paths, target.to_str().unwrap(), |_, _| Ok(()), |_| {})
            .unwrap();
        assert!(!results[0].success, "同目录输出应标记失败而非覆盖原文件");
        crate::cancel::deactivate(&token);
        let _ = std::fs::remove_dir_all(&target);
    }
}
