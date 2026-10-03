use crate::models::GpsLocation;

#[tauri::command]
pub fn is_geocoder_ready() -> bool {
    let ready = crate::geocode::is_geocoder_ready();
    log::debug!("地理编码器状态: {ready}");
    ready
}

/// 根据经纬度离线查询最近地名（省/市/区/地点）
#[tauri::command]
pub fn query_location(latitude: f64, longitude: f64) -> Result<GpsLocation, String> {
    log::info!("逆地理坐标: lat={latitude}, lng={longitude}");
    crate::geocode::reverse_geocode(latitude, longitude)
}
