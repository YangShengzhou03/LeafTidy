use std::path::Path;

use exif::experimental::Writer;
use exif::{In, Rational, Tag, Value};

use crate::models::*;
use crate::cancel;

pub fn read_exif(path: &str) -> Result<ExifInfo, String> {
    let file = std::fs::File::open(path).map_err(|e| format!("打开文件失败: {}", e))?;
    let mut bufreader = std::io::BufReader::new(file);

    let exif_reader = exif::Reader::new();
    let exif = exif_reader
        .read_from_container(&mut bufreader)
        .map_err(|e| format!("读取EXIF失败: {}", e))?;

    let get_str = |tag: exif::Tag| -> Option<String> {
        exif.get_field(tag, exif::In::PRIMARY)
            .map(|f| {
                let s = f.value.display_as(f.tag).to_string();
                s.trim_matches('"').trim().to_string()
            })
    };

    let get_u32 = |tag: exif::Tag| -> Option<u32> {
        exif.get_field(tag, exif::In::PRIMARY)
            .and_then(|f| match f.value {
                exif::Value::Short(ref v) => v.first().map(|&x| x as u32),
                exif::Value::Long(ref v) => v.first().copied(),
                _ => None,
            })
    };

    let gps_lat = parse_gps_coordinate(&exif, exif::Tag::GPSLatitude, exif::Tag::GPSLatitudeRef);
    let gps_lng = parse_gps_coordinate(&exif, exif::Tag::GPSLongitude, exif::Tag::GPSLongitudeRef);
    let gps_alt = parse_gps_altitude(&exif);

    Ok(ExifInfo {
        has_exif: true,
        camera_make: get_str(exif::Tag::Make),
        camera_model: get_str(exif::Tag::Model),
        date_taken: get_str(exif::Tag::DateTimeOriginal).or_else(|| get_str(exif::Tag::DateTime)),
        gps_latitude: gps_lat,
        gps_longitude: gps_lng,
        gps_altitude: gps_alt,
        width: get_u32(exif::Tag::ImageWidth),
        height: get_u32(exif::Tag::ImageLength),
        orientation: get_u32(exif::Tag::Orientation),
        iso: get_u32(exif::Tag::PhotographicSensitivity),
        focal_length: get_str(exif::Tag::FocalLength),
        aperture: get_str(exif::Tag::ApertureValue),
        shutter_speed: get_str(exif::Tag::ShutterSpeedValue),
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
            let degrees = r.num as f64 / r.denom as f64;
            if ratios.len() < 2 {
                degrees
            } else {
                let r2 = ratios[1];
                let minutes = r2.num as f64 / r2.denom as f64;
                if ratios.len() < 3 {
                    degrees + minutes / 60.0
                } else {
                    let r3 = ratios[2];
                    let seconds = r3.num as f64 / r3.denom as f64;
                    degrees + minutes / 60.0 + seconds / 3600.0
                }
            }
        }
        _ => return None,
    };

    match coord_ref.as_deref() {
        Some("S") | Some("W") => Some(-value),
        _ => Some(value),
    }
}

fn parse_gps_altitude(exif: &exif::Exif) -> Option<f64> {
    let alt = exif.get_field(exif::Tag::GPSAltitude, exif::In::PRIMARY)?;
    let alt_ref = exif
        .get_field(exif::Tag::GPSAltitudeRef, exif::In::PRIMARY)
        .and_then(|f| match f.value {
            exif::Value::Byte(ref v) => v.first().copied(),
            _ => None,
        });

    match &alt.value {
        exif::Value::Rational(ref ratios) if !ratios.is_empty() => {
            let r = ratios[0];
            let value = r.num as f64 / r.denom as f64;
            match alt_ref {
                Some(1) => Some(-value),
                _ => Some(value),
            }
        }
        _ => None,
    }
}

pub fn reverse_geocode(lat: f64, lng: f64) -> Result<GpsLocation, String> {
    crate::geocode::reverse_geocode(lat, lng)
}

pub fn is_image_file(path: &str) -> bool {
    let path = Path::new(path);
    match path.extension() {
        Some(ext) => {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(
                ext.as_str(),
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif" | "heic" | "heif"
            )
        }
        None => false,
    }
}

