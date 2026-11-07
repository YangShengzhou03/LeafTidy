use crate::models::GpsLocation;

#[tauri::command]
#[must_use]
pub fn is_geocoder_ready() -> bool {
    let ready = crate::geocode::is_geocoder_ready();
    log::debug!("地理编码器状态: {ready}");
    ready
}

/// 根据经纬度离线查询最近地名（省/市/区/地点）
///
/// # Errors
///
/// - 地理编码器未初始化或查询失败时返回错误字符串
#[tauri::command]
pub fn query_location(latitude: f64, longitude: f64) -> Result<GpsLocation, String> {
    log::info!("逆地理坐标: lat={latitude}, lng={longitude}");
    crate::geocode::reverse_geocode(latitude, longitude)
}
