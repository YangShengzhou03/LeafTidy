use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub format: String,
    pub modified: String,
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileDetail {
    pub name: String,
    pub path: String,
    pub format: String,
    pub size: u64,
    pub modified: String,
    pub created: String,
    pub is_dir: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExifInfo {
    pub has_exif: bool,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub date_taken: Option<String>,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub gps_altitude: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub orientation: Option<u32>,
    pub iso: Option<u32>,
    pub focal_length: Option<String>,
    pub aperture: Option<String>,
    pub shutter_speed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpsLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub place: Option<String>,  // 具体地点名称（村庄、城镇等）
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileDetailInfo {
    pub basic: FileDetail,
    pub exif: Option<ExifInfo>,
    pub gps: Option<GpsLocation>,
    pub ai_tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub operation_type: String,
    pub source_path: String,
    pub target_path: Option<String>,
    pub status: String,
    pub detail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationResult {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchOperationResult {
    pub total: usize,
    pub success_count: usize,
    pub fail_count: usize,
    pub results: Vec<OperationResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Option<Vec<DirEntry>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryStats {
    pub total_files: u64,
    pub total_dirs: u64,
    pub total_size: u64,
    pub file_types: HashMap<String, u64>,
    pub oldest_file: Option<String>,
    pub newest_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizeRule {
    pub tags: Vec<String>,
    pub time_source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizeResult {
    pub source_path: String,
    pub target_path: String,
    pub success: bool,
    pub error: Option<String>,
    pub metadata: Option<OrganizeMetadata>,
    pub process_time_ms: Option<u64>,  // 处理耗时（毫秒）
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizeMetadata {
    pub modified: Option<String>,
    pub created: Option<String>,
    pub taken: Option<String>,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub gps_province: Option<String>,
    pub gps_city: Option<String>,
    pub gps_district: Option<String>,
    pub gps_place: Option<String>,  // 具体地点名称
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub size: Option<u64>,
    pub format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenameRule {
    pub template_parts: Vec<TemplatePart>,
    pub time_source: String,
    pub start_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplatePart {
    pub part_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenameResult {
    pub source_path: String,
    pub new_name: String,
    pub target_path: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DuplicateGroup {
    pub md5: String,
    pub size: u64,
    pub files: Vec<DuplicateFile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DuplicateFile {
    pub path: String,
    pub name: String,
    pub modified: String,
    pub is_original: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DuplicateScanResult {
    pub total_files: u64,
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub total_duplicates: u64,
    pub wasted_space: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CleanupResult {
    pub cleanup_type: String,
    pub files: Vec<CleanupFile>,
    pub total_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CleanupFile {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizeProgress {
    pub total: u64,
    pub processed: u64,
    pub success_count: u64,
    pub fail_count: u64,
    pub current_file: Option<String>,
    pub percentage: f64,
}

/// 通用任务进度，用于批量重命名、重复文件检测、附属文件清理等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    pub total: u64,
    pub processed: u64,
    pub current_item: Option<String>,
    pub percentage: f64,
}

/// 取消操作命令的请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelResult {
    pub cancelled: bool,
    pub message: String,
}