pub fn is_video_file(path: &str) -> bool {
    let path = Path::new(path);
    match path.extension() {
        Some(ext) => {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(
                ext.as_str(),
                "mp4" | "avi" | "mov" | "mkv" | "wmv" | "flv" | "webm" | "mts" | "m2ts"
            )
        }
        None => false,
    }
}

// ===== 图像元数据编辑（修复拍摄时间 / 脱敏 / 写入GPS）=====

/// 逐文件处理并输出到目标目录。ponytail: 当前仅支持 JPG（EXIF 重写依赖 JPEG APP1 段结构），
/// 其他格式报错跳过；后续如需 PNG/HEIC 再引入对应分段读写。
fn process_image_batch<F>(
    paths: &[String],
    target_dir: &str,
    mut process: F,
    progress_callback: impl Fn(&TaskProgress),
) -> Result<Vec<ImageProcessResult>, String>
where
    F: FnMut(&Path, &Path) -> Result<(), String>,
{
    let target = Path::new(target_dir);
    if !target.exists() {
        std::fs::create_dir_all(target).map_err(|e| format!("创建目标目录失败: {}", e))?;
    }

    let total = paths.len() as u64;
    let mut results = Vec::new();

    for (i, path_str) in paths.iter().enumerate() {
        progress_callback(&TaskProgress {
            total,
            processed: i as u64,
            current_item: Path::new(path_str)
                .file_name()
                .map(|n| n.to_string_lossy().to_string()),
            percentage: if total > 0 { i as f64 / total as f64 * 100.0 } else { 100.0 },
        });

        if cancel::is_cancelled() {
            return Err("用户已取消操作".to_string());
        }

        let source = Path::new(path_str);
        let file_name = source
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let out = target.join(&file_name);
        let target_path_str = out.to_string_lossy().to_string();

        match process(source, &out) {
            Ok(()) => results.push(ImageProcessResult {
                source_path: path_str.clone(),
                target_path: target_path_str,
                success: true,
                error: None,
            }),
            Err(e) => results.push(ImageProcessResult {
                source_path: path_str.clone(),
                target_path: target_path_str,
                success: false,
                error: Some(e),
            }),
        }
    }

    progress_callback(&TaskProgress {
        total,
        processed: total,
        current_item: None,
        percentage: 100.0,
    });

    Ok(results)
}

pub fn fix_date_taken_batch(
    paths: &[String],
    target_dir: &str,
    date_source: &str,
    specified_time: Option<&str>,
    progress_callback: impl Fn(&TaskProgress),
) -> Result<Vec<ImageProcessResult>, String> {
    process_image_batch(
        paths,
        target_dir,
        |source, out| fix_date_one(source, out, date_source, specified_time),
        progress_callback,
    )
}

/// 脱敏选项：各隐私项是否清除（EXIF 整段清除时自动涵盖 GPS 与相机信息）
#[derive(Clone, Copy, serde::Deserialize)]
pub struct StripOptions {
    pub exif: bool,
    pub gps: bool,
    pub camera: bool,
    pub xmp: bool,
    pub photoshop: bool,
}

pub fn strip_metadata_batch(
    paths: &[String],
    target_dir: &str,
    options: StripOptions,
    progress_callback: impl Fn(&TaskProgress),
) -> Result<Vec<ImageProcessResult>, String> {
    process_image_batch(
        paths,
        target_dir,
        |source, out| strip_metadata_one(source, out, options),
        progress_callback,
    )
}

pub fn write_gps_batch(
    paths: &[String],
    target_dir: &str,
    latitude: f64,
    longitude: f64,
    progress_callback: impl Fn(&TaskProgress),
) -> Result<Vec<ImageProcessResult>, String> {
    process_image_batch(
        paths,
        target_dir,
        |source, out| write_gps_one(source, out, latitude, longitude),
        progress_callback,
    )
}

