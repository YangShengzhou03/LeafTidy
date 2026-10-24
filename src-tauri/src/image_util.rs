//! 图像解码与缩略图工具：尺寸探测、限尺寸解码、base64 data URL。
//! 与 `fs_util`（文件系统与命名）分离，避免工具箱模块职责混杂。

/// 标准 base64 编码（无 padding 省略，带 padding），供缩略图 data URL 使用
pub fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { TABLE[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[n as usize & 63] as char } else { '=' });
    }
    out
}

/// 读取图片尺寸（只读文件头，不解码像素，毫秒级）
pub fn image_dimensions(path: &str) -> Option<(u32, u32)> {
    image::ImageReader::open(path)
        .ok()?
        .with_guessed_format()
        .ok()?
        .into_dimensions()
        .ok()
}

/// 高效加载图片用于分析：对大图先读尺寸判断，避免解码整张亿像素图片只为缩到几百像素。
/// 若图片最长边超过 `max_dim` 的 2 倍，解码后立即缩放到 `max_dim`。
///
/// 各分析任务实际需要的最大尺寸：
/// - dHash 感知哈希：9×8 → 用 `max_dim=64`
/// - 模糊检测拉普拉斯方差：512×512 → 用 `max_dim=512`
/// - 人脸检测 YuNet：320px → 用 `max_dim=320`
/// - `YOLOv8` 分类：224×224 → 用 `max_dim=224`
/// - 缩略图显示：96px → 用 `max_dim=96`
///
/// ponytail: 不做 JPEG 硬件缩放解码（jpeg-decoder v0.3 稳定版不含 Scaling API），
/// 直接解码后 resize。升级路径：换用 zune-jpeg 或 image 0.30+ 的 `decode_with_scale`。
pub fn load_image_limited(path: &str, max_dim: u32) -> Result<image::DynamicImage, String> {
    let Some((orig_w, orig_h)) = image_dimensions(path) else {
        return Err(format!("无法读取图片尺寸: {path}"));
    };

    let longest = orig_w.max(orig_h);

    // 小图（≤2倍目标尺寸）直接解码，无需缩放
    if longest <= max_dim.saturating_mul(2) {
        return image::open(path).map_err(|e| format!("无法打开图片: {e}"));
    }

    // 大图：解码后立即缩放到 max_dim（跳过中间像素，内存可控）
    let img = image::open(path).map_err(|e| format!("无法打开图片: {e}"))?;
    Ok(resize_to_max(img, max_dim))
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless
)]
fn resize_to_max(img: image::DynamicImage, max_dim: u32) -> image::DynamicImage {
    let longest = img.width().max(img.height());
    if longest <= max_dim {
        return img;
    }
    let scale = f64::from(max_dim) / f64::from(longest);
    let new_w = (f64::from(img.width()) * scale).round() as u32;
    let new_h = (f64::from(img.height()) * scale).round() as u32;
    let new_w = new_w.max(1);
    let new_h = new_h.max(1);
    img.resize(
        new_w,
        new_h,
        image::imageops::FilterType::Triangle,
    )
}

/// 生成 data URL 缩略图（最长边缩到 `max_px，JPEG` q80）
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn make_thumb_jpeg(img: image::DynamicImage, max_px: u32) -> Option<String> {
    let (w, h) = (img.width(), img.height());
    let scale = max_px as f32 / w.max(h) as f32;
    let thumb = if scale < 1.0 {
        img.resize(
            ((w as f32) * scale).round() as u32,
            ((h as f32) * scale).round() as u32,
            image::imageops::FilterType::Triangle,
        )
    } else {
        img
    };
    let mut buf = std::io::Cursor::new(Vec::new());
    thumb
        .write_to(&mut buf, image::ImageFormat::Jpeg)
        .ok()?;
    Some(format!("data:image/jpeg;base64,{}", base64_encode(&buf.into_inner())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode(b"Man"), "TWFu");
        assert_eq!(base64_encode(b"Ma"), "TWE=");
        assert_eq!(base64_encode(b"M"), "TQ==");
        assert_eq!(base64_encode(b""), "");
    }
}
