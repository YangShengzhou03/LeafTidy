use std::path::Path;

use crate::models::*;

pub fn read_exif(path: &str) -> Result<ExifInfo, String> {
    let file = std::fs::File::open(path).map_err(|e| format!("打开文件失败: {}", e))?;
    let mut bufreader = std::io::BufReader::new(file);

    let exif_reader = exif::Reader::new();
    let exif = exif_reader
        .read_from_container(&mut bufreader)
        .map_err(|e| format!("读取EXIF失败: {}", e))?;

    let get_str = |tag: exif::Tag| -> Option<String> {
        exif.get_field(tag, exif::In::PRIMARY)
            .map(|f| f.value.display_as(f.tag).to_string())
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

/// 逆地理编码 - 使用 geocode 模块实现
pub fn reverse_geocode(lat: f64, lng: f64) -> Result<GpsLocation, String> {
    crate::geocode::reverse_geocode(lat, lng)
}

/// 初始化地理编码数据
pub fn init_geocode_data(data: &str) -> Result<(), String> {
    crate::geocode::init_geocoder(data)
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

pub fn get_file_category(format: &str) -> String {
    match format.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif" | "heic" | "heif" | "svg" => "image".to_string(),
        "mp4" | "avi" | "mov" | "mkv" | "wmv" | "flv" | "webm" | "mts" | "m2ts" => "video".to_string(),
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => "audio".to_string(),
        "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "pdf" | "txt" | "md" => "document".to_string(),
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => "archive".to_string(),
        _ => "other".to_string(),
    }
}