fn fix_date_one(source: &Path, out: &Path, date_source: &str, specified_time: Option<&str>) -> Result<(), String> {
    ensure_jpg(source)?;
    let name = source
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let dt = if date_source == "specified" {
        // 前端日期选择器保证格式为 "YYYY:MM:DD HH:mm:ss"
        specified_time
            .filter(|s| s.len() == 19 && s.as_bytes()[4] == b':' && s.as_bytes()[7] == b':')
            .ok_or("指定时间无效")?
            .to_string()
    } else if date_source == "filename" {
        parse_date_from_filename(&name)
            .or_else(|| file_modified_time(source))
            .ok_or("无法从文件名或文件时间获取日期")?
    } else {
        file_modified_time(source).ok_or("获取文件修改时间失败")?
    };

    let data = std::fs::read(source).map_err(|e| format!("读取文件失败: {}", e))?;
    let new_data = rebuild_jpeg_exif(&data, |fields| {
        fields.retain(|f| f.tag != Tag::DateTimeOriginal && f.tag != Tag::DateTimeDigitized);
        push_field(fields, Tag::DateTimeOriginal, Value::Ascii(vec![dt.clone().into_bytes()]));
        push_field(fields, Tag::DateTimeDigitized, Value::Ascii(vec![dt.clone().into_bytes()]));
    })?;
    std::fs::write(out, new_data).map_err(|e| format!("写入文件失败: {}", e))
}

