use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- AI 智能分类 ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifyResult {
    pub source_path: String,
    pub success: bool,
    pub error: Option<String>,
    pub category: Option<String>,
    pub confidence: Option<f32>,
    pub process_time_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifyRule {
    pub confidence_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifyProgress {
    pub total: u64,
    pub processed: u64,
    pub current_item: Option<String>,
    pub percentage: f64,
    pub current_category: Option<String>,
    /// 刚处理完的单张结果（分类中逐张推送给前端实时展示）
    pub result: Option<ClassifyResult>,
}

/// 内置模型定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub size_mb: f64,
    pub input_size: u32,
    pub top1_accuracy: f64,
}

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
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub action: String,
    pub size: u64,
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
    /// 成功处理文件的总字节数（无法统计时为 0）
    #[serde(default)]
    pub total_size: u64,
    pub results: Vec<OperationResult>,
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
    /// 组类型："exact"（MD5/大小精确重复）或 "similar"（感知哈希相似照片）
    #[serde(default = "default_group_kind")]
    pub kind: String,
}

fn default_group_kind() -> String {
    "exact".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DuplicateFile {
    pub path: String,
    pub name: String,
    pub modified: String,
    pub is_original: bool,
    /// 缩略图 data URL（仅相似照片模式生成）
    #[serde(default)]
    pub thumb: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DuplicateScanResult {
    pub total_files: u64,
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub total_duplicates: u64,
    pub wasted_space: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LowQualityImage {
    pub path: String,
    pub name: String,
    pub size: u64,
    /// 拉普拉斯方差（清晰度，值越低越模糊；非模糊检测时为 0）
    pub variance: f64,
    /// 检测原因 (`"blur" / "small" / "screenshot" / "closed_eye"`)
    pub reason: Option<String>,
    /// 缩略图 data URL（用于预览）
    pub thumb: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LowQualityScanResult {
    pub images: Vec<LowQualityImage>,
    pub total_scanned: u64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    pub total: u64,
    pub processed: u64,
    pub current_item: Option<String>,
    pub percentage: f64,
}

// --- 人脸聚类 ---

/// 已保存的人物记录（faces.json）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    /// 用户确认过（改名/合并过）的人物，匹配阈值更严格且优先匹配
    pub confirmed: bool,
    /// 成员特征向量的运行均值（L2 归一化后）
    pub centroid: Vec<f32>,
    pub photo_count: u32,
}

/// 人物分组中的一张照片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacePhoto {
    pub path: String,
    pub name: String,
    /// 人脸裁剪缩略图 data URL
    pub thumb: Option<String>,
    /// 与本组质心的余弦相似度
    pub similarity: f32,
}

/// 人物分组结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonGroup {
    pub id: String,
    pub name: String,
    pub confirmed: bool,
    pub photos: Vec<FacePhoto>,
}

/// 按人物归档时由前端提交的分组（faces.json 不存照片路径，跨会话需前端回传）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveGroup {
    pub name: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageProcessResult {
    pub source_path: String,
    pub target_path: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelResult {
    pub cancelled: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub log_retention_days: u32,
    pub log_dir: String,
}

// --- SMB 网络共享 ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMBConnectionResult {
    pub unc_path: String,
    pub newly_connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMBShareInfo {
    pub unc_path: String,
    pub server: String,
    pub share: String,
    pub accessible: bool,
}

