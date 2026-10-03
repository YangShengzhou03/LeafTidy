export interface LogEntry {
  id: string
  timestamp: string
  action: string
  size: number
}

export interface LogDaySummary {
  date: string
  count: number
  totalSize: number
}

export interface LayoutState {
  showLeftBar: boolean
  showRightBar: boolean
  leftBarWidth: number
  rightBarWidth: number
  /** @deprecated 旧版比例存储，读取时转换为像素值后删除 */
  leftBarRatio?: number
  /** @deprecated 旧版比例存储，读取时转换为像素值后删除 */
  rightBarRatio?: number
}

export interface WorkDirectory {
  path: string
  name: string
}

export interface DirectoryStats {
  total_files: number
  total_dirs: number
  total_size: number
  file_types: Record<string, number>
  oldest_file?: string
  newest_file?: string
}

export interface OrganizeResult {
  source_path: string
  target_path: string
  success: boolean
  error?: string
  metadata?: OrganizeMetadata
  process_time_ms?: number  // 处理耗时（毫秒）
}

export interface OrganizeMetadata {
  modified?: string
  created?: string
  taken?: string
  gps_latitude?: number
  gps_longitude?: number
  gps_province?: string
  gps_city?: string
  gps_district?: string
  gps_place?: string
  camera_make?: string
  camera_model?: string
  size?: number
  format?: string
}

export interface RenameRule {
  template_parts: TemplatePart[]
  time_source: string
  start_index: number
}

export interface TemplatePart {
  part_type: string
  value: string
}

export interface RenameResult {
  source_path: string
  new_name: string
  target_path: string
  success: boolean
  error?: string
}

export interface DuplicateGroup {
  md5: string
  size: number
  files: DuplicateFile[]
  /** "exact"（MD5/大小精确重复）或 "similar"（相似照片） */
  kind?: string
}

export interface DuplicateFile {
  path: string
  name: string
  modified: string
  is_original: boolean
  /** 缩略图 data URL（仅相似照片模式） */
  thumb?: string
}

export interface DuplicateScanResult {
  total_files: number
  duplicate_groups: DuplicateGroup[]
  total_duplicates: number
  wasted_space: number
}

export interface LowQualityImage {
  path: string
  name: string
  size: number
  /** 拉普拉斯方差（清晰度，值越低越模糊；非模糊检测时为 0） */
  variance: number
  /** 检测原因 "blur" / "small" / "screenshot" / "closed_eye" */
  reason?: string
  /** 缩略图 data URL */
  thumb?: string
}

export interface LowQualityScanResult {
  images: LowQualityImage[]
  total_scanned: number
}

export interface BatchOperationResult {
  total: number
  success_count: number
  fail_count: number
  /** 成功处理文件的总字节数（无法统计时为 0） */
  total_size?: number
  results: OperationResult[]
}

export interface OperationResult {
  success: boolean
  message?: string
  error?: string
}

export interface ImageProcessResult {
  source_path: string
  target_path: string
  success: boolean
  error?: string
}

export type FunctionPanel =
  | 'home'
  | 'file-organize'
  | 'batch-rename'
  | 'duplicate-clean'
  | 'cleanup'
  | 'fix-date'
  | 'exif-clean'
  | 'write-gps'
  | 'ai-classify'
  | 'log-view'
  | 'log-detail'
  | 'settings'
  | 'privacy-policy'
  | 'license-agreement'
  | 'about'

export interface OrganizeProgress {
  total: number
  processed: number
  success_count: number
  fail_count: number
  current_file?: string
  percentage: number
}

export interface TaskProgress {
  total: number
  processed: number
  current_item?: string
  percentage: number
}

export interface CancelResult {
  cancelled: boolean
  message: string
}

// ===== AI 智能分类 =====

export interface ClassifyResult {
  source_path: string
  success: boolean
  error?: string
  category?: string
  confidence?: number
  process_time_ms?: number
}

export interface ClassifyRule {
  confidence_threshold: number
}

export interface ClassifyProgress {
  total: number
  processed: number
  current_item?: string
  percentage: number
  current_category?: string
  /** 刚处理完的单张结果（分类中逐张推送） */
  result?: ClassifyResult | null
}

// ===== 人脸聚类 =====

export interface Person {
  id: string
  name: string
  confirmed: boolean
  centroid: number[]
  photo_count: number
}

export interface FacePhoto {
  path: string
  name: string
  /** 人脸裁剪缩略图 data URL */
  thumb?: string
  /** 与本组质心的余弦相似度 */
  similarity: number
}

export interface PersonGroup {
  id: string
  name: string
  confirmed: boolean
  photos: FacePhoto[]
}


export interface ModelDefinition {
  id: string
  name: string
  description: string
  size_mb: number
  input_size: number
  top1_accuracy: number
}

// ===== SMB 网络共享 =====

export interface SMBConnection {
  /** 服务器地址，如 192.168.1.100 */
  server: string
  /** 共享文件夹名 */
  share: string
  /** 用户名（可选） */
  username?: string
  /** 密码（可选，不持久化存储） */
  password?: string
  /** 是否记住连接（使用系统凭据管理器） */
  persistent?: boolean
}

export interface SMBConnectionResult {
  /** 连接成功后的 UNC 路径，如 \\192.168.1.100\photo */
  unc_path: string
  /** 是否新连接（false 表示已存在） */
  newly_connected: boolean
}

export interface SMBShareInfo {
  /** UNC 路径 */
  unc_path: string
  /** 服务器地址 */
  server: string
  /** 共享名 */
  share: string
  /** 当前是否可访问 */
  accessible: boolean
}