fn strip_metadata_one(source: &Path, out: &Path, options: StripOptions) -> Result<(), String> {
    ensure_jpg(source)?;
    let data = std::fs::read(source).map_err(|e| format!("读取文件失败: {}", e))?;
    let stripped = strip_jpeg_metadata(&data, options)?;
    // EXIF 段未整体清除时，按选项选择性删除 GPS / 相机品牌型号字段
    let stripped = if !options.exif && (options.gps || options.camera) {
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
    std::fs::write(out, stripped).map_err(|e| format!("写入文件失败: {}", e))
}

fn write_gps_one(source: &Path, out: &Path, latitude: f64, longitude: f64) -> Result<(), String> {
    ensure_jpg(source)?;
    if !(-90.0..=90.0).contains(&latitude) {
        return Err("纬度超出范围（-90 到 90）".to_string());
    }
    if !(-180.0..=180.0).contains(&longitude) {
        return Err("经度超出范围（-180 到 180）".to_string());
    }

    let data = std::fs::read(source).map_err(|e| format!("读取文件失败: {}", e))?;
    let new_data = rebuild_jpeg_exif(&data, |fields| {
        fields.retain(|f| f.tag.context() != exif::Context::Gps);
        push_field(fields, Tag::GPSVersionID, Value::Byte(vec![2, 2, 0, 0]));
        push_field(fields, Tag::GPSLatitudeRef, ascii_value(if latitude >= 0.0 { "N" } else { "S" }));
        push_field(fields, Tag::GPSLatitude, Value::Rational(coord_to_dms(latitude.abs())));
        push_field(fields, Tag::GPSLongitudeRef, ascii_value(if longitude >= 0.0 { "E" } else { "W" }));
        push_field(fields, Tag::GPSLongitude, Value::Rational(coord_to_dms(longitude.abs())));
    })?;
    std::fs::write(out, new_data).map_err(|e| format!("写入文件失败: {}", e))
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

/// 从文件名解析拍摄日期，返回 EXIF 时间格式 "YYYY:MM:DD HH:MM:SS"。
/// ponytail: 启发式匹配常见命名（IMG_20240115_143022 / 2024-01-15 14.30.22 等），
/// 覆盖不了的长尾命名由"使用文件修改时间"兜底。
fn parse_date_from_filename(name: &str) -> Option<String> {
    let chars: Vec<char> = name.chars().collect();
    // 连续 8 位日期：20240115
    for i in 0..chars.len().saturating_sub(7) {
        if !chars[i..i + 8].iter().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let s: String = chars[i..i + 8].iter().collect();
        if let Some(dt) = build_datetime(&s[0..4], &s[4..6], &s[6..8], &chars[i + 8..]) {
            return Some(dt);
        }
    }
    // 分隔日期：2024-01-15 / 2024.1.15
    for i in 0..chars.len().saturating_sub(9) {
        if !chars[i..i + 4].iter().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let is_sep = |c: char| c == '-' || c == '.' || c == '_';
        if !is_sep(chars[i + 4]) || !is_sep(chars[i + 7]) {
            continue;
        }
        let (year, rest) = (&chars[i..i + 4], &chars[i + 5..]);
        let month: String = rest.iter().take_while(|c| c.is_ascii_digit()).collect();
        if month.is_empty() || month.len() > 2 || rest.get(month.len()).copied().map(|c| is_sep(c)) != Some(true) {
            continue;
        }
        let day: String = rest[month.len() + 1..].iter().take_while(|c| c.is_ascii_digit()).collect();
        if day.is_empty() || day.len() > 2 {
            continue;
        }
        let y: String = year.iter().collect();
        let after = &chars[std::cmp::min(i + 5 + month.len() + 1 + day.len(), chars.len())..];
        if let Some(dt) = build_datetime(&y, &month, &day, after) {
            return Some(dt);
        }
    }
    None
}

fn build_datetime(year: &str, month: &str, day: &str, after: &[char]) -> Option<String> {
    if !(year.starts_with("19") || year.starts_with("20")) {
        return None;
    }
    let (year, month, day): (u32, u32, u32) = (year.parse().ok()?, month.parse().ok()?, day.parse().ok()?);
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let (hh, mm, ss) = parse_time_after(after).unwrap_or((0, 0, 0));
    Some(format!("{:04}:{:02}:{:02} {:02}:{:02}:{:02}", year, month, day, hh, mm, ss))
}

/// 在日期后几个字符内寻找 6 位时间（hhmmss），允许分隔符。
fn parse_time_after(chars: &[char]) -> Option<(u32, u32, u32)> {
    let mut j = 0;
    while j < chars.len() && j <= 4 {
        if chars[j].is_ascii_digit() {
            let run: String = chars[j..].iter().take_while(|c| c.is_ascii_digit()).collect();
            if run.len() < 6 {
                return None;
            }
            let hh: u32 = run[0..2].parse().ok()?;
            let mm: u32 = run[2..4].parse().ok()?;
            let ss: u32 = run[4..6].parse().ok()?;
            if hh < 24 && mm < 60 && ss < 60 {
                return Some((hh, mm, ss));
            }
            return None;
        }
        j += 1;
    }
    None
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
fn coord_to_dms(v: f64) -> Vec<Rational> {
    let deg = v.trunc();
    let min_full = (v - deg) * 60.0;
    let min = min_full.trunc();
    let sec = ((min_full - min) * 60.0 * 10000.0).round();
    vec![
        Rational { num: deg as u32, denom: 1 },
        Rational { num: min as u32, denom: 1 },
        Rational { num: sec as u32, denom: 10000 },
    ]
}

fn load_exif_fields(data: &[u8]) -> (Vec<exif::Field>, bool) {
    let mut cursor = std::io::Cursor::new(data);
    match exif::Reader::new().read_from_container(&mut cursor) {
        Ok(exif) => (exif.fields().cloned().collect(), exif.little_endian()),
        // 无 EXIF 的图片从空字段开始重建
        Err(_) => (Vec::new(), false),
    }
}

/// 重建 JPEG 的 EXIF APP1 段：保留原有字段，交给 modify 修改后重新序列化。
fn rebuild_jpeg_exif(data: &[u8], modify: impl FnOnce(&mut Vec<exif::Field>)) -> Result<Vec<u8>, String> {
    if data.len() < 4 || data[0] != 0xFF || data[1] != 0xD8 {
        return Err("不是有效的JPEG文件".to_string());
    }

    let (mut fields, little_endian) = load_exif_fields(data);
    modify(&mut fields);

    let mut writer = Writer::new();
    for field in &fields {
        writer.push_field(field);
    }
    let mut cursor = std::io::Cursor::new(Vec::new());
    writer
        .write(&mut cursor, little_endian)
        .map_err(|e| format!("生成EXIF数据失败: {}", e))?;
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
            // 非段数据，原样保留剩余部分
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
                out.extend_from_slice(&((payload.len() + 2) as u16).to_be_bytes());
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
            out.extend_from_slice(&((payload.len() + 2) as u16).to_be_bytes());
            out.extend_from_slice(&payload);
            inserted = true;
        }
        // 跳过原 EXIF APP1（已重建），其余原样复制
        let payload_start = j + 3;
        let is_exif_app1 = marker == 0xE1
            && data.get(payload_start..payload_start + 6).map_or(false, |p| p == b"Exif\0\0");
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
}
